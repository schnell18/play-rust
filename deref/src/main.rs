use std::fmt::Display;
use std::ops::{Deref, DerefMut};

fn main() {
    let s = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 2
    };
    // This does not compile as Selector<&str> does not implement std::fmt::Display
    // show_it_generic(&s);
    show_it_generic(&s as &str);
}

fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}
struct Selector<T> {
    /// Elements available in this `selector`.
    elements: Vec<T>,

    /// The index of the "current" element in `elements`.
    /// A `Selector` behaves like a pointer to the current element.
    current: usize
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_selector() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2
    };

    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(*s, 'w');
    assert_eq!(s.elements, ['x', 'y', 'w']);
}