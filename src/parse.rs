extern crate comrak;
use crate::section::Section;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};
pub type Slide = Vec<Section>;
pub type Presentation = Vec<Slide>;
pub fn parse() -> Presentation {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let input = include_str!("../sample/input.md");
    let slides_str = input.split("---").collect::<Vec<&str>>();
    let mut slides: Presentation = Vec::new();

    for slide in slides_str {
        let mut new_slide: Vec<Section> = Vec::new();
        let root = parse_document(&arena, slide, &Options::default());
        for node in root.children() {
            match node.data.borrow().value {
                NodeValue::Paragraph => {
                    let text = parse_paragraph(node);
                    new_slide.push(Section::Text(text));
                }
                // NodeValue::Text(ref text) => new_slide.push(Section::Text(text.to_string())),
                // NodeValue::Code(ref code) => println!("code '{:?}'", code),
                // NodeValue::HtmlBlock(ref html) => println!("html '{:?}'", html),
                // NodeValue::HtmlInline(ref html) => println!("inline html '{}'", html),
                // NodeValue::Link(ref link) => println!("link '{}'", link.url),
                // NodeValue::Image(ref image) => println!("image '{}'", image.url),
                // NodeValue::LineBreak => println!("line break"),
                // NodeValue::SoftBreak => println!("soft break"),
                // NodeValue::Heading(ref heading) => println!("heading '{}'", heading.level),
                // NodeValue::Paragraph => println!("paragraph"),
                // NodeValue::List(ref list) => println!("list {:?}", list),
                // NodeValue::Item(list) => println!("item"),
                // NodeValue::FootnoteDefinition(ref def) => {
                //     println!("footnote definition '{:?}'", def)
                // }
                // NodeValue::Table(ref table) => println!("table {:?}", table),
                // NodeValue::TableRow(header) => println!("table row"),
                // NodeValue::TableCell => println!("table cell"),
                // NodeValue::Emph => println!("emphasis"),
                // NodeValue::Strong => println!("strong"),
                // NodeValue::Strikethrough => println!("strikethrough"),
                // NodeValue::Superscript => println!("superscript"),
                // NodeValue::CodeBlock(ref codeblock) => println!("code block '{}'", codeblock.info),
                // NodeValue::FootnoteReference(ref reference) => {
                //     println!("footnote reference '{:?}'", reference)
                // }
                _ => {}
            };
        }
        slides.push(new_slide);
    }
    slides
}
fn parse_paragraph<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    for child in node.children() {
        match child.data.borrow().value {
            NodeValue::Text(ref text_node) => {
                text.push_str(text_node);
            }
            NodeValue::LineBreak => {
                text.push('\n');
            }
            _ => {}
        }
    }
    text
}
