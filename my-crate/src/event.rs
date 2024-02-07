use std::{clone, fmt};

use pulldown_cmark::{BrokenLinkCallback, CowStr, Event, Parser, Tag};


// pub struct Point {
//     /// 1-indexed integer representing a line in a source file.
//     pub line: usize,
//     /// 1-indexed integer representing a column in a source file.
//     pub column: usize,
//     /// 0-indexed integer representing a character in a source file.
//     pub offset: usize,
// }

// impl Point {
//     #[must_use]
//     pub fn new(line: usize, column: usize, offset: usize) -> Point {
//         Point {
//             line,
//             column,
//             offset,
//         }
//     }
// }

// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}:{} ({})", self.line, self.column, self.offset)
//     }
// }
#[derive(Clone)]
pub struct Position {
    /// Represents the place of the first character of the parsed source region.
    pub start: usize,
    /// Represents the place of the first character after the parsed source
    /// region, whether it exists or not.
    pub end: usize,
}

impl Position {
    fn new(&mut self,start:usize,end:usize)->Option<Position>{
        Some(Position{start:start,end:end})
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            self.start,
            self.end
        )
    }
}


// impl Position {
//     #[must_use]
//     pub fn new(
//         start_line: usize,
//         start_column: usize,
//         start_offset: usize,
//         end_line: usize,
//         end_column: usize,
//         end_offset: usize,
//     ) -> Position {
//         Position {
//             start: Point::new(start_line, start_column, start_offset),
//             end: Point::new(end_line, end_column, end_offset),
//         }
//     }
// }

// 新的 Event 类型，包含 Position 信息
// #[derive(Debug)]
// pub enum PositionedEvent {
//     Start(Tag, Position),
//     End(Tag, Position),
//     Text(String, Position),
//     Code(CowStr, Position),
//     Html(CowStr, Position),
//     InlineHtml(CowStr, Position),
//     FootnoteReference(CowStr, Position),
//     SoftBreak(Position),
//     HardBreak(Position),
//     Rule(Position),
//     TaskListMarker(bool, Position),
// }

// 位置信息的结构体

// 实现 Iterator trait 的新类型
// pub struct PositionedParser<'a, 'callback> {
//     parser: Parser<'a, 'callback>,
// }

// impl<'a, 'callback> Iterator for PositionedParser<'a, 'callback> {
//     type Item = PositionedEvent;

//     fn next(&mut self) -> Option<PositionedEvent> {
//         match self.parser.next() {
//             None => None,
//             Some(event) => {
//                 // 获取事件的位置信息，或者根据需要计算位置信息
//                 let position = Position {
//                     line: 1, // 示例中简化为行号 1
//                     column: 1, // 示例中简化为列号 1
//                 };

//                 // 构建新的 PositionedEvent
//                 let positioned_event = match event {
//                     Event::Start(tag) => PositionedEvent::Start(tag, position.clone()),
//                     Event::End(tag) => PositionedEvent::End(tag, position.clone()),
//                     Event::Text(text) => PositionedEvent::Text(text.to_string(), position.clone()),
//                     // 其他 Event 类型...
//                 };

//                 Some(positioned_event)
//             }
//         }
//     }
// }

// fn main() {
//     // 示例：创建 PositionedParser
//     let markdown_input = "Some **markdown** text.";
//     let parser = pulldown_cmark::Parser::new(markdown_input);
//     let positioned_parser = PositionedParser { parser };

//     // 遍历 PositionedParser 获取 PositionedEvent
//     for positioned_event in positioned_parser {
//         println!("{:?}", positioned_event);
//     }
// }
