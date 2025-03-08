use alloc::borrow::ToOwned;

use tap::Pipe;

use crate::{
  MiniStr,
  error::{ResolverError, ResolverResult},
  parsers::get_slice_value,
  part::{TemplatePart, VariableRef},
  resolver::TemplateResolver,
  selector, template,
};

impl TemplateResolver {
  pub(crate) fn process_template(
    &self,
    template: &template::Template,
    sorted_context: &[(&str, &str)],
  ) -> ResolverResult<MiniStr> {
    use template::Template::*;
    match template {
      Conditional(x) => self.process_tmpl_selector(sorted_context, x),
      Parts(parts) => self.process_tmpl_parts(sorted_context, parts),
    }
  }

  pub(crate) fn process_tmpl_parts(
    &self,
    context: &[(&str, &str)],
    parts: &[TemplatePart],
  ) -> Result<MiniStr, ResolverError> {
    parts.iter().try_fold(
      MiniStr::const_new(""),
      |mut result, part| -> Result<MiniStr, ResolverError> {
        let push_str = |s| {
          result.push_str(s);
          Ok(result)
        };
        match part {
          TemplatePart::Text(text) => push_str(text),
          TemplatePart::Variable(var) => match var {
            VariableRef::Variable(var_name) => self
              .process_ref_var(context, var_name)?
              .pipe_deref(push_str),
            VariableRef::Parameter(param) => {
              let err = || {
                param
                  .to_owned()
                  .pipe(ResolverError::MissingParameter)
              };
              get_slice_value(context, param)
                .ok_or_else(err)?
                .pipe(push_str)
            }
          },
        }
      },
    )
  }

  pub(crate) fn process_ref_var(
    &self,
    context: &[(&str, &str)],
    var_name: &MiniStr,
  ) -> Result<MiniStr, ResolverError> {
    let var_template = self
      .get_value_by_key(var_name)
      .ok_or_else(|| {
        var_name
          .to_owned()
          .pipe(ResolverError::UndefinedVariable)
      })?;
    self.process_template(var_template, context)
  }

  pub(crate) fn process_tmpl_selector(
    &self,
    context: &[(&str, &str)],
    selector: &selector::Selector,
  ) -> Result<MiniStr, ResolverError> {
    let new_err = |missing_param| {
      use crate::error::ResolverError::*;
      selector
        .param
        .to_owned()
        .pipe(match missing_param {
          true => MissingParameter,
          _ => NoDefaultBranch,
        })
    };

    let param_value = get_slice_value(context, &selector.param) //
      .ok_or_else(|| new_err(true))?;

    for (value, case_template) in &selector.cases {
      if value == param_value {
        return self.process_template(case_template, context);
      }
    }

    match &selector.default {
      Some(default) => self.process_template(default, context),
      _ => new_err(false).pipe(Err),
    }
  }
}
