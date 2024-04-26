use nom::{
    bytes::complete::{is_not, tag},
    character::complete::anychar,
    combinator::{eof, map},
    multi::many_till,
    sequence::delimited,
    IResult,
};

mod ast;
pub use ast::Q3Ast;

use crate::Q3Error;

use nom::branch::alt;
use nom::combinator::all_consuming;
use nom::combinator::peek;
use nom::combinator::recognize;

pub fn parse_id(input: &str) -> IResult<&str, Q3Ast> {
    map(
        delimited(tag("#{"), is_not("{} \t"), tag("}")),
        |item: &str| Q3Ast::Id(item.into()),
    )(input)
}

pub fn parse_any(input: &str) -> IResult<&str, Q3Ast> {
    map(
        many_till(
            anychar,
            alt((recognize(peek(parse_id)), recognize(peek(eof)))),
        ),
        |elem| Q3Ast::Other(elem.0.into_iter().collect()),
    )(input)
}

pub fn parse_query(input: &str) -> Result<Vec<Q3Ast>, Q3Error> {
    let (_rest, matched) = all_consuming(many_till(alt((parse_id, parse_any)), eof))(input)
        .map_err(|_err| Q3Error::FailedToParseQuery)?;
    Ok(matched.0)
}

// #[test]
//
// fn test_parse_id() {
//     let input = "#{test1}";
//
//     assert_eq!(parse_id(input), Ok(("", Q3Ast::Id("test1".into()))));
//
//     let input = "#{ test1 }";
//     assert!(parse_id(input).is_err());
//
//     let input = "#{test1";
//     assert!(parse_id(input).is_err());
// }
//
// #[test]
// fn test_parse_any() {
//     let input = "lorem ipsum dolor #{id1} sit";
//
//     assert_eq!(
//         parse_any(input),
//         Ok(("#{id1} sit", Q3Ast::Other("lorem ipsum dolor ".into())))
//     );
//
//     let input = "lorem ipsum dolor sit";
//
//     assert_eq!(
//         parse_any(input),
//         Ok(("", Q3Ast::Other("lorem ipsum dolor sit".into())))
//     );
//
//     let input = "lorem ipsum dolor #{id1 sit";
//
//     assert_eq!(
//         parse_any(input),
//         Ok(("", Q3Ast::Other("lorem ipsum dolor #{id1 sit".into())))
//     );
//
//     let input = "#{lorem ipsum dolor #{id1 sit";
//
//     assert_eq!(
//         parse_any(input),
//         Ok(("", Q3Ast::Other("#{lorem ipsum dolor #{id1 sit".into())))
//     );
// }
//
// #[test]
// fn parse() {
//     let input = "lorem ipsum dolor #{id1} sit";
//
//     assert_eq!(
//         parse_query(input),
//         Ok((
//             "",
//             vec![
//                 Q3Ast::Other("lorem ipsum dolor ".into()),
//                 Q3Ast::Other("id1".into()),
//                 Q3Ast::Other(" sit".into())
//             ]
//         ))
//     );
//
//     let input = "lorem ipsum dolor #{id1 sit";
//
//     assert_eq!(
//         parse_query(input),
//         Ok(("", vec![Q3Ast::Other("lorem ipsum dolor #{id1 sit".into())]))
//     );
//
//     let input = "#{lorem ipsum dolor #{id1 sit";
//
//     assert_eq!(
//         parse_query(input),
//         Ok((
//             "",
//             vec![Q3Ast::Other("#{lorem ipsum dolor #{id1 sit".into()),]
//         ))
//     );
//
//     let input = "#{id1} lorem ipsum #{ dolor #{id2} sit";
//
//     assert_eq!(
//         parse_query(input),
//         Ok((
//             "",
//             vec![
//                 Q3Ast::Id("id1".into()),
//                 Q3Ast::Other(" lorem ipsum #{ dolor ".into()),
//                 Q3Ast::Id("id2".into()),
//                 Q3Ast::Other(" sit".into()),
//             ]
//         ))
//     )
// }
