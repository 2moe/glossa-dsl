use alloc::vec::Vec;

use tap::{Pipe, Tap};

use super::TemplateResolver;

impl serde::Serialize for TemplateResolver {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    use serde::ser::SerializeMap;

    let entries = self
      .iter()
      .collect::<Vec<_>>()
      // .tap_mut(|x| x.sort_unstable());
      .tap_mut(|x| x.sort_unstable_by_key(|(k, _)| *k));

    let mut map = entries
      .len()
      .pipe(Some)
      .pipe(|len| serializer.serialize_map(len))?;

    for (k, v) in entries {
      map.serialize_entry(k, v)?;
    }
    map.end()
  }
}
