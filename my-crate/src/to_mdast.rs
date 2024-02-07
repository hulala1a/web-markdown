use std::{fmt};
use pulldown_cmark::{CodeBlockKind, Parser, Tag};
use serde::de::value;
use web_sys::Document;
use std::io::{self, Write};
use crate::mdast::*;
use pulldown_cmark::Event;
use crate::{event::{self, Position}};


#[derive(Debug)]
pub struct CompileContext {
    // Static info.
    /// List of events.
    stack: Vec<Node>
}

impl CompileContext{
    pub fn new()->CompileContext{

        CompileContext { stack: vec![Node::Root(Root { children: Vec::new(), position: None }, ElementNode::default())] }
        // self.stack.push((*root).clone());
    }
    pub fn run(&mut self, parser: Parser)->Option<Node>{
        let offset_parser = parser.into_offset_iter();
        for (event,range) in offset_parser {
            match event {
                Event::Start(tag)=>{
                    self.start_tag(tag,Some(Position{ start: range.start, end: range.end }));
                }
                Event::End(tag) => {
                    self.end_tag(tag);
                }
                Event::Text(text) => {
                    let parent = self.stack.pop().unwrap();
                    match parent {
                        Node::Code(mut Code)=>{
                            Code.value = Some(text.to_string());
                            self.stack.push(Node::Code(Code));
                        }
                        _=>{
                            let text_node = Node::Text(Text { value: Some(text.to_string()), position: Some(Position{ start: range.start, end: range.end }) },TextNode::default());
                            self.push_node_to_parent(text_node, parent)
                        }
                    }
                    let end_newline = text.ends_with('\n');
                    println!("{}", end_newline);
                }
                Event::SoftBreak=>{
                    let parent = self.stack.pop().unwrap();
                    let break_node = Node::SoftBreak(SoftBreak { position: Some(Position{ start: range.start, end: range.end }) });
                    self.push_node_to_parent(break_node, parent)
                }
                Event::HardBreak=>{
                    let parent = self.stack.pop().unwrap();
                    let break_node = Node::HardBreak(HardBreak { position: Some(Position{ start: range.start, end: range.end }) });
                    self.push_node_to_parent(break_node, parent)
                }
                Event::Code(value)=>{
                    let parent = self.stack.pop().unwrap();
                    let code_node = Node::InlineCode(InlineCode { position: Some(Position{ start: range.start, end: range.end }), value:Some(value.to_string())});
                    self.push_node_to_parent(code_node, parent)
                }
                Event::Html(value)=>{
                    let parent = self.stack.pop().unwrap();
                    let html_node = Node::Html(Html { position: Some(Position{ start: range.start, end: range.end }), value:Some(value.to_string())});
                    self.push_node_to_parent(html_node, parent)
                }
                _=>{
                    // print!("no");
                }
            }
            // println!("{}", self.stack.len());
        }
        self.stack.pop()
    }
    fn start_tag(&mut self, tag: Tag,positon:Option<Position>) -> io::Result<()> {
        println!("{}", self.stack.len());
        match tag {
            Tag::Heading(level,id ,classes ) => {
                let heading = Node::Heading(Heading { children:Vec::new(), position: positon, depth: level as u8},ElementNode::default());
                self.stack.push(heading);
            },
            Tag::List(index)=>{
                let list = Node::List(List { index: index, children: Vec::new(), position: positon });
                self.stack.push(list);
                // println!("list");
            }
            Tag::Item=>{
                let item = Node::ListItem(ListItem {  children: Vec::new(), position: positon });
                self.stack.push(item);
                // println!("item");
            }
            Tag::CodeBlock(info)=>{
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        let item = Node::Code(Code {   position: positon, lang:Some(lang.to_owned()), value: None });
                        self.stack.push(item);
                    }
                    CodeBlockKind::Indented => {
                        let item = Node::Code(Code {   position: positon, lang:None, value: None });
                        self.stack.push(item);
                    }
                }
                println!("{}", self.stack.len());
                // println!("item");
            }
            Tag::Paragraph => {
                let item = Node::Paragraph(Paragraph {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::Emphasis=>{
                let item = Node::Emphasis(Emphasis {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::Strong=>{
                let item = Node::Strong(Strong {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::Table(alignment)=>{
                let item = Node::Table(Table {  children: Vec::new(), position: positon, alignment:alignment });
                self.stack.push(item);
            }
            Tag::TableHead=>{
                let item = Node::TableHead(TableHead {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::TableRow=>{
                let item = Node::TableRow(TableRow {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::TableCell=>{
                let item = Node::TableCell(TableCell {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::BlockQuote=>{
                let item = Node::BlockQuote(BlockQuote {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::Strikethrough=>{
                let item = Node::Delete(Delete {  children: Vec::new(), position: positon });
                self.stack.push(item);
            }
            Tag::Link(link_type,url ,title )=>{
                let item = Node::Link(Link {  children: Vec::new(), position: positon,link_type:link_type,url:Some(url.to_string()),title:Some(title.to_string()) });
                self.stack.push(item);
            }
            Tag::Image(link_type,url ,title )=>{
                let item = Node::Image(Image {  position: positon,link_type:link_type,url:Some(url.to_string()),title:Some(title.to_string()) });
                self.stack.push(item);
            }
            _ => {
                // println!("ok");
            }
        }
        Ok(())
    }
    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        println!("{}", self.stack.len());
        let node = self.stack.pop().unwrap();
        let mut parent = self.stack.pop().unwrap();
        self.push_node_to_parent(node, parent);
        // match tag {
        //     Tag::Heading(level,id ,classes ) => {
        //         let node = self.stack.pop().unwrap();
        //         let mut parent = self.stack.pop().unwrap();
        //         self.push_node_to_parent(node, parent);
                
        //     },
        //     Tag::List(index)=>{
        //         let node = self.stack.pop().unwrap();
        //         let mut parent = self.stack.pop().unwrap();
        //         println!("1:{}", self.stack.len());
        //         self.push_node_to_parent(node, parent);
        //         println!("listend");
        //     }
        //     Tag::Item=>{
        //         let node = self.stack.pop().unwrap();
        //         let mut parent = self.stack.pop().unwrap();
        //         println!("1:{}", self.stack.len());
        //         self.push_node_to_parent(node, parent);
        //         println!("itemend");
        //     }
        //     // Tag::Paragraph => {
        //     //     if self.end_newline {
        //     //         self.write("<p>")
        //     //     } else {
        //     //         self.write("\n<p>")
        //     //     }
        //     // }
        //     _ => {
        //         println!("ok");
        //     }
        // }
        Ok(())

    }

    fn push_node_to_parent(&mut self, node: Node, mut parent: Node) {
        if let Some(mut children) = parent.children_mut() {
            children.push(node);
            // println!("{}", self.stack.len());
        }
        self.stack.push(parent);
    }
    
}

pub fn create(node:&mut Node,doc:&Document,parent:&mut Node){
    if let Some(mut element) = parent.create_node(doc){
        element = NOde
    }
    
    if let Some(mut children) = node.children_mut() {
        for child in children{
            create(child, doc, node)
        }
    }
}

pub fn render(markdown_input:String){
    let mut compileContext: CompileContext = CompileContext::new();
    let parser = Parser::new(&markdown_input);
    let root = compileContext.run(parser).unwrap();
}



#[cfg(test)]
mod test {
    use pulldown_cmark::Options;
    use web_sys::window;

    use super::*;
    #[test]
    fn t() {
        let window = window().unwrap();
    let doc = window.document().unwrap();
        let markdown_input = r#"# Heading 1

        
## Heading 2

as*a*dasd
as**d**asd
<div>aaa</div>
        
1. List item 1
2. List item 2

```rust
fn main() {
    println!("Hello, World!");
}"#;
        let mut compileContext: CompileContext = CompileContext::new();
        let parser = Parser::new(markdown_input);
        let node = compileContext.run(parser).unwrap();
         print!("{:#?}", node)
    }
    // TODO: move these tests to tests/html.rs?
    #[test]
    fn t2() {
        let markdown_input = r#"# Hello
| abc | def |
| --- | --- |
| bar | baz |"#;
        let mut compileContext: CompileContext = CompileContext::new();
        let parser = Parser::new_ext(markdown_input,Options::ENABLE_TABLES);
        let node = compileContext.run(parser).unwrap();
         print!("{:#?}", node)
    }
    #[test]
    fn t3() {
        let markdown_input = r#"111  
222`aaa`
<div>
aaa
</div>
[OpenAI](https://www.openai.com/)
![OpenAI](https://www.openai.com/)

        "#;
        let mut compileContext: CompileContext = CompileContext::new();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown_input,options);
        let node = compileContext.run(parser).unwrap();
         print!("{:#?}", node)
    }
    // TODO: move these tests to tests/html.rs?

    
    
}