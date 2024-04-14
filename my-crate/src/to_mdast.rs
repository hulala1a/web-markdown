use std::{cell::RefCell, fmt, rc::Rc};
use pulldown_cmark::{CodeBlockKind, Options, Parser, Tag};
use serde::de::value;
use web_sys::{window, Document};
use std::io::{self, Write};
use crate::mdast::*;
use pulldown_cmark::Event;
use crate::{event::{self, Position}};
use wasm_bindgen::prelude::*;


#[derive(Debug,Clone)]
#[wasm_bindgen]
pub struct CompileContext {
    // Static info.
    /// List of events.
    old_root: Option<Rc<RefCell<Node>>>,
    root: Rc<RefCell<Node>>,
    stack: Vec<Node>
}

#[wasm_bindgen]
impl CompileContext{
    pub fn new(&mut self){
        let root =Node::Root(Root { children: Vec::new(), position: None }, ElementNode::default());
        self.root = Rc::new(RefCell::new(root));
        self.stack= vec![];
        // self.stack.push((*root).clone());
    }
    pub fn run(&mut self, markdown_input:&str){
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown_input,options);
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
                        Node::Code(mut Code,_)=>{
                            Code.value = Some(text.to_string());
                            self.stack.push(Node::Code(Code,ElementNode::default()));
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
                    let break_node = Node::SoftBreak(SoftBreak { position: Some(Position{ start: range.start, end: range.end }) },ElementNode::default());
                    self.push_node_to_parent(break_node, parent)
                }
                Event::HardBreak=>{
                    let parent = self.stack.pop().unwrap();
                    let break_node = Node::HardBreak(HardBreak { position: Some(Position{ start: range.start, end: range.end }) },ElementNode::default());
                    self.push_node_to_parent(break_node, parent)
                }
                Event::Code(value)=>{
                    let parent = self.stack.pop().unwrap();
                    let code_node = Node::InlineCode(InlineCode { position: Some(Position{ start: range.start, end: range.end }), value:Some(value.to_string())},ElementNode::default());
                    self.push_node_to_parent(code_node, parent)
                }
                Event::Html(value)=>{
                    let parent = self.stack.pop().unwrap();
                    let html_node = Node::Html(Html { position: Some(Position{ start: range.start, end: range.end }), value:Some(value.to_string())},ElementNode::default());
                    self.push_node_to_parent(html_node, parent)
                }
                _=>{
                    // print!("no");
                }
            }
            // println!("{}", self.stack.len());
        }
        
    }
    fn start_tag(&mut self, tag: Tag,positon:Option<Position>) -> io::Result<()> {
        println!("{}", self.stack.len());
        match tag {
            Tag::Heading(level,id ,classes ) => {
                let heading = Node::Heading(Heading { children:Vec::new(), position: positon, depth: level as u8},ElementNode::default());
                self.stack.push(heading);
            },
            Tag::List(index)=>{
                let list = Node::List(List { index: index, children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(list);
                // println!("list");
            }
            Tag::Item=>{
                let item = Node::ListItem(ListItem {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
                // println!("item");
            }
            Tag::CodeBlock(info)=>{
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        let item = Node::Code(Code {   position: positon, lang:Some(lang.to_owned()), value: None },ElementNode::default());
                        self.stack.push(item);
                    }
                    CodeBlockKind::Indented => {
                        let item = Node::Code(Code {   position: positon, lang:None, value: None },ElementNode::default());
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
                let item = Node::Emphasis(Emphasis {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::Strong=>{
                let item = Node::Strong(Strong {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::Table(alignment)=>{
                let item = Node::Table(Table {  children: Vec::new(), position: positon, alignment:alignment },ElementNode::default());
                self.stack.push(item);
            }
            Tag::TableHead=>{
                let item = Node::TableHead(TableHead {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::TableRow=>{
                let item = Node::TableRow(TableRow {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::TableCell=>{
                let item = Node::TableCell(TableCell {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::BlockQuote=>{
                let item = Node::BlockQuote(BlockQuote {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::Strikethrough=>{
                let item = Node::Delete(Delete {  children: Vec::new(), position: positon },ElementNode::default());
                self.stack.push(item);
            }
            Tag::Link(link_type,url ,title )=>{
                let item = Node::Link(Link {  children: Vec::new(), position: positon,link_type:link_type,url:Some(url.to_string()),title:Some(title.to_string()) },ElementNode::default());
                self.stack.push(item);
            }
            Tag::Image(link_type,url ,title )=>{
                let item = Node::Image(Image {  position: positon,link_type:link_type,url:Some(url.to_string()),title:Some(title.to_string()) },ElementNode::default());
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
            children.push(Rc::new(RefCell::new(node)));
            // println!("{}", self.stack.len());
        }
        self.stack.push(parent);
    }
    
    pub fn init(& self){
        let window = window().unwrap();
        let doc = window.document().unwrap();
        let root_rc = Rc::clone(&self.root);
        {
            let mut cur_root = root_rc.borrow_mut();
            let node = cur_root.create_node(&doc, None).unwrap();
            match node {
                DocNode::Element(element)=>{
                    let renderElement = doc.get_element_by_id("render").unwrap();
                    let mut current_child = renderElement.first_child();
                    match current_child{
                        Some(node)=>{renderElement.remove_child(&node).unwrap();},
                        _ =>{}
                    }
                    renderElement.append_child(&element).unwrap();
                }
                _=>{}
            }
        }
        let cur_root1 = root_rc.borrow();
        if let Some(children) = cur_root1.children() {
            for child in children {
                log(&children.len().to_string());
                let mut cur_child = child.borrow_mut();
                cur_child.create(&doc, root_rc.clone());
            }
        }
        self.old_root = Some(Rc::clone(&self.root));
    }
    pub fn render(&mut self, markdown_input:&str){
        match self.old_root{
            None=>{
                self.run(markdown_input);
                self.init();
            }
            Some(root)=>{

            }
        }
    }
}



// pub fn render(markdown_input:&str,doc:&Document){
//     let mut compileContext: CompileContext = CompileContext::new();
//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_STRIKETHROUGH);
//     options.insert(Options::ENABLE_TABLES);
//     let parser = Parser::new_ext(&markdown_input,options);
//     let root = compileContext.run(parser).unwrap();
//     let root_rc = Rc::new(RefCell::new(root));
//     {
//         let mut cur_root = root_rc.borrow_mut();
//         let node = cur_root.create_node(doc, None).unwrap();
//         match node {
//             DocNode::Element(element)=>{
//                 let renderElement = doc.get_element_by_id("render").unwrap();
//                 let mut current_child = renderElement.first_child();
//                 match current_child{
//                     Some(node)=>{renderElement.remove_child(&node).unwrap();},
//                     _ =>{}
//                 }
//                 renderElement.append_child(&element).unwrap();
//             }
//             _=>{}
//         }
//     }
//     let cur_root1 = root_rc.borrow();
//     if let Some(children) = cur_root1.children() {
//         for child in children {
//             create(Rc::clone(child), doc, root_rc.clone());
//         }
//     }
// }



// #[cfg(test)]
// mod test {
//     use pulldown_cmark::Options;
//     use web_sys::window;

//     use super::*;
//     #[test]
//     fn t() {
//         let window = window().unwrap();
//     let doc = window.document().unwrap();
//         let markdown_input = r#"# Heading 1

        
// ## Heading 2

// as*a*dasd
// as**d**asd
// <div>aaa</div>
        
// 1. List item 1
// 2. List item 2

// ```"#;
//         let mut compileContext: CompileContext = CompileContext::new();
//         let parser = Parser::new(markdown_input);
//         let node = compileContext.run(parser).unwrap();
//          print!("{:#?}", node)
//     }
//     // TODO: move these tests to tests/html.rs?
//     #[test]
//     fn t2() {
//         let markdown_input = r#"# Hello
// | abc | def |
// | --- | --- |
// | bar | baz |"#;
//         let mut compileContext: CompileContext = CompileContext::new();
//         let parser = Parser::new_ext(markdown_input,Options::ENABLE_TABLES);
//         let node = compileContext.run(parser).unwrap();
//          print!("{:#?}", node)
//     }
//     #[test]
//     fn t3() {
//         let markdown_input = r#"111  
// 222`aaa`
// <div>
// aaa
// </div>

// [OpenAI](https://www.openai.com/)
// ![OpenAI](https://www.openai.com/)

//         "#;
//         let mut compileContext: CompileContext = CompileContext::new();
//         let mut options = Options::empty();
//         options.insert(Options::ENABLE_STRIKETHROUGH);
//         options.insert(Options::ENABLE_TABLES);
//         let parser = Parser::new_ext(markdown_input,options);
//         let node = compileContext.run(parser).unwrap();
//          print!("{:#?}", node)
//     }
//     // TODO: move these tests to tests/html.rs?
//     #[test]
//     fn t4() {
//         let markdown_input = r#"# 123"#;
//         let window = window().unwrap();
//         let doc = window.document().unwrap();
//         // render(markdown_input, &doc)
//     }
    
    
// }