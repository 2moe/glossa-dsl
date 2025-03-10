use compact_str::CompactString;
use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_until, take_while},
};

fn parse_delimited(input: &str) -> IResult<&str, &str> {
  // Match initial double braces
  let (input, _) = tag("{{").parse(input)?;

  // Count additional opening braces
  let (input, additional) = take_while(|c| c == '{').parse(input)?;
  let n = 2 + additional.len();

  // if n = 2 => }}
  //   n = 3 => }}}
  let closing_pattern = ['}']
    // .repeat(n)
    .iter()
    .cycle()
    .take(n)
    .collect::<CompactString>();

  // Extract content until closing pattern
  let (input, content) = take_until(closing_pattern.as_str()).parse(input)?;

  // Verify and consume closing braces
  let (input, _) = tag(closing_pattern.as_str()).parse(input)?;

  Ok((input, content.trim_ascii()))
}

#[test]
fn test_parse_delimited() {
  assert_eq!(parse_delimited("{{ aa }}"), Ok(("", "aa")));
  assert_eq!(parse_delimited("{{{ {a} }}}"), Ok(("", "{a}")));
  assert_eq!(parse_delimited("{{ {a} }}"), Ok(("", "{a}")));
  assert_eq!(parse_delimited("{{ {  a } }}"), Ok(("", "{  a }")));
  assert_eq!(parse_delimited("{{{{abc}}}}"), Ok(("", "abc")));
  assert_eq!(
    parse_delimited(
      "{{{
  {{a}
    }}}"
    ),
    Ok(("", "{{a}"))
  );
}

// "{{ a }}" => "a"
// "{{{a}}}" => "a"
// "{{a}" => ❌ Error
// "{{{ {{a} }}}" => "{{a}"
