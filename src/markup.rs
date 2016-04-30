#[derive(PartialEq, Debug)]
pub enum Node<'a> {
    Text(&'a [u8]),
    Tag(&'a [u8]),
}

named!(pub tag<&[u8], Node>, chain!(
    tag!("{{") ~
    content: take_until_and_consume!("}}") ,
    ||{Node::Tag(content)}
));
named!(pub text<&[u8], Node>, chain!(
    content: take_until!("{{") ,
    ||{Node::Text(content)}
));
named!(pub parse<&[u8], Vec<Node> >, many0!(alt!(tag | text)));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn tag_works() {
        let expected: IResult<&[u8], Node> = IResult::Done(b"", Node::Tag(b"bacon"));
        assert_eq!(tag(b"{{bacon}}"), expected);
    }

    #[test]
    fn parse_works() {
        let expected: IResult<&[u8], Vec<Node>> = IResult::Done(b"",
                                                                vec![
                          Node::Tag(b"bacon"),
                          Node::Text(b"cheese"),
                          Node::Tag(b"tomato"),
                          Node::Tag(b"chair"),
                          ]);
        assert_eq!(parse(b"{{bacon}}cheese{{tomato}}{{chair}}"), expected);
    }
}
