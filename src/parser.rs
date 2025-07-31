use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::{alpha1, alphanumeric1, space0, space1, line_ending, char},
    combinator::*,
    multi::{many0, many1, separated_list1},
    sequence::*,
    IResult,
};

use crate::ast::{Statement, CategoryAST};

/// Parse an identifier (alphanumeric characters)
fn identifier(input: &str) -> IResult<&str, String> {
    map(
        recognize(pair(
            alt((alpha1, char('_'))),
            many0(alt((alphanumeric1, char('_')))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

/// Parse whitespace
fn whitespace(input: &str) -> IResult<&str, &str> {
    recognize(many0(alt((space1, line_ending))))(input)
}

/// Parse object declaration: object A
pub fn parse_object(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("object")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = opt(whitespace)(input)?;
    Ok((input, Statement::Object(name)))
}

/// Parse morphism declaration: morphism f: A -> B
pub fn parse_morphism(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("morphism")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, from) = identifier(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space0(input)?;
    let (input, to) = identifier(input)?;
    let (input, _) = opt(whitespace)(input)?;
    Ok((input, Statement::Morphism { name, from, to }))
}

/// Parse composition operator: ∘
fn parse_composition_op(input: &str) -> IResult<&str, &str> {
    delimited(space0, tag("∘"), space0)(input)
}

/// Parse commutativity assertion: assert commute: g ∘ f == h
pub fn parse_assert_commute(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("assert commute:")(input)?;
    let (input, _) = space0(input)?;
    let (input, lhs) = separated_list1(parse_composition_op, identifier)(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("==")(input)?;
    let (input, _) = space0(input)?;
    let (input, rhs) = separated_list1(parse_composition_op, identifier)(input)?;
    let (input, _) = opt(whitespace)(input)?;
    Ok((input, Statement::AssertCommute { lhs, rhs }))
}

/// Parse a single statement line
pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = opt(whitespace)(input)?;
    let result = alt((parse_object, parse_morphism, parse_assert_commute))(input)?;
    let (input, _) = opt(whitespace)(input)?;
    Ok(result)
}

/// Parse entire category theory file
pub fn parse_category_file(input: &str) -> IResult<&str, CategoryAST> {
    let (input, statements) = separated_list1(
        many1(line_ending),
        parse_statement,
    )(input)?;
    Ok((input, CategoryAST { statements }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        assert_eq!(
            parse_object("object A"),
            Ok(("", Statement::Object("A".to_string())))
        );
    }

    #[test]
    fn test_parse_morphism() {
        assert_eq!(
            parse_morphism("morphism f: A -> B"),
            Ok(("", Statement::Morphism {
                name: "f".to_string(),
                from: "A".to_string(),
                to: "B".to_string(),
            }))
        );
    }

    #[test]
    fn test_parse_assert_commute() {
        assert_eq!(
            parse_assert_commute("assert commute: g ∘ f == h"),
            Ok(("", Statement::AssertCommute {
                lhs: vec!["g".to_string(), "f".to_string()],
                rhs: vec!["h".to_string()],
            }))
        );
    }
} 