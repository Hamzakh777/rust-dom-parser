use rust_dom_parser::html_parser::HtmlParser;

#[test]
fn test_html_parser_consume_while() {
    let mut parser = HtmlParser::new("aaabbb");
    let text = parser.consume_while(|c| c == 'a');
    assert_eq!(text, "aaa");
}