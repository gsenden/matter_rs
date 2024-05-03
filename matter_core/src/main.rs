mod core;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::core::common;

mod test_utils;
use crate::core::xy::XY;
use crate::test_utils::geometry_test_utils;

mod geometry;
use crate::geometry::vector;

mod body_mod;
use crate::body_mod::body;


// struct Vertex {
//     body: Body,
//     y: f64,
// }

// impl Vertex {
//     pub fn new(body: Body, y: f64) -> Self {
//         Vertex { body: body, y: y}
//     }
// }

// struct BodyContent {
//     x: f64,
//     parts: Option<Vec<Body>>,
// }

// struct Body {
//     content: Rc<RefCell<BodyContent>>,
//     parent: Weak<RefCell<BodyContent>>,
    
// }

// impl Body {
//     pub fn new(x: f64) -> Self {
//         let content = BodyContent { x: x, parts: None };
//         Body { content: Rc::new(RefCell::new(content)), parent: Weak::new() }
//     }

//     fn clone(&self) -> Body {
//         Body { content: self.content.clone(), parent: self.parent.clone() }
//     }

//     pub fn get_x(&self) -> f64 {
//         self.content.as_ref().borrow().x
//     }

//     pub fn set_x(&mut self, x: f64 ) {
//         self.content.as_ref().borrow_mut().x = x;
//     } 

//     pub fn set_parent(&mut self, parent: &Body) {
//         self.parent = Rc::downgrade(&parent.content);
//     }

//     pub fn get_parent(&self) -> Option<Body> {
//         if let Some(content) = self.parent.upgrade() {
//             Some(Body { content, parent: Weak::new() })
//         } else {
//             None
//         }
//     }

//     pub fn set_parts(&mut self, parts: Vec<Body>) {
//         let mut this = self.content.as_ref().borrow_mut();
//         this.parts = Some(parts);
//     }

//     pub fn get_parts(&self) -> Vec<Body> {
//         let mut parts = vec![self.clone()];
//         let this = self.content.as_ref().borrow();
//         if let Some(my_parts) = &this.parts {
//             for part in my_parts.iter() {
//                 parts.push(part.clone());
//             }
//         }
//         parts
//     }

// }


fn main() {
    // let mut d = Body::new(12.);
    // let mut e = Body::new(20.);

    // d.set_parent(&e);
    // d.get_parent().unwrap().set_x(13.);


    
    // println!("{}", e.get_x());
    // let a = Rc::new(RefCell::new(Body{x: 1., parent: Weak::new()}));
    // let b = Rc::new(RefCell::new(Body{x: 10., parent: Rc::downgrade(&a)}));
    // a.as_ref().borrow_mut().x = 2.;

    // let c = b.as_ref().borrow().parent.upgrade().unwrap().as_ref().borrow().x;
    // println!("{}",c);

    let vector = vector::create(2.0, 2.0);
    println!("Hello, world! {}", vector.get_x());
}
