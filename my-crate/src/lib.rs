
mod utils;
mod to_mdast;
mod event;
mod mdast;

use to_mdast::CompileContext;
use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use utils::set_panic_hook;
use std::collections::HashMap;
use rand::Rng;
use cfg_if::cfg_if;
use web_sys::window;
use pulldown_cmark::{html, Options, Parser};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

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

pub struct BufferStorage {
  pub buffer_map: HashMap<String, Vec<u8>>
}

impl BufferStorage {
    fn new() -> Self {
      BufferStorage {
        buffer_map: HashMap::new()
      }
    }
}

macro_rules! log {
  ($($t:tt)*) => (crate::log(&("[C]".to_string() + &format_args!($($t)*).to_string())))
}

lazy_static! {
  pub static ref GlobalBufferStorage: Mutex<BufferStorage> = {
    let buffer_storage = BufferStorage::new();
    Mutex::new(buffer_storage)
  };
}

#[wasm_bindgen]
pub fn set_wasm_panic_hook() {
  // can be continued
  set_panic_hook();
}


#[wasm_bindgen]
pub fn get_buffer(key: String) -> *const u8 {
  let mut global_buffer_storage = GlobalBufferStorage.lock().unwrap();
  if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
    return buffer.as_ptr();
  } else {
    return Vec::new().as_ptr();
  }
}

#[wasm_bindgen]
pub fn print_buffer(key: String) {
  let mut global_buffer_storage = GlobalBufferStorage.lock().unwrap();
  if let Some(buffer) = global_buffer_storage.buffer_map.get(&key) {
    log!("[render-wasm]print buffer: {:?}", buffer);
  }
}

#[wasm_bindgen]
pub fn remove_buffer(key: String) {
  let mut global_buffer_storage = GlobalBufferStorage.lock().unwrap();
  if let Some(buffer) = global_buffer_storage.buffer_map.remove(&key) {
    log!("remove buffer success");
  } else {
    log!("remove buffer error");
  }
}

#[wasm_bindgen]
pub fn render_markdown(key: String) -> String {
    // let mut global_buffer_storage = GlobalBufferStorage.lock().unwrap();
    let mut html_output = String::new();
    // if let Some(buffer) = global_buffer_storage.buffer_map.get_mut(&key){
        let options = Options::empty();
        let parser = Parser::new_ext(&key, options);
        // let parser = Parser::new_ext(std::str::from_utf8(&buffer).unwrap(), options);
        html::push_html(&mut html_output, parser);
    // }
    html_output
}

#[wasm_bindgen]
pub fn new_buffer(key: String, len: usize) -> *const u8 {
  log!("new_buffer, key: {:?}, len: {:?}", key, len);
  let mut global_buffer_storage = GlobalBufferStorage.lock().unwrap();
  let mut buffer = vec![255; len];
  // 这里我们增加一个随机数逻辑：
//   for val in buffer.iter_mut() {
//     *val = __random();
//   }
  let ptr = buffer.as_ptr();
  global_buffer_storage.buffer_map.insert(key, buffer);
  ptr
}

#[wasm_bindgen]
pub fn wwwwaaaa()->String  {
    let window = window().unwrap();
    let doc = window.document().unwrap();
    // 获取test节点（我在html声明了一个div）
    let test_node = doc.get_element_by_id("test").unwrap();
    // 在节点里添加内容
    test_node.set_text_content(Some("Rust 操作 Dom"));

    // 最后再来个alert
    // let _ = window.alert_with_message("我是通过 web_sys 生成的");
    String::from('a')
}
#[wasm_bindgen]
pub fn t(markdown_input:String) {
  
  let mut compileContext: CompileContext = CompileContext::new();
  let parser = Parser::new(&markdown_input);
  let node = compileContext.run(parser).unwrap();
  //  print!("{:#?}", node)
}