#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn string_concat(parts: &[&str]) -> Rc<str> {
    let mut joined = Rc::<[u8]>::new_uninit_slice(parts.iter().map(|part| part.len()).sum());
    let bytes = Rc::get_mut(&mut joined).unwrap().as_mut_ptr() as *mut u8;
    let mut at = 0;
    for part in parts {
        unsafe { std::ptr::copy_nonoverlapping(part.as_ptr(), bytes.add(at), part.len()) };
        at += part.len();
    }
    unsafe { Rc::from_raw(Rc::into_raw(joined.assume_init()) as *const str) }
}
#[derive(Clone)]
pub enum level {
    level_H1,
    level_H2,
    level_H3,
    level_H4,
    level_H5,
    level_H6,
}
#[derive(Clone)]
pub enum inline {
    inline_Text(Rc<str>),
    inline_Code(Rc<str>),
    inline_Strong(Rc<list_inline>),
    inline_Link(Rc<str>, Rc<str>),
    inline_Image(Rc<str>, Rc<str>),
    inline_AutoLink(Rc<str>),
    inline_LinkedImage(Rc<str>, Rc<str>, Rc<str>),
}
#[derive(Clone)]
pub enum list_inline {
    list_inline_Nil,
    list_inline_Cons(Rc<inline>, Rc<list_inline>),
}
#[derive(Clone)]
pub enum list_list_inline {
    list_list_inline_Nil,
    list_list_inline_Cons(Rc<list_inline>, Rc<list_list_inline>),
}
#[derive(Clone)]
pub enum list_string {
    list_string_Nil,
    list_string_Cons(Rc<str>, Rc<list_string>),
}
#[derive(Clone)]
pub enum align {
    align_AlignNone,
    align_AlignLeft,
    align_AlignCenter,
    align_AlignRight,
}
#[derive(Clone)]
pub enum list_align {
    list_align_Nil,
    list_align_Cons(align, Rc<list_align>),
}
#[derive(Clone)]
pub enum list_list_list_inline {
    list_list_list_inline_Nil,
    list_list_list_inline_Cons(Rc<list_list_inline>, Rc<list_list_list_inline>),
}
#[derive(Clone)]
pub enum block {
    block_Heading(level, Rc<list_inline>),
    block_Paragraph(Rc<list_inline>),
    block_Bullets(Rc<list_list_inline>),
    block_Numbered(i64, Rc<list_list_inline>),
    block_CodeBlock(Rc<str>, Rc<list_string>),
    block_Rule,
    block_Table(Rc<list_align>, Rc<list_list_inline>, Rc<list_list_list_inline>),
}
#[derive(Clone)]
pub enum list_block {
    list_block_Nil,
    list_block_Cons(block, Rc<list_block>),
}
#[derive(Clone)]
pub enum list_node {
    list_node_Nil,
    list_node_Cons(Rc<node>, Rc<list_node>),
}
#[derive(Clone)]
pub enum node {
    node_File(Rc<str>, Rc<str>, i64),
    node_Folder(Rc<str>, Rc<str>, Rc<list_node>),
}
#[derive(Clone)]
pub enum option_level {
    option_level_Some(level),
    option_level_None,
}
#[derive(Clone)]
pub enum markup {
    markup_Markup(Rc<str>),
}
#[derive(Clone)]
pub enum option_list_align {
    option_list_align_Some(Rc<list_align>),
    option_list_align_None,
}
#[derive(Clone)]
pub enum viewer {
    viewer_Direct,
    viewer_Source,
    viewer_Delimited,
    viewer_Prose,
    viewer_Picture,
    viewer_Diagram,
}
impl level {
    pub fn tag(&self) -> i32 {
        match self {
            level::level_H1 => 0,
            level::level_H2 => 1,
            level::level_H3 => 2,
            level::level_H4 => 3,
            level::level_H5 => 4,
            level::level_H6 => 5,
        }
    }
}
impl inline {
    pub fn tag(&self) -> i32 {
        match self {
            inline::inline_Text(..) => 0,
            inline::inline_Code(..) => 1,
            inline::inline_Strong(..) => 2,
            inline::inline_Link(..) => 3,
            inline::inline_Image(..) => 4,
            inline::inline_AutoLink(..) => 5,
            inline::inline_LinkedImage(..) => 6,
        }
    }
}
impl list_inline {
    pub fn tag(&self) -> i32 {
        match self {
            list_inline::list_inline_Nil => 0,
            list_inline::list_inline_Cons(..) => 1,
        }
    }
}
impl list_list_inline {
    pub fn tag(&self) -> i32 {
        match self {
            list_list_inline::list_list_inline_Nil => 0,
            list_list_inline::list_list_inline_Cons(..) => 1,
        }
    }
}
impl list_string {
    pub fn tag(&self) -> i32 {
        match self {
            list_string::list_string_Nil => 0,
            list_string::list_string_Cons(..) => 1,
        }
    }
}
impl align {
    pub fn tag(&self) -> i32 {
        match self {
            align::align_AlignNone => 0,
            align::align_AlignLeft => 1,
            align::align_AlignCenter => 2,
            align::align_AlignRight => 3,
        }
    }
}
impl list_align {
    pub fn tag(&self) -> i32 {
        match self {
            list_align::list_align_Nil => 0,
            list_align::list_align_Cons(..) => 1,
        }
    }
}
impl list_list_list_inline {
    pub fn tag(&self) -> i32 {
        match self {
            list_list_list_inline::list_list_list_inline_Nil => 0,
            list_list_list_inline::list_list_list_inline_Cons(..) => 1,
        }
    }
}
impl block {
    pub fn tag(&self) -> i32 {
        match self {
            block::block_Heading(..) => 0,
            block::block_Paragraph(..) => 1,
            block::block_Bullets(..) => 2,
            block::block_Numbered(..) => 3,
            block::block_CodeBlock(..) => 4,
            block::block_Rule => 5,
            block::block_Table(..) => 6,
        }
    }
}
impl list_block {
    pub fn tag(&self) -> i32 {
        match self {
            list_block::list_block_Nil => 0,
            list_block::list_block_Cons(..) => 1,
        }
    }
}
impl list_node {
    pub fn tag(&self) -> i32 {
        match self {
            list_node::list_node_Nil => 0,
            list_node::list_node_Cons(..) => 1,
        }
    }
}
impl node {
    pub fn tag(&self) -> i32 {
        match self {
            node::node_File(..) => 0,
            node::node_Folder(..) => 1,
        }
    }
}
impl option_level {
    pub fn tag(&self) -> i32 {
        match self {
            option_level::option_level_Some(..) => 0,
            option_level::option_level_None => 1,
        }
    }
}
impl markup {
    pub fn tag(&self) -> i32 {
        match self {
            markup::markup_Markup(..) => 0,
        }
    }
}
impl option_list_align {
    pub fn tag(&self) -> i32 {
        match self {
            option_list_align::option_list_align_Some(..) => 0,
            option_list_align::option_list_align_None => 1,
        }
    }
}
impl viewer {
    pub fn tag(&self) -> i32 {
        match self {
            viewer::viewer_Direct => 0,
            viewer::viewer_Source => 1,
            viewer::viewer_Delimited => 2,
            viewer::viewer_Prose => 3,
            viewer::viewer_Picture => 4,
            viewer::viewer_Diagram => 5,
        }
    }
}
impl Default for level {
    fn default() -> Self {
        level::level_H1
    }
}
impl Default for inline {
    fn default() -> Self {
        inline::inline_Text({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
impl Default for list_inline {
    fn default() -> Self {
        list_inline::list_inline_Nil
    }
}
impl Default for list_list_inline {
    fn default() -> Self {
        list_list_inline::list_list_inline_Nil
    }
}
impl Default for list_string {
    fn default() -> Self {
        list_string::list_string_Nil
    }
}
impl Default for align {
    fn default() -> Self {
        align::align_AlignNone
    }
}
impl Default for list_align {
    fn default() -> Self {
        list_align::list_align_Nil
    }
}
impl Default for list_list_list_inline {
    fn default() -> Self {
        list_list_list_inline::list_list_list_inline_Nil
    }
}
impl Default for block {
    fn default() -> Self {
        block::block_Heading(<level>::default(), <Rc<list_inline>>::default())
    }
}
impl Default for list_block {
    fn default() -> Self {
        list_block::list_block_Nil
    }
}
impl Default for list_node {
    fn default() -> Self {
        list_node::list_node_Nil
    }
}
impl Default for node {
    fn default() -> Self {
        node::node_File({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }, <i64>::default())
    }
}
impl Default for option_level {
    fn default() -> Self {
        option_level::option_level_Some(<level>::default())
    }
}
impl Default for markup {
    fn default() -> Self {
        markup::markup_Markup({ thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) })
    }
}
impl Default for option_list_align {
    fn default() -> Self {
        option_list_align::option_list_align_Some(<Rc<list_align>>::default())
    }
}
impl Default for viewer {
    fn default() -> Self {
        viewer::viewer_Direct
    }
}
fn closure0() -> Rc<dyn Fn(block) -> Rc<list_block>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(block) -> Rc<list_block>> = Rc::new(move |mut v0: block| -> Rc<list_block> {
        let mut v1: Rc<list_block> = { thread_local!{ static CASE: Rc<list_block> = Rc::new(list_block::list_block_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(list_block::list_block_Cons(v0, v1))
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure1() -> Rc<dyn Fn(Rc<node>) -> Rc<list_node>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<node>) -> Rc<list_node>> = Rc::new(move |mut v0: Rc<node>| -> Rc<list_node> {
        let mut v1: Rc<list_node> = { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) };
        Rc::new(list_node::list_node_Cons(v0, v1))
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure2() -> Rc<dyn Fn(i32) -> option_level> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i32) -> option_level> = Rc::new(move |mut v0: i32| -> option_level {
        let mut v1: bool = v0 == 1i32;
        if v1 {
            let v2: level = level::level_H1;
            option_level::option_level_Some(v2)
        } else {
            let v4: bool = v0 == 2i32;
            if v4 {
                let v5: level = level::level_H2;
                option_level::option_level_Some(v5)
            } else {
                let v7: bool = v0 == 3i32;
                if v7 {
                    let v8: level = level::level_H3;
                    option_level::option_level_Some(v8)
                } else {
                    let v10: bool = v0 == 4i32;
                    if v10 {
                        let v11: level = level::level_H4;
                        option_level::option_level_Some(v11)
                    } else {
                        let v13: bool = v0 == 5i32;
                        if v13 {
                            let v14: level = level::level_H5;
                            option_level::option_level_Some(v14)
                        } else {
                            let v16: bool = v0 == 6i32;
                            if v16 {
                                let v17: level = level::level_H6;
                                option_level::option_level_Some(v17)
                            } else {
                                option_level::option_level_None
                            }
                        }
                    }
                }
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn level_tag_0(mut v0: level) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("H1"); } LIT.with(|lit| lit.clone()) };
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("H2"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("H3"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("H4"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("H5"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("H6"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<RefCell<Vec<Rc<str>>>> = Rc::new(RefCell::new(vec![v1, v2, v3, v4, v5, v6]));
    let v8: i32 = (v0.tag() as i32);
    let v9: Rc<str> = v7.borrow()[v8 as usize].clone();
    let v10: Rc<str> = Rc::<str>::from(v9.to_lowercase());
    v10
}
fn closure3() -> Rc<dyn Fn(level) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(level) -> Rc<str>> = Rc::new(move |mut v0: level| -> Rc<str> {
        level_tag_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn run_of_3(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let v3: bool = v2 <= v1;
    if v3 {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: i32 = v2.wrapping_sub(1i32);
        let v6: Rc<str> = string_slice(&v0, v1 as i64, v5 as i64);
        v6
    }
}
fn entity_5(mut v0: u8) -> Rc<str> {
    let mut v1: bool = v0 == b'&';
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&amp;"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: bool = v0 == b'<';
        if v3 {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&lt;"); } LIT.with(|lit| lit.clone()) };
            v4
        } else {
            let v5: bool = v0 == b'>';
            if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&gt;"); } LIT.with(|lit| lit.clone()) };
                v6
            } else {
                let v7: bool = v0 == b'"';
                if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&quot;"); } LIT.with(|lit| lit.clone()) };
                    v8
                } else {
                    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    v9
                }
            }
        }
    }
}
fn escape_step_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> (i32, Rc<str>) {
    let v4: u8 = v0.clone().as_bytes()[v1 as usize];
    let v5: Rc<str> = entity_5(v4);
    let v6: bool = &*v5 == "";
    if v6 {
        (v2, v3)
    } else {
        let v7: i32 = v1.wrapping_add(1i32);
        let v8: Rc<str> = run_of_3(v0, v2, v1);
        let v10: Rc<str> = string_concat(&[&*v3, &*v8, &*v5]);
        (v7, v10)
    }
}
fn escape_from_2(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = run_of_3(v0, v2, v1);
            let v7: Rc<str> = string_concat(&[&*v3, &*v6]);
            return v7;
        } else {
            let (mut v8, mut v9): (i32, Rc<str>) = escape_step_4(v0.clone(), v1, v2, v3);
            let v10: i32 = v1.wrapping_add(1i32);
            (v1, v2, v3) = (v10, v8, v9);
            continue;
        }
    }
}
fn escape_1(mut v0: Rc<str>) -> markup {
    let mut v1: i32 = 0i32;
    let v2: i32 = 0i32;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = escape_from_2(v0, v1, v2, v3);
    markup::markup_Markup(v4)
}
fn closure4() -> Rc<dyn Fn(Rc<str>) -> markup> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> markup> = Rc::new(move |mut v0: Rc<str>| -> markup {
        escape_1(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn spaces_from_7(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 < v2;
        let v6: bool = if v3 {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b' ';
            v5
        } else {
            false
        };
        if v6 {
            let v7: i32 = v1.wrapping_add(1i32);
            v1 = v7;
            continue;
        } else {
            return v1;
        }
    }
}
fn trail_from_8(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: bool = v1 > 0i32;
        let v6: bool = if v2 {
            let v3: i32 = v1.wrapping_sub(1i32);
            let v4: u8 = v0.clone().as_bytes()[v3 as usize];
            let v5: bool = v4 == b' ';
            v5
        } else {
            false
        };
        if v6 {
            let v7: i32 = v1.wrapping_sub(1i32);
            v1 = v7;
            continue;
        } else {
            return v1;
        }
    }
}
fn trim_6(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let v2: i32 = spaces_from_7(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = trail_from_8(v0.clone(), v3);
    run_of_3(v0, v2, v4)
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        trim_6(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure6() -> Rc<dyn Fn(Rc<list_align>) -> option_list_align> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<list_align>) -> option_list_align> = Rc::new(move |mut v0: Rc<list_align>| -> option_list_align {
        option_list_align::option_list_align_Some(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(viewer) -> viewer> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(viewer) -> viewer> = Rc::new(move |mut v0: viewer| -> viewer {
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_model_blocks_of(v0: block) -> Rc<list_block> {
    closure0()(v0)
}
pub fn html_model_nodes_of(v0: Rc<node>) -> Rc<list_node> {
    closure1()(v0)
}
pub fn html_model_level_of(v0: i32) -> option_level {
    closure2()(v0)
}
pub fn html_model_level_tag(v0: level) -> Rc<str> {
    closure3()(v0)
}
pub fn html_model_escape(v0: &str) -> markup {
    closure4()(Rc::<str>::from(v0))
}
pub fn html_model_trim(v0: &str) -> Rc<str> {
    closure5()(Rc::<str>::from(v0))
}
pub fn html_model_aligns_some(v0: Rc<list_align>) -> option_list_align {
    closure6()(v0)
}
pub fn html_model_viewer_same(v0: viewer) -> viewer {
    closure7()(v0)
}
