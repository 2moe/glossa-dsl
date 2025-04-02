use alloc::borrow::ToOwned;

use tap::Pipe;

use crate::{
  MiniStr,
  error::{ResolverError, ResolverResult},
  parsers::context::Context,
  part::{TemplatePart, VariableRef},
  resolver::Resolver,
  selector, template,
};

impl Resolver {
  pub(crate) fn process_template(
    &self,
    template: &template::Template,
    context: &Context<'_>,
  ) -> ResolverResult<MiniStr> {
    use template::Template::*;
    match template {
      Conditional(x) => self.process_tmpl_selector(context, x),
      Parts(parts) => self.process_tmpl_parts(context, parts),
    }
  }

  pub(crate) fn process_tmpl_parts(
    &self,
    context: &Context<'_>,
    parts: &[TemplatePart],
  ) -> Result<MiniStr, ResolverError> {
    parts.iter().try_fold(
      MiniStr::const_new(""), //
      |mut result, part| {
        let push_str = |s| {
          result.push_str(s);
          Ok(result)
        };
        match part {
          TemplatePart::Text(text) => push_str(text),
          TemplatePart::Variable(var) => match var {
            VariableRef::Variable(var_name) => self
              .try_get_template_and_process(var_name, context)?
              .pipe_deref(push_str),
            VariableRef::Parameter(param) => {
              let err = || {
                param
                  .to_owned()
                  .pipe(ResolverError::MissingParameter)
              };
              context
                .get_value(param)
                .ok_or_else(err)?
                .pipe(push_str)
            }
          },
        }
      },
    )
  }

  /// old_name: process_ref_var
  pub(crate) fn try_get_template_and_process(
    &self,
    var_name: &str,
    context: &Context<'_>,
  ) -> Result<MiniStr, ResolverError> {
    let var_template = self.try_get_template(var_name)?;
    self.process_template(var_template, context)
  }

  pub(crate) fn process_tmpl_selector(
    &self,
    context: &Context<'_>,
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

    let param_value = context
      .get_value(&selector.param)
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
