#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let v1: Rc<str> = html_pages::html_pages_md(&v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_markdown(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
