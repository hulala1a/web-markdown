use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

use pulldown_cmark::{Alignment, HeadingLevel, LinkType};

use web_sys::{Document, Element, Text as TextElement};

use crate::event::Position;

#[derive(Clone)]
pub enum Node {
    // Document:
    /// Root.
    Root(Root, ElementNode),

    // Container:
    /// Block quote.
    BlockQuote(BlockQuote, ElementNode),

    /// Footnote definition.

    /// MDX: JSX element (container).

    /// List.
    List(List, ElementNode),

    // Frontmatter:
    /// MDX.js ESM.

    /// Toml.

    /// Yaml.

    // Phrasing:
    /// Break.
    SoftBreak(SoftBreak, ElementNode),
    HardBreak(HardBreak, ElementNode),

    /// Code (phrasing).
    InlineCode(InlineCode, ElementNode),

    /// Math (phrasing).

    /// Delete.
    Delete(Delete, ElementNode),

    /// Emphasis.
    Emphasis(Emphasis, ElementNode),

    // MDX: expression (text).
    /// Footnote reference.

    /// Html (phrasing).
    Html(Html, ElementNode),

    /// Image.
    Image(Image, ElementNode),

    /// Image reference.

    /// Link.
    Link(Link, ElementNode),

    /// Link reference.

    /// Strong
    Strong(Strong, ElementNode),

    /// Text.
    Text(Text, TextNode),

    // Flow:
    /// Code (flow).
    Code(Code, ElementNode),

    /// Math (flow).

    /// Heading.
    Heading(Heading, ElementNode),

    /// Table
    Table(Table, ElementNode),

    // Table content.
    /// Table head.
    TableHead(TableHead, ElementNode),

    /// Table row.
    TableRow(TableRow, ElementNode),

    /// Table cell.
    TableCell(TableCell, ElementNode),

    // List content.
    /// List item.
    ListItem(ListItem, ElementNode),

    /// Paragraph.
    Paragraph(Paragraph, ElementNode),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        // TODO
        match (self, other) {
            (Node::Root(_, _), Node::Root(_, _)) => true,
            (Node::BlockQuote(_, _), Node::BlockQuote(_, _)) => true,
            (Node::List(_, _), Node::List(_, _)) => true,
            (Node::SoftBreak(_, _), Node::SoftBreak(_, _)) => true,
            (Node::HardBreak(_, _), Node::HardBreak(_, _)) => true,
            (Node::InlineCode(_, _), Node::InlineCode(_, _)) => true,
            (Node::Delete(_, _), Node::Delete(_, _)) => true,
            (Node::Emphasis(_, _), Node::Emphasis(_, _)) => true,
            (Node::Html(_, _), Node::Html(_, _)) => true,
            (Node::Image(_, _), Node::Image(_, _)) => true,
            (Node::Link(_, _), Node::Link(_, _)) => true,
            (Node::Strong(_, _), Node::Strong(_, _)) => true,
            (Node::Text(_, _), Node::Text(_, _)) => true,
            (Node::Code(_, _), Node::Code(_, _)) => true,
            (Node::Heading(heading_self, _), Node::Heading(heading_other, _)) => {
                heading_self.depth == heading_other.depth
            }
            (Node::Table(_, _), Node::Table(_, _)) => true,
            (Node::TableHead(_, _), Node::TableHead(_, _)) => true,
            (Node::TableRow(_, _), Node::TableRow(_, _)) => true,
            (Node::TableCell(_, _), Node::TableCell(_, _)) => true,
            (Node::ListItem(_, _), Node::ListItem(_, _)) => true,
            (Node::Paragraph(_, _), Node::Paragraph(_, _)) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Node {
    // Debug the wrapped struct.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Root(x, _) => x.fmt(f),
            Node::BlockQuote(x, _) => x.fmt(f),
            Node::List(x, _) => x.fmt(f),
            Node::SoftBreak(x, _) => x.fmt(f),
            Node::HardBreak(x, _) => x.fmt(f),
            Node::InlineCode(x, _) => x.fmt(f),
            Node::Delete(x, _) => x.fmt(f),
            Node::Emphasis(x, _) => x.fmt(f),
            Node::Html(x, _) => x.fmt(f),
            Node::Image(x, _) => x.fmt(f),
            Node::Link(x, _) => x.fmt(f),
            Node::Strong(x, _) => x.fmt(f),
            Node::Text(x, _) => x.fmt(f),
            Node::Code(x, _) => x.fmt(f),
            Node::Heading(x, _) => x.fmt(f),
            Node::Table(x, _) => x.fmt(f),
            Node::TableHead(x, _) => x.fmt(f),
            Node::TableRow(x, _) => x.fmt(f),
            Node::TableCell(x, _) => x.fmt(f),
            Node::ListItem(x, _) => x.fmt(f),
            Node::Paragraph(x, _) => x.fmt(f),
        }
    }
}

impl Node {
    #[must_use]
    pub fn children(&self) -> Option<&Vec<Rc<RefCell<Node>>>> {
        match self {
            // Parent.
            Node::Root(x, _) => Some(&x.children),
            Node::Paragraph(x, _) => Some(&x.children),
            Node::Heading(x, _) => Some(&x.children),
            Node::BlockQuote(x, _) => Some(&x.children),
            Node::List(x, _) => Some(&x.children),
            Node::ListItem(x, _) => Some(&x.children),
            Node::Emphasis(x, _) => Some(&x.children),
            Node::Strong(x, _) => Some(&x.children),
            Node::Link(x, _) => Some(&x.children),
            Node::Table(x, _) => Some(&x.children),
            Node::TableHead(x, _) => Some(&x.children),
            Node::TableRow(x, _) => Some(&x.children),
            Node::TableCell(x, _) => Some(&x.children),
            Node::Delete(x, _) => Some(&x.children),
            _ => None,
        }
    }
    pub fn value(&self) -> Option<&String> {
        match self {
            // Parent.
            Node::Text(x, _) => x.value.as_ref(),
            Node::Code(x, _) => x.value.as_ref(),
            Node::InlineCode(x, _) => x.value.as_ref(),
            Node::Html(x, _) => x.value.as_ref(),
            // Node::Paragraph(x,_) => Some(&x.children),
            // Node::Heading(x, _) => Some(&x.children),
            // Node::BlockQuote(x, _) => Some(&x.children),
            // Node::List(x, _) => Some(&x.children),
            // Node::ListItem(x, _) => Some(&x.children),
            // Node::Emphasis(x, _) => Some(&x.children),
            // Node::Strong(x, _) => Some(&x.children),
            // Node::Link(x, _) => Some(&x.children),
            // Node::Table(x, _) => Some(&x.children),
            // Node::TableHead(x, _) => Some(&x.children),
            // Node::TableRow(x, _) => Some(&x.children),
            // Node::TableCell(x, _) => Some(&x.children),
            // Node::Delete(x, _) => Some(&x.children),
            // Non-parent.
            _ => None,
        }
    }

    pub fn url(&self) -> Option<&String> {
        match self {
            // Parent.
            // Node::Text(x, _) => x.value.as_ref(),
            // Node::Code(x, _) => x.value.as_ref(),
            // Node::InlineCode(x, _) => x.value.as_ref(),
            // Node::Html(x, _) => x.value.as_ref(),
            // Node::Paragraph(x,_) => Some(&x.children),
            // Node::Heading(x, _) => Some(&x.children),
            // Node::BlockQuote(x, _) => Some(&x.children),
            // Node::List(x, _) => Some(&x.children),
            // Node::ListItem(x, _) => Some(&x.children),
            // Node::Emphasis(x, _) => Some(&x.children),
            // Node::Strong(x, _) => Some(&x.children),
            Node::Link(x, _) => x.url.as_ref(),
            Node::Image(x, _) => x.url.as_ref(),
            // Node::Table(x, _) => Some(&x.children),
            // Node::TableHead(x, _) => Some(&x.children),
            // Node::TableRow(x, _) => Some(&x.children),
            // Node::TableCell(x, _) => Some(&x.children),
            // Node::Delete(x, _) => Some(&x.children),
            // Non-parent.
            _ => None,
        }
    }

    pub fn title(&self) -> Option<&String> {
        match self {
            // Parent.
            // Node::Text(x, _) => x.value.as_ref(),
            // Node::Code(x, _) => x.value.as_ref(),
            // Node::InlineCode(x, _) => x.value.as_ref(),
            // Node::Html(x, _) => x.value.as_ref(),
            // Node::Paragraph(x,_) => Some(&x.children),
            // Node::Heading(x, _) => Some(&x.children),
            // Node::BlockQuote(x, _) => Some(&x.children),
            // Node::List(x, _) => Some(&x.children),
            // Node::ListItem(x, _) => Some(&x.children),
            // Node::Emphasis(x, _) => Some(&x.children),
            // Node::Strong(x, _) => Some(&x.children),
            Node::Link(x, _) => x.title.as_ref(),
            Node::Image(x, _) => x.title.as_ref(),
            // Node::Table(x, _) => Some(&x.children),
            // Node::TableHead(x, _) => Some(&x.children),
            // Node::TableRow(x, _) => Some(&x.children),
            // Node::TableCell(x, _) => Some(&x.children),
            // Node::Delete(x, _) => Some(&x.children),
            // Non-parent.
            _ => None,
        }
    }
    // pub fn parent(&self) -> Option<&Rc<RefCell<Node>>> {
    //     match self {
    //         // Parent.
    //         Node::Root(_, x) => {
    //             let parent_el = x.parent.as_ref();
    //             parent_el
    //         }
    //         Node::Paragraph(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Heading(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::BlockQuote(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::List(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::ListItem(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Emphasis(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Strong(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Link(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         // Node::LinkReference(x) => Some(&x.children),
    //         // Node::FootnoteDefinition(x) => Some(&x.children),
    //         Node::Table(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableHead(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableRow(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableCell(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Delete(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::MdxJsxFlowElement(x) => Some(&x.children),
    //         Node::MdxJsxTextElement(x) => Some(&x.children),
    //         Non-parent.
    //         _ => None,
    //     }
    // }

    // pub fn parent_mut(&mut self) -> Option<&mut Rc<RefCell<Node>>> {
    //     match self {
    //         // Parent.
    //         Node::Root(_, x) => {
    //             let parent_el = x.parent.as_mut();
    //             parent_el
    //         }
    //         Node::Paragraph(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Heading(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::BlockQuote(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::List(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::ListItem(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Emphasis(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Strong(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Link(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         // Node::LinkReference(x) => Some(&x.children),
    //         // Node::FootnoteDefinition(x) => Some(&x.children),
    //         Node::Table(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableHead(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableRow(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableCell(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Delete(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::MdxJsxFlowElement(x) => Some(&x.children),
    //         Node::MdxJsxTextElement(x) => Some(&x.children),
    //         Non-parent.
    //         _ => None,
    //     }
    // }

    // pub fn set_parent(&mut self, parent: Option<&Rc<RefCell<Node>>>) {
    //     match self {
    //         // Parent.
    //         Node::Root(_, x) => {
    //             x.parent = parent.cloned();
    //         }
    //         Node::Paragraph(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Heading(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::BlockQuote(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::List(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::ListItem(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Emphasis(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Strong(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Link(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         // Node::LinkReference(x) => Some(&x.children),
    //         // Node::FootnoteDefinition(x) => Some(&x.children),
    //         Node::Table(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableHead(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableRow(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::TableCell(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::Delete(_,x) => x.parent.unwrap().borrow().node_mut(),
    //         Node::MdxJsxFlowElement(x) => Some(&x.children),
    //         Node::MdxJsxTextElement(x) => Some(&x.children),
    //         Non-parent.
    //         _ => {}
    //     }
    // }

    pub fn children_mut(&mut self) -> Option<&mut Vec<Rc<RefCell<Node>>>> {
        match self {
            // Parent.
            Node::Root(x, _) => Some(&mut x.children),
            Node::Paragraph(x, _) => Some(&mut x.children),
            Node::Heading(x, _) => Some(&mut x.children),
            Node::BlockQuote(x, _) => Some(&mut x.children),
            Node::List(x, _) => Some(&mut x.children),
            Node::ListItem(x, _) => Some(&mut x.children),
            Node::Emphasis(x, _) => Some(&mut x.children),
            Node::Strong(x, _) => Some(&mut x.children),
            Node::Link(x, _) => Some(&mut x.children),
            Node::Table(x, _) => Some(&mut x.children),
            Node::TableHead(x, _) => Some(&mut x.children),
            Node::TableRow(x, _) => Some(&mut x.children),
            Node::TableCell(x, _) => Some(&mut x.children),
            Node::Delete(x, _) => Some(&mut x.children),
            // Non-parent.
            _ => None,
        }
    }

    pub fn create(&mut self, doc: &Document, parent_rc: Rc<RefCell<Node>>) {
        let parent = parent_rc.borrow();

        if let Some(parent_element) = parent.node_mut() {
            match parent_element {
                DocNode::Element(parent_node) => {
                    let child_element = self.create_node(doc, Some(Rc::clone(&parent_rc))).unwrap();
                    match child_element {
                        DocNode::Element(child_node) => {
                            parent_node.append_child(child_node).unwrap();
                        }
                        DocNode::Text(child_node) => {
                            parent_node.append_child(child_node).unwrap();
                        }
                    }
                }
                DocNode::Text(_) => {}
            }
        }
        if let Some(children) = self.children() {
            for child in children {
                let mut cur_child = child.borrow_mut();
                cur_child.create(doc, Rc::new(RefCell::new(self.clone())));
            }
        }
    }

    pub fn node_mut(&self) -> Option<&DocNode> {
        match self {
            // Parent.
            Node::Root(_, node) => node.cur.as_ref(),
            Node::Paragraph(_, node) => node.cur.as_ref(),
            Node::Heading(_, node) => node.cur.as_ref(),
            Node::Text(_, node) => node.cur.as_ref(),
            Node::BlockQuote(_, node) => node.cur.as_ref(),
            Node::List(_, node) => node.cur.as_ref(),
            Node::SoftBreak(_, node) => node.cur.as_ref(),
            Node::HardBreak(_, node) => node.cur.as_ref(),
            Node::InlineCode(_, node) => node.cur.as_ref(),
            Node::Delete(_, node) => node.cur.as_ref(),
            Node::Emphasis(_, node) => node.cur.as_ref(),
            Node::Html(_, node) => node.cur.as_ref(),
            Node::Image(_, node) => node.cur.as_ref(),
            Node::Link(_, node) => node.cur.as_ref(),
            Node::Strong(_, node) => node.cur.as_ref(),
            Node::Code(_, node) => node.cur.as_ref(),
            Node::Table(_, node) => node.cur.as_ref(),
            Node::TableHead(_, node) => node.cur.as_ref(),
            Node::TableRow(_, node) => node.cur.as_ref(),
            Node::TableCell(_, node) => node.cur.as_ref(),
            Node::ListItem(_, node) => node.cur.as_ref(),
            // Non-parent.
        }
    }

    pub fn set_node(&mut self, el: Option<&DocNode>) {
        match self {
            // Parent.
            Node::Root(_, node) => node.cur = el.cloned(),
            Node::Paragraph(_, node) => node.cur = el.cloned(),
            Node::Heading(_, node) => node.cur = el.cloned(),
            Node::Text(_, node) => node.cur = el.cloned(),
            Node::BlockQuote(_, node) => node.cur = el.cloned(),
            Node::List(_, node) => node.cur = el.cloned(),
            Node::SoftBreak(_, node) => node.cur = el.cloned(),
            Node::HardBreak(_, node) => node.cur = el.cloned(),
            Node::InlineCode(_, node) => node.cur = el.cloned(),
            Node::Delete(_, node) => node.cur = el.cloned(),
            Node::Emphasis(_, node) => node.cur = el.cloned(),
            Node::Html(_, node) => node.cur = el.cloned(),
            Node::Image(_, node) => node.cur = el.cloned(),
            Node::Link(_, node) => node.cur = el.cloned(),
            Node::Strong(_, node) => node.cur = el.cloned(),
            Node::Code(_, node) => node.cur = el.cloned(),
            Node::Table(_, node) => node.cur = el.cloned(),
            Node::TableHead(_, node) => node.cur = el.cloned(),
            Node::TableRow(_, node) => node.cur = el.cloned(),
            Node::TableCell(_, node) => node.cur = el.cloned(),
            Node::ListItem(_, node) => node.cur = el.cloned(),
            // Non-parent.
        }
    }
    // pub fn value_set(&mut self,value:String){
    //     match self {
    //         // Parent.
    //         Node::Code(x) => x.value = Some(value),
    // Node::Paragraph(x) => Some(&mut x.children),
    // Node::Heading(x) => Some(&mut x.children),
    // Node::BlockQuote(x) => Some(&mut x.children),
    // Node::List(x) => Some(&mut x.children),
    // Node::ListItem(x) => Some(&mut x.children),
    // Node::Emphasis(x) => Some(&mut x.children),
    // Node::Strong(x) => Some(&mut x.children),
    // Node::Link(x) => Some(&mut x.children),
    // Node::LinkReference(x) => Some(&mut x.children),
    // Node::FootnoteDefinition(x) => Some(&mut x.children),
    // Node::Table(x) => Some(&mut x.children),
    // Node::TableRow(x) => Some(&mut x.children),
    // Node::TableCell(x) => Some(&mut x.children),
    // Node::Delete(x) => Some(&mut x.children),
    // Node::MdxJsxFlowElement(x) => Some(&mut x.children),
    // Node::MdxJsxTextElement(x) => Some(&mut x.children),
    // Non-parent.
    // _=>{}
    // }
    // }
    pub fn create_node(
        &mut self,
        document: &Document,
        parent: Option<Rc<RefCell<Node>>>,
    ) -> Option<&DocNode> {
        match self {
            Node::Root(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("div").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::BlockQuote(x, node) => {
                node.cur = Some(DocNode::Element(
                    document.create_element("blockquote").unwrap(),
                ));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::List(x, node) => {
                node.cur = match x.index {
                    None => Some(DocNode::Element(document.create_element("ul").unwrap())),
                    _ => Some(DocNode::Element(document.create_element("ol").unwrap())),
                };
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::SoftBreak(x, node) => {
                node.cur = Some(DocNode::Text(document.create_text_node(" ")));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::HardBreak(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("br").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::InlineCode(x, node) => {
                let code = document.create_element("code").unwrap();
                let content = document.create_text_node(&x.value.clone().unwrap());
                code.append_child(&content).unwrap();
                node.cur = Some(DocNode::Element(code));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Delete(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("del").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Emphasis(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("em").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Html(x, node) => {
                let div = document.create_element("div").unwrap();
                div.set_inner_html(&x.value.clone()?);
                node.cur = Some(DocNode::Element(div));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Image(x, node) => {
                let link = document.create_element("img").unwrap();
                if let Some(src) = &x.url {
                    link.set_attribute("src", src).unwrap();
                }
                if let Some(title) = &x.title {
                    link.set_attribute("title", title).unwrap();
                }
                node.cur = Some(DocNode::Element(link));
                node.parent = parent;
                node.cur.as_ref()
            }
            Node::Link(x, node) => {
                let link = document.create_element("a").unwrap();
                if let Some(url) = &x.url {
                    link.set_attribute("href", url).unwrap();
                }
                if let Some(title) = &x.title {
                    link.set_attribute("title", title).unwrap();
                }
                node.cur = Some(DocNode::Element(link));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Strong(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("strong").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Text(x, node) => {
                node.cur = Some(DocNode::Text(
                    document.create_text_node(&x.value.clone().unwrap()),
                ));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Code(x, node) => {
                let pre = document.create_element("pre").unwrap();
                let code = document.create_element("code").unwrap();
                let content = document
                    .create_text_node(&x.value.clone().map_or_else(|| "".to_string(), |x| x));
                pre.append_child(&code)
                    .unwrap()
                    .append_child(&content)
                    .unwrap();
                node.cur = Some(DocNode::Element(pre));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Heading(x, node) => {
                node.cur = Some(DocNode::Element(
                    document
                        .create_element(create_heading_text(x.depth))
                        .unwrap(),
                ));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Table(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("table").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::TableHead(x, node) => {
                // let head = document.create_element("thead").unwrap();
                let tr = document.create_element("tr").unwrap();

                // head.append_child(&tr).unwrap();
                node.cur = Some(DocNode::Element(tr));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::TableRow(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("tr").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::TableCell(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("th").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::ListItem(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("li").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
            Node::Paragraph(x, node) => {
                node.cur = Some(DocNode::Element(document.create_element("p").unwrap()));
                node.parent = parent;
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            }
        }
    }
    // #[must_use]
    // pub fn position(&self) -> Option<&Position> {
    //     match self {
    //         Node::Root(x) => x.position.as_ref(),
    //         // Node::BlockQuote(x) => x.position.as_ref(),
    //         // Node::FootnoteDefinition(x) => x.position.as_ref(),
    //         // Node::MdxJsxFlowElement(x) => x.position.as_ref(),
    //         Node::List(x) => x.position.as_ref(),
    //         // Node::MdxjsEsm(x) => x.position.as_ref(),
    //         // Node::Toml(x) => x.position.as_ref(),
    //         // Node::Yaml(x) => x.position.as_ref(),
    //         // Node::Break(x) => x.position.as_ref(),
    //         // Node::InlineCode(x) => x.position.as_ref(),
    //         // Node::InlineMath(x) => x.position.as_ref(),
    //         // Node::Delete(x) => x.position.as_ref(),
    //         // Node::Emphasis(x) => x.position.as_ref(),
    //         // Node::MdxTextExpression(x) => x.position.as_ref(),
    //         // Node::FootnoteReference(x) => x.position.as_ref(),
    //         // Node::Html(x) => x.position.as_ref(),
    //         // Node::Image(x) => x.position.as_ref(),
    //         // Node::ImageReference(x) => x.position.as_ref(),
    //         // Node::MdxJsxTextElement(x) => x.position.as_ref(),
    //         // Node::Link(x) => x.position.as_ref(),
    //         // Node::LinkReference(x) => x.position.as_ref(),
    //         // Node::Strong(x) => x.position.as_ref(),
    //         Node::Text(x) => x.position.as_ref(),
    //         // Node::Code(x) => x.position.as_ref(),
    //         // Node::Math(x) => x.position.as_ref(),
    //         // Node::MdxFlowExpression(x) => x.position.as_ref(),
    //         Node::Heading(x) => x.position.as_ref(),
    //         // Node::Table(x) => x.position.as_ref(),
    //         // Node::ThematicBreak(x) => x.position.as_ref(),
    //         // Node::TableRow(x) => x.position.as_ref(),
    //         // Node::TableCell(x) => x.position.as_ref(),
    //         // Node::ListItem(x) => x.position.as_ref(),
    //         // Node::Definition(x) => x.position.as_ref(),
    //         // Node::Paragraph(x) => x.position.as_ref(),
    //     }
    // }

    // pub fn position_mut(&mut self) -> Option<&mut Position> {
    //     match self {
    //         Node::Root(x) => x.position.as_mut(),
    //         // Node::BlockQuote(x) => x.position.as_mut(),
    //         // Node::FootnoteDefinition(x) => x.position.as_mut(),
    //         // Node::MdxJsxFlowElement(x) => x.position.as_mut(),
    //         Node::List(x) => x.position.as_mut(),
    //         // Node::MdxjsEsm(x) => x.position.as_mut(),
    //         // Node::Toml(x) => x.position.as_mut(),
    //         // Node::Yaml(x) => x.position.as_mut(),
    //         // Node::Break(x) => x.position.as_mut(),
    //         // Node::InlineCode(x) => x.position.as_mut(),
    //         // Node::InlineMath(x) => x.position.as_mut(),
    //         // Node::Delete(x) => x.position.as_mut(),
    //         // Node::Emphasis(x) => x.position.as_mut(),
    //         // Node::MdxTextExpression(x) => x.position.as_mut(),
    //         // Node::FootnoteReference(x) => x.position.as_mut(),
    //         // Node::Html(x) => x.position.as_mut(),
    //         // Node::Image(x) => x.position.as_mut(),
    //         // Node::ImageReference(x) => x.position.as_mut(),
    //         // Node::MdxJsxTextElement(x) => x.position.as_mut(),
    //         // Node::Link(x) => x.position.as_mut(),
    //         // Node::LinkReference(x) => x.position.as_mut(),
    //         // Node::Strong(x) => x.position.as_mut(),
    //         Node::Text(x) => x.position.as_mut(),
    //         // Node::Code(x) => x.position.as_mut(),
    //         // Node::Math(x) => x.position.as_mut(),
    //         // Node::MdxFlowExpression(x) => x.position.as_mut(),
    //         Node::Heading(x) => x.position.as_mut(),
    //         // Node::Table(x) => x.position.as_mut(),
    //         // Node::ThematicBreak(x) => x.position.as_mut(),
    //         // Node::TableRow(x) => x.position.as_mut(),
    //         // Node::TableCell(x) => x.position.as_mut(),
    //         // Node::ListItem(x) => x.position.as_mut(),
    //         // Node::Definition(x) => x.position.as_mut(),
    //         // Node::Paragraph(x) => x.position.as_mut(),
    //     }
    // }

    // pub fn position_set(&mut self, position: Option<Position>) {
    //     match self {
    //         Node::Root(x) => x.position = position,
    //         // Node::BlockQuote(x) => x.position = position,
    //         // Node::FootnoteDefinition(x) => x.position = position,
    //         // Node::MdxJsxFlowElement(x) => x.position = position,
    //         Node::List(x) => x.position = position,
    //         // Node::MdxjsEsm(x) => x.position = position,
    //         // Node::Toml(x) => x.position = position,
    //         // Node::Yaml(x) => x.position = position,
    //         // Node::Break(x) => x.position = position,
    //         // Node::InlineCode(x) => x.position = position,
    //         // Node::InlineMath(x) => x.position = position,
    //         // Node::Delete(x) => x.position = position,
    //         // Node::Emphasis(x) => x.position = position,
    //         // Node::MdxTextExpression(x) => x.position = position,
    //         // Node::FootnoteReference(x) => x.position = position,
    //         // Node::Html(x) => x.position = position,
    //         // Node::Image(x) => x.position = position,
    //         // Node::ImageReference(x) => x.position = position,
    //         // Node::MdxJsxTextElement(x) => x.position = position,
    //         // Node::Link(x) => x.position = position,
    //         // Node::LinkReference(x) => x.position = position,
    //         // Node::Strong(x) => x.position = position,
    //         Node::Text(x) => x.position = position,
    //         // Node::Code(x) => x.position = position,
    //         // Node::Math(x) => x.position = position,
    //         // Node::MdxFlowExpression(x) => x.position = position,
    //         Node::Heading(x) => x.position = position,
    //         // Node::Table(x) => x.position = position,
    //         // Node::ThematicBreak(x) => x.position = position,
    //         // Node::TableRow(x) => x.position = position,
    //         // Node::TableCell(x) => x.position = position,
    //         // Node::ListItem(x) => x.position = position,
    //         // Node::Definition(x) => x.position = position,
    //         // Node::Paragraph(x) => x.position = position,
    //     }
    // }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Root {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Heading {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
    pub depth: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub value: Option<String>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    pub index: Option<u64>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ListItem {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Code {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Paragraph {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Emphasis {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Strong {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
    pub alignment: Vec<Alignment>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TableHead {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TableRow {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TableCell {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockQuote {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Delete {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub children: Vec<Rc<RefCell<Node>>>,
    pub position: Option<Position>,
    pub link_type: LinkType,
    pub url: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub position: Option<Position>,
    pub link_type: LinkType,
    pub url: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoftBreak {
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HardBreak {
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InlineCode {
    pub value: Option<String>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Html {
    pub value: Option<String>,
    pub position: Option<Position>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElementNode {
    pub cur: Option<DocNode>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextNode {
    pub cur: Option<DocNode>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl ElementNode {
    pub fn default() -> Self {
        ElementNode {
            cur: None,
            parent: None,
        }
    }
}

impl TextNode {
    pub fn default() -> Self {
        TextNode {
            cur: None,
            parent: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DocNode {
    Element(Element),
    Text(TextElement),
}

fn create_heading_text(depth: u8) -> &'static str {
    match depth {
        1 => "h1",
        2 => "h2",
        3 => "h3",
        4 => "h4",
        5 => "h5",
        6 => "h6",
        // Add more cases as needed
        _ => "Unknown heading level",
    }
}
