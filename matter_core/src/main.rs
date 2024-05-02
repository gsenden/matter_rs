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


struct InnerBody {
    x: f64,
}

struct Body {
    inner: Rc<RefCell<InnerBody>>,
    parent: Weak<RefCell<InnerBody>>
}

impl Body {
    pub fn new(x: f64) -> Self {
        let inner = InnerBody { x: x };
        Body { inner: Rc::new(RefCell::new(inner)), parent: Weak::new() }
    }

    pub fn get_x(&self) -> f64 {
        self.inner.as_ref().borrow().x
    }

    pub fn set_x(&mut self, x: f64 ) {
        self.inner.as_ref().borrow_mut().x = x;
    } 

    pub fn set_parent(&mut self, parent: &Body) {
        self.parent = Rc::downgrade(&parent.inner);
    }

    pub fn get_parent(&self) -> Option<Body> {
        if let Some(body) = self.parent.upgrade() {
            Some(Body { inner: body, parent: Weak::new() })
        } else {
            None
        }
    }
}


fn main() {
    let mut d = Body::new(12.);
    let mut e = Body::new(20.);

    d.set_parent(&e);
    d.get_parent().unwrap().set_x(13.);


    
    println!("{}", e.get_x());
    // let a = Rc::new(RefCell::new(Body{x: 1., parent: Weak::new()}));
    // let b = Rc::new(RefCell::new(Body{x: 10., parent: Rc::downgrade(&a)}));
    // a.as_ref().borrow_mut().x = 2.;

    // let c = b.as_ref().borrow().parent.upgrade().unwrap().as_ref().borrow().x;
    // println!("{}",c);

    let vector = vector::create(2.0, 2.0);
    println!("Hello, world! {}", vector.get_x());
}
