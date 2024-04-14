use std::{cell::RefCell, rc::Rc};

use web_sys::window;

use crate::{mdast::Node, DocNode};

pub fn update(old_rc: Rc<RefCell<Node>>, cur_rc: Rc<RefCell<Node>>) {
    let value: Option<String>;
    let title: Option<String>;
    let url: Option<String>;
    {
        let old = old_rc.borrow();
        let mut cur = cur_rc.borrow_mut();
        // if *old==*cur{
        //     return
        // }
        cur.set_node(old.node_mut());
        // cur.set_parent(old.parent());
        // element = cur.node_mut();
        value = cur.value().cloned();
        url = cur.url().cloned();
        title = cur.title().cloned();
    }
    // let parent = old.parent().unwrap().borrow_mut().node_mut();
    match value {
        None => {
            match url {
                Some(cur_url) => {
                    let cur = cur_rc.borrow_mut();
                    let element = cur.node_mut();
                    match element.unwrap() {
                        DocNode::Element(node_element) => {
                            if node_element.tag_name().to_lowercase() == "a" {
                                let _ = node_element.set_attribute("href", &cur_url);
                            } else {
                                let _ = node_element.set_attribute("src", &cur_url);
                            }

                            let _ = node_element.set_attribute("title", &title.unwrap());
                        }
                        DocNode::Text(_) => {}
                    }
                }
                None => {}
            }
            update_children(old_rc.clone(), cur_rc.clone());
        }
        Some(cur_str) => {
            let old = old_rc.borrow();
            match old.value().cloned() {
                Some(old_str) => {
                    if cur_str != old_str {
                        let cur = cur_rc.borrow_mut();
                        let element = cur.node_mut();
                        match element.unwrap() {
                            DocNode::Element(node_element) => {
                                node_element
                                    .first_child()
                                    .unwrap()
                                    .set_text_content(Some(&cur_str));
                            }
                            DocNode::Text(text_element) => {
                                text_element.set_node_value(Some(&cur_str));
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }
    // cur
}

fn update_children(old: Rc<RefCell<Node>>, new: Rc<RefCell<Node>>) {
    let window = window().unwrap();
    let doc = window.document().unwrap();
    let old_el = old.borrow();
    let new_el = new.borrow();
    // log(&(*old_el==*new_el).to_string());
    if let Some(old_children) = old_el.children() {
        if let Some(new_children) = new_el.children() {
            if old_children.len() > 0 && new_children.len() > 0 {
                let parent_node = new_el.node_mut().unwrap();
                path_children(old_children, new_children, new.clone(), parent_node);
                // log(&old_children.len().to_string());
                // log(&new_children.len().to_string());
            } else if new_children.len() > 0 {
                // log("555");
                for child in new_children {
                    let mut cur_child = child.borrow_mut();
                    cur_child.create(&doc, new.clone());
                }
            } else {
                // log("666");
                match new_el.node_mut().unwrap() {
                    crate::DocNode::Element(element) => {
                        while let Some(child) = element.first_child() {
                            element
                                .remove_child(&child)
                                .expect("Failed to remove child");
                        }
                    }
                    crate::DocNode::Text(_) => {}
                }
            }
        }
    }
}

fn path_children(
    old_ch: &Vec<Rc<RefCell<Node>>>,
    new_ch: &Vec<Rc<RefCell<Node>>>,
    parent: Rc<RefCell<Node>>,
    parent_node: &DocNode,
) {
    let window = window().unwrap();
    let doc = window.document().unwrap();
    let mut old_start_idx = 0;
    let mut new_start_idx = 0;
    let mut old_end_idx = old_ch.len() - 1;
    let mut new_end_idx = new_ch.len() - 1;
    while old_start_idx <= old_end_idx && new_start_idx <= new_end_idx {
        // log(&(old_ch[old_start_idx] == new_ch[new_start_idx]).to_string());
        if old_ch[old_start_idx] == new_ch[new_start_idx] {
            update(old_ch[old_start_idx].clone(), new_ch[new_start_idx].clone());
            old_start_idx += 1;
            new_start_idx += 1;
        } else if old_ch[old_end_idx] == new_ch[new_end_idx] {
            update(old_ch[old_end_idx].clone(), new_ch[new_end_idx].clone());
            old_end_idx -= 1;
            new_end_idx -= 1;
        } else if old_ch[old_start_idx] == new_ch[new_end_idx] {
            update(old_ch[old_start_idx].clone(), new_ch[new_end_idx].clone());
            let old_pre = old_ch[old_start_idx].borrow();
            let element = old_pre.node_mut();
            match parent_node {
                DocNode::Element(parent_element) => match element.unwrap() {
                    DocNode::Element(child_element) => {
                        let brother_node = old_ch[old_end_idx + 1].borrow();
                        let brother = brother_node.node_mut();
                        match brother.unwrap() {
                            DocNode::Element(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                            DocNode::Text(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                        }
                    }
                    DocNode::Text(child_element) => {
                        let brother_node = old_ch[old_end_idx + 1].borrow();
                        let brother = brother_node.node_mut();
                        match brother.unwrap() {
                            DocNode::Element(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                            DocNode::Text(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                        }
                    }
                },
                DocNode::Text(_) => {}
            }
            old_start_idx += 1;
            new_end_idx -= 1;
        } else if old_ch[old_end_idx] == new_ch[new_start_idx] {
            update(old_ch[old_end_idx].clone(), new_ch[new_start_idx].clone());
            let old_pre = old_ch[old_end_idx].borrow();
            let element = old_pre.node_mut();
            match parent_node {
                DocNode::Element(parent_element) => match element.unwrap() {
                    DocNode::Element(child_element) => {
                        let brother_node = old_ch[old_start_idx].borrow();
                        let brother = brother_node.node_mut();
                        match brother.unwrap() {
                            DocNode::Element(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                            DocNode::Text(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                        }
                    }
                    DocNode::Text(child_element) => {
                        let brother_node = old_ch[old_start_idx].borrow();
                        let brother = brother_node.node_mut();
                        match brother.unwrap() {
                            DocNode::Element(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                            DocNode::Text(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                        }
                    }
                },
                DocNode::Text(_) => {}
            }
            old_end_idx -= 1;
            new_start_idx += 1;
        } else {
            // TODO
            break;
        }
    }
    for idx in new_start_idx..new_end_idx + 1 {
        {
            let mut child_node = new_ch[idx].borrow_mut();
            let _ = child_node.create_node(&doc, Some(parent.clone()));
        }
        {
            let child_node = new_ch[idx].borrow();
            if let Some(children) = child_node.children() {
                for child in children {
                    let mut cur_child = child.borrow_mut();
                    cur_child.create(&doc, new_ch[idx].clone());
                }
            }
        }

        let child_node = new_ch[idx].borrow_mut();
        let element = child_node.node_mut();
        match parent_node {
            DocNode::Element(parent_element) => match element.unwrap() {
                DocNode::Element(child_element) => {
                    if old_start_idx < old_ch.len() {
                        let brother_node = old_ch[old_start_idx].borrow();
                        let brother = brother_node.node_mut();
                        match brother.unwrap() {
                            DocNode::Element(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                            DocNode::Text(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                        }
                    } else {
                        let _ = parent_element.append_child(&child_element);
                    }
                }
                DocNode::Text(child_element) => {
                    if old_start_idx < old_ch.len() {
                        let brother_node = old_ch[old_start_idx].borrow();
                        let brother = brother_node.node_mut();
                        match brother.unwrap() {
                            DocNode::Element(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                            DocNode::Text(brother_element) => {
                                let _ = parent_element
                                    .insert_before(&child_element, Some(&brother_element));
                            }
                        }
                    } else {
                        let _ = parent_element.append_child(&child_element);
                    }
                }
            },
            DocNode::Text(_) => {}
        }
    }
    for idx in old_start_idx..old_end_idx + 1 {
        let child_node = old_ch[idx].borrow_mut();
        let element = child_node.node_mut();
        match parent_node {
            DocNode::Element(parent_element) => match element.unwrap() {
                DocNode::Element(child_element) => {
                    let _ = parent_element.remove_child(&child_element);
                }
                DocNode::Text(child_element) => {
                    let _ = parent_element.remove_child(&child_element);
                }
            },
            DocNode::Text(_) => {}
        }
    }
}
