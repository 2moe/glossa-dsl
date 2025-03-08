// fn take_while1<F>(f: F) -> impl Fn(&str) -> IResult<&str, &str>
// where
//   F: Fn(char) -> bool,
// {
//   move |input| {
//     let mut chars = input.chars();
//     match chars.next() {
//       Some(c) if f(c) => {
//         let len = c.len_utf8();
//         let mut end = len;
//         for c in chars {
//           if !f(c) {
//             break;
//           }
//           end += c.len_utf8();
//         }
//         Ok((&input[end..], &input[0..end]))
//       }
//       _ => Err(nom::Err::Error(Error::new(input, ErrorKind::TakeWhile1))),
//     }
//   }
// }
