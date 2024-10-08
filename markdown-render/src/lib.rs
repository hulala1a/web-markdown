mod diff;
mod event;
mod mdast;
mod utils;

use std::{cell::RefCell, rc::Rc};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// use crate::diff::update;
use crate::diff::*;
use crate::event::Position;
use crate::mdast::*;
use pulldown_cmark::Event;
use pulldown_cmark::{CodeBlockKind, Tag};
use pulldown_cmark::{Options, Parser};
use std::io::{self};
use web_sys::window;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;
extern crate console_error_panic_hook;
use std::panic;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    //   #[wasm_bindgen(js_namespace = window)]
    //   pub fn __random() -> u8;
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct CompileContext {
    old_root: Option<Rc<RefCell<Node>>>,
    root: Rc<RefCell<Node>>,
    stack: Vec<Node>,
}

#[wasm_bindgen]
impl CompileContext {
    pub fn new() -> CompileContext {
        let root = Node::Root(
            Root {
                children: Vec::new(),
                position: None,
            },
            ElementNode::default(),
        );
        CompileContext {
            old_root: None,
            root: Rc::new(RefCell::new(root)),
            stack: vec![],
        }
        // self.stack.push((*root).clone());
    }
    fn run(&mut self, markdown_input: &str) {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(markdown_input, options);
        let offset_parser = parser.into_offset_iter();
        if self.root.borrow().children().unwrap().len() != 0 {
            self.old_root = Some(self.root.clone());
            self.root = Rc::new(RefCell::new(Node::Root(
                Root {
                    children: Vec::new(),
                    position: None,
                },
                ElementNode::default(),
            )));
        }
        for (event, range) in offset_parser {
            // log("222");
            match event {
                Event::Start(tag) => {
                    let _ = self.start_tag(
                        tag,
                        Some(Position {
                            start: range.start,
                            end: range.end,
                        }),
                    );
                }
                Event::End(tag) => {
                    let _ = self.end_tag(tag);
                }
                Event::Text(text) => {
                    let parent = self.stack.pop().unwrap();
                    match parent {
                        Node::Code(mut code, _) => {
                            code.value = Some(text.to_string());
                            self.stack.push(Node::Code(code, ElementNode::default()));
                        }
                        _ => {
                            let text_node = Node::Text(
                                Text {
                                    value: Some(text.to_string()),
                                    position: Some(Position {
                                        start: range.start,
                                        end: range.end,
                                    }),
                                },
                                TextNode::default(),
                            );
                            self.push_node_to_parent(text_node, Some(parent))
                        }
                    }
                    let end_newline = text.ends_with('\n');
                    println!("{}", end_newline);
                }
                Event::SoftBreak => {
                    let parent = self.stack.pop();
                    let break_node = Node::SoftBreak(
                        SoftBreak {
                            position: Some(Position {
                                start: range.start,
                                end: range.end,
                            }),
                        },
                        ElementNode::default(),
                    );
                    self.push_node_to_parent(break_node, parent)
                }
                Event::HardBreak => {
                    let parent = self.stack.pop();
                    let break_node = Node::HardBreak(
                        HardBreak {
                            position: Some(Position {
                                start: range.start,
                                end: range.end,
                            }),
                        },
                        ElementNode::default(),
                    );
                    self.push_node_to_parent(break_node, parent)
                }
                Event::Code(value) => {
                    let parent = self.stack.pop();
                    let code_node = Node::InlineCode(
                        InlineCode {
                            position: Some(Position {
                                start: range.start,
                                end: range.end,
                            }),
                            value: Some(value.to_string()),
                        },
                        ElementNode::default(),
                    );
                    self.push_node_to_parent(code_node, parent)
                }
                Event::Html(value) => {
                    let parent = self.stack.pop();
                    let html_node = Node::Html(
                        Html {
                            position: Some(Position {
                                start: range.start,
                                end: range.end,
                            }),
                            value: Some(value.to_string()),
                        },
                        ElementNode::default(),
                    );
                    self.push_node_to_parent(html_node, parent)
                }
                _ => {
                    // print!("no");
                }
            }
            // println!("{}", self.stack.len());
        }
    }
    fn start_tag(&mut self, tag: Tag, positon: Option<Position>) -> io::Result<()> {
        println!("{}", self.stack.len());
        match tag {
            Tag::Heading(level, _id, _classes) => {
                let heading = Node::Heading(
                    Heading {
                        children: Vec::new(),
                        position: positon,
                        depth: level as u8,
                    },
                    ElementNode::default(),
                );
                self.stack.push(heading);
            }
            Tag::List(index) => {
                let list = Node::List(
                    List {
                        index: index,
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(list);
                // println!("list");
            }
            Tag::Item => {
                let item = Node::ListItem(
                    ListItem {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
                // println!("item");
            }
            Tag::CodeBlock(info) => {
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        let item = Node::Code(
                            Code {
                                position: positon,
                                lang: Some(lang.to_owned()),
                                value: None,
                            },
                            ElementNode::default(),
                        );
                        self.stack.push(item);
                    }
                    CodeBlockKind::Indented => {
                        let item = Node::Code(
                            Code {
                                position: positon,
                                lang: None,
                                value: None,
                            },
                            ElementNode::default(),
                        );
                        self.stack.push(item);
                    }
                }
                println!("{}", self.stack.len());
                // println!("item");
            }
            Tag::Paragraph => {
                let item = Node::Paragraph(
                    Paragraph {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::Emphasis => {
                let item = Node::Emphasis(
                    Emphasis {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::Strong => {
                let item = Node::Strong(
                    Strong {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::Table(alignment) => {
                let item = Node::Table(
                    Table {
                        children: Vec::new(),
                        position: positon,
                        alignment: alignment,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::TableHead => {
                let item = Node::TableHead(
                    TableHead {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::TableRow => {
                let item = Node::TableRow(
                    TableRow {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::TableCell => {
                let item = Node::TableCell(
                    TableCell {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::BlockQuote => {
                let item = Node::BlockQuote(
                    BlockQuote {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::Strikethrough => {
                let item = Node::Delete(
                    Delete {
                        children: Vec::new(),
                        position: positon,
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::Link(link_type, url, title) => {
                let item = Node::Link(
                    Link {
                        children: Vec::new(),
                        position: positon,
                        link_type: link_type,
                        url: Some(url.to_string()),
                        title: Some(title.to_string()),
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            Tag::Image(link_type, url, title) => {
                let item = Node::Image(
                    Image {
                        position: positon,
                        link_type: link_type,
                        url: Some(url.to_string()),
                        title: Some(title.to_string()),
                    },
                    ElementNode::default(),
                );
                self.stack.push(item);
            }
            _ => {
                // println!("ok");
            }
        }
        Ok(())
    }
    fn end_tag(&mut self, _tag: Tag) -> io::Result<()> {
        println!("{}", self.stack.len());
        let node = self.stack.pop().unwrap();
        let parent = self.stack.pop();
        self.push_node_to_parent(node, parent);
        Ok(())
    }

    fn push_node_to_parent(&mut self, node: Node, parent: Option<Node>) {
        match parent {
            Some(mut p) => {
                if let Some(children) = p.children_mut() {
                    children.push(Rc::new(RefCell::new(node)));
                    // println!("{}", self.stack.len());
                }
                self.stack.push(p);
            }
            _ => {
                let mut p = self.root.borrow_mut();
                if let Some(children) = p.children_mut() {
                    children.push(Rc::new(RefCell::new(node)));
                }
            }
        }
    }

    fn init(&self) {
        let window = window().unwrap();
        let doc = window.document().unwrap();
        let root_rc = Rc::clone(&self.root);
        {
            let mut cur_root = root_rc.borrow_mut();
            let node = cur_root.create_node(&doc, None).unwrap();
            match node {
                DocNode::Element(element) => {
                    let render_element = doc.get_element_by_id("render").unwrap();
                    let current_child = render_element.first_child();
                    match current_child {
                        Some(node) => {
                            render_element.remove_child(&node).unwrap();
                        }
                        _ => {}
                    }
                    render_element.append_child(&element).unwrap();
                }
                _ => {}
            }
        }
        let cur_root1 = root_rc.borrow();
        if let Some(children) = cur_root1.children() {
            for child in children {
                let mut cur_child = child.borrow_mut();
                cur_child.create(&doc, root_rc.clone());
            }
        }
    }

    pub fn render(&mut self, markdown_input: &str) {
        if markdown_input == "" {
            return;
        }

        self.run(markdown_input);
        let new_root = self.root.clone();
        match &self.old_root {
            None => {
                self.init();
            }
            Some(_root) => {
                {
                    let old = _root.borrow();
                    let mut new = (&new_root).borrow_mut();
                    new.set_node(old.node_mut());
                }

                update(_root.clone(), Rc::clone(&self.root));
            }
        }
        self.old_root = Some(Rc::clone(&self.root));
    }
}
