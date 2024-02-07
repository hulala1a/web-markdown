use std::{fmt::{self, Debug}, rc::Rc};


use pulldown_cmark::{Alignment, HeadingLevel, LinkType};
use serde::de::value;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element,Text as TextElement};

use crate::event::Position;

#[derive(Clone)]
pub enum Node {
    // Document:
    /// Root.
    Root(Root,ElementNode),

    // Container:
    /// Block quote.
    BlockQuote(BlockQuote),
    /// Footnote definition.
    // FootnoteDefinition(FootnoteDefinition),
    /// MDX: JSX element (container).
    // MdxJsxFlowElement(MdxJsxFlowElement),
    /// List.
    List(List),

    // Frontmatter:
    /// MDX.js ESM.
    // MdxjsEsm(MdxjsEsm),
    /// Toml.
    // Toml(Toml),
    /// Yaml.
    // Yaml(Yaml),

    // Phrasing:
    /// Break.
    SoftBreak(SoftBreak),
    HardBreak(HardBreak),
    /// Code (phrasing).
    InlineCode(InlineCode),
    /// Math (phrasing).
    // InlineMath(InlineMath),
    /// Delete.
    Delete(Delete),
    // /// Emphasis.
    Emphasis(Emphasis),
    // // MDX: expression (text).
    // MdxTextExpression(MdxTextExpression),
    // /// Footnote reference.
    // FootnoteReference(FootnoteReference),
    // /// Html (phrasing).
    Html(Html),
    // /// Image.
    Image(Image),
    // /// Image reference.
    // ImageReference(ImageReference),
    // // MDX: JSX element (text).
    // MdxJsxTextElement(MdxJsxTextElement),
    // /// Link.
    Link(Link),
    // /// Link reference.
    // LinkReference(LinkReference),
    // /// Strong
    Strong(Strong),
    // /// Text.
    Text(Text,TextNode),

    // // Flow:
    // /// Code (flow).
    Code(Code),
    // /// Math (flow).
    // Math(Math),
    // // MDX: expression (flow).
    // MdxFlowExpression(MdxFlowExpression),
    // /// Heading.
    Heading(Heading,ElementNode),
    // Html(Html),
    Table(Table),
    TableHead(TableHead),
    // /// Thematic break.
    // ThematicBreak(ThematicBreak),

    // // Table content.
    // /// Table row.
    TableRow(TableRow),

    // // Row content.
    // /// Table cell.
    TableCell(TableCell),

    // // List content.
    // /// List item.
    ListItem(ListItem),

    // // Content.
    // /// Definition.
    // Definition(Definition),
    // /// Paragraph.
    Paragraph(Paragraph,ElementNode),
}

impl fmt::Debug for Node {
    // Debug the wrapped struct.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Root(x, _) => x.fmt(f),
            Node::BlockQuote(x) => x.fmt(f),
            // Node::FootnoteDefinition(x) => x.fmt(f),
            // Node::MdxJsxFlowElement(x) => x.fmt(f),
            Node::List(x) => x.fmt(f),
            // Node::MdxjsEsm(x) => x.fmt(f),
            // Node::Toml(x) => x.fmt(f),
            // Node::Yaml(x) => x.fmt(f),
            Node::SoftBreak(x) => x.fmt(f),
            Node::HardBreak(x) => x.fmt(f),
            Node::InlineCode(x) => x.fmt(f),
            // Node::InlineMath(x) => x.fmt(f),
            Node::Delete(x) => x.fmt(f),
            Node::Emphasis(x) => x.fmt(f),
            // Node::MdxTextExpression(x) => x.fmt(f),
            // Node::FootnoteReference(x) => x.fmt(f),
            Node::Html(x) => x.fmt(f),
            Node::Image(x) => x.fmt(f),
            // Node::ImageReference(x) => x.fmt(f),
            // Node::MdxJsxTextElement(x) => x.fmt(f),
            Node::Link(x) => x.fmt(f),
            // Node::LinkReference(x) => x.fmt(f),
            Node::Strong(x) => x.fmt(f),
            Node::Text(x,_) => x.fmt(f),
            Node::Code(x) => x.fmt(f),
            // Node::Math(x) => x.fmt(f),
            // Node::MdxFlowExpression(x) => x.fmt(f),
            Node::Heading(x, _) => x.fmt(f),
            Node::Table(x) => x.fmt(f),
            // Node::ThematicBreak(x) => x.fmt(f),
            Node::TableHead(x) => x.fmt(f),
            Node::TableRow(x) => x.fmt(f),
            Node::TableCell(x) => x.fmt(f),
            Node::ListItem(x) => x.fmt(f),
            // Node::Definition(x) => x.fmt(f),
            Node::Paragraph(x,_) => x.fmt(f),
        }
    }
}

impl Node {
    #[must_use]
    pub fn children(& self) -> Option<&Vec<Node>> {
        match self {
            // Parent.
            Node::Root(x, _) => Some(&x.children),
            Node::Paragraph(x,_) => Some(&x.children),
            Node::Heading(x, _) => Some(&x.children),
            Node::BlockQuote(x) => Some(&x.children),
            Node::List(x) => Some(&x.children),
            Node::ListItem(x) => Some(&x.children),
            Node::Emphasis(x) => Some(&x.children),
            Node::Strong(x) => Some(&x.children),
            Node::Link(x) => Some(&x.children),
            // Node::LinkReference(x) => Some(&x.children),
            // Node::FootnoteDefinition(x) => Some(&x.children),
            Node::Table(x) => Some(&x.children),
            Node::TableHead(x) => Some(&x.children),
            Node::TableRow(x) => Some(&x.children),
            Node::TableCell(x) => Some(&x.children),
            Node::Delete(x) => Some(&x.children),
            // Node::MdxJsxFlowElement(x) => Some(&x.children),
            // Node::MdxJsxTextElement(x) => Some(&x.children),
            // Non-parent.
            _ => None,
        }
    }

    pub fn children_mut(& mut self) -> Option<&mut Vec<Node>> {
        match self {
            // Parent.
            Node::Root(x, _) => Some(&mut x.children),
            Node::Paragraph(x,_) => Some(&mut x.children),
            Node::Heading(x, _) => Some(&mut x.children),
            Node::BlockQuote(x) => Some(&mut x.children),
            Node::List(x) => Some(&mut x.children),
            Node::ListItem(x) => Some(&mut x.children),
            Node::Emphasis(x) => Some(&mut x.children),
            Node::Strong(x) => Some(&mut x.children),
            Node::Link(x) => Some(&mut x.children),
            // Node::LinkReference(x) => Some(&mut x.children),
            // Node::FootnoteDefinition(x) => Some(&mut x.children),
            Node::Table(x) => Some(&mut x.children),
            Node::TableHead(x) => Some(&mut x.children),
            Node::TableRow(x) => Some(&mut x.children),
            Node::TableCell(x) => Some(&mut x.children),
            Node::Delete(x) => Some(&mut x.children),
            // Node::MdxJsxFlowElement(x) => Some(&mut x.children),
            // Node::MdxJsxTextElement(x) => Some(&mut x.children),
            // Non-parent.
            _ => None,
        }
    }

    pub fn value_set(&mut self,value:String){
        match self {
            // Parent.
            Node::Code(x) => x.value = Some(value),
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
            _=>{}
        }
    }
    pub fn create_node(& mut self,document:&Document,parent:&Node)->Option<&DocNode>{
        match self {
            Node::Root(x, _) => todo!(),
            Node::BlockQuote(_) => todo!(),
            Node::List(_) => todo!(),
            Node::SoftBreak(_) => todo!(),
            Node::HardBreak(_) => todo!(),
            Node::InlineCode(_) => todo!(),
            Node::Delete(_) => todo!(),
            Node::Emphasis(_) => todo!(),
            Node::Html(_) => todo!(),
            Node::Image(_) => todo!(),
            Node::Link(_) => todo!(),
            Node::Strong(_) => todo!(),
            Node::Text(x,node) => {
                node.cur = Some(DocNode::Text(document.create_text_node(&x.value.clone().unwrap())));
                node.parent = Box::new(parent);
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            },
            Node::Code(_) => todo!(),
            Node::Heading(x,node) => {
                node.cur = Some(DocNode::Element(document.create_element(create_heading_text(x.depth)).unwrap()));
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            },
            Node::Table(_) => todo!(),
            Node::TableHead(_) => todo!(),
            Node::TableRow(_) => todo!(),
            Node::TableCell(_) => todo!(),
            Node::ListItem(_) => todo!(),
            Node::Paragraph(x,node) => {
                node.cur = Some(DocNode::Element(document.create_element("p").unwrap()));
                node.cur.as_ref()
                // for child in x.children{
                //     child.create_node(document);
                // }
            
            },
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

#[derive(Clone,Debug)]
pub struct Root {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Heading {
    // Parent.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    // Extra.
    /// Rank (between `1` and `6`, both including).
    pub depth: u8,
}
#[derive(Clone,Debug)]
pub struct Text {
    // Text.
    /// Content model.
    pub value: Option<String>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct List {
    // Text.
    /// Content model.
    pub index: Option<u64>,
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}
#[derive(Clone,Debug)]
pub struct ListItem {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Code {
    // Text.
    /// Content model.
    pub lang: Option<String>,
    pub value: Option<String>,
    // pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Paragraph {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Emphasis {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Strong {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Table {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    pub alignment: Vec<Alignment>,
}
#[derive(Clone,Debug)]
pub struct TableHead {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}
#[derive(Clone,Debug)]
pub struct TableRow {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}
#[derive(Clone,Debug)]
pub struct TableCell {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct BlockQuote {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Delete {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Link {
    // Text.
    /// Content model.
    pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    pub link_type: LinkType,
    pub url: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone,Debug)]
pub struct Image {
    // Text.
    /// Content model.
    // pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
    pub link_type: LinkType,
    pub url: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone,Debug)]
pub struct SoftBreak {
    // Text.
    /// Content model.
    // pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct HardBreak {
    // Text.
    /// Content model.
    // pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct InlineCode {
    pub value:Option<String>,
    // Text.
    /// Content model.
    // pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone,Debug)]
pub struct Html {
    pub value:Option<String>,
    // Text.
    /// Content model.
    // pub children: Vec<Node>,
    /// Positional info.
    pub position: Option<Position>,
}

#[derive(Clone, Debug)]
pub struct ElementNode {
    pub cur: Option<DocNode>,
    pub parent: Option<Box<Node>>,
}

#[derive(Clone, Debug)]
pub struct TextNode {
    pub cur: Option<DocNode>,
    pub parent: Option<Box<Node>>,
}

impl ElementNode {
    pub fn default() -> Self {
        ElementNode { cur: None, parent: None }
    }
}

impl TextNode {
    pub fn default() -> Self {
        TextNode { cur: None, parent: None }
    }
}

#[derive(Clone, Debug)]
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