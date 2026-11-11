#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{list_string, viewer, node, list_node, markup};
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
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<str>, Rc<str>, Rc<UH3>),
}
#[derive(Clone)]
enum US2 {
    US2_Some(markup),
    US2_None,
}
#[derive(Clone)]
enum UH4 {
    UH4_IndexPage(markup),
}
fn by_extension_1(mut v0: Rc<str>) -> viewer {
    let mut v1: bool = &*v0 == "tsv";
    if v1 {
        viewer::viewer_Delimited
    } else {
        let v3: bool = &*v0 == "md";
        if v3 {
            viewer::viewer_Prose
        } else {
            let v5: bool = &*v0 == "mmd";
            if v5 {
                viewer::viewer_Diagram
            } else {
                viewer::viewer_Direct
            }
        }
    }
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
fn last_dot_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'.';
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                (v1, v2) = (v7, v1);
                continue;
            } else {
                let v9: i32 = v1.wrapping_add(1i32);
                v1 = v9;
                continue;
            }
        }
    }
}
fn extension_2(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = -1i32;
    let v3: i32 = last_dot_4(v0.clone(), v1, v2);
    let v4: bool = v3 <= 0i32;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let v6: i32 = v3.wrapping_add(1i32);
        let v7: i32 = (v0.clone().len() as i32);
        run_of_3(v0, v6, v7)
    }
}
fn listed_5(mut v0: Rc<str>, mut v1: Rc<list_string>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v1) {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<list_string> = v3;
                let v4: bool = &*v2 == &*v0;
                if v4 {
                    return true;
                } else {
                    v1 = v3;
                    continue;
                }
            }
            list_string::list_string_Nil => {
                return false;
            }
        }
    }
}
fn viewer_of_0(mut v0: Rc<str>) -> viewer {
    let mut v1: Rc<str> = extension_2(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("png"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("svg"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jpg"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("jpeg"); } LIT.with(|lit| lit.clone()) };
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gif"); } LIT.with(|lit| lit.clone()) };
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("webp"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
    let v9: Rc<list_string> = Rc::new(list_string::list_string_Cons(v7, v8));
    let v10: Rc<list_string> = Rc::new(list_string::list_string_Cons(v6, v9));
    let v11: Rc<list_string> = Rc::new(list_string::list_string_Cons(v5, v10));
    let v12: Rc<list_string> = Rc::new(list_string::list_string_Cons(v4, v11));
    let v13: Rc<list_string> = Rc::new(list_string::list_string_Cons(v3, v12));
    let v14: Rc<list_string> = Rc::new(list_string::list_string_Cons(v2, v13));
    let v15: bool = listed_5(v1.clone(), v14);
    if v15 {
        viewer::viewer_Picture
    } else {
        let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi"); } LIT.with(|lit| lit.clone()) };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spir"); } LIT.with(|lit| lit.clone()) };
        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiproj"); } LIT.with(|lit| lit.clone()) };
        let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toml"); } LIT.with(|lit| lit.clone()) };
        let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps1"); } LIT.with(|lit| lit.clone()) };
        let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("psm1"); } LIT.with(|lit| lit.clone()) };
        let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs"); } LIT.with(|lit| lit.clone()) };
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsx"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsproj"); } LIT.with(|lit| lit.clone()) };
        let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rs"); } LIT.with(|lit| lit.clone()) };
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
        let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ex"); } LIT.with(|lit| lit.clone()) };
        let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exs"); } LIT.with(|lit| lit.clone()) };
        let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
        let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
        let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cpp"); } LIT.with(|lit| lit.clone()) };
        let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cs"); } LIT.with(|lit| lit.clone()) };
        let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("py"); } LIT.with(|lit| lit.clone()) };
        let v36: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh"); } LIT.with(|lit| lit.clone()) };
        let v37: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
        let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bat"); } LIT.with(|lit| lit.clone()) };
        let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lock"); } LIT.with(|lit| lit.clone()) };
        let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cfg"); } LIT.with(|lit| lit.clone()) };
        let v41: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ini"); } LIT.with(|lit| lit.clone()) };
        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sql"); } LIT.with(|lit| lit.clone()) };
        let v43: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("csv"); } LIT.with(|lit| lit.clone()) };
        let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dib"); } LIT.with(|lit| lit.clone()) };
        let v45: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("livemd"); } LIT.with(|lit| lit.clone()) };
        let v46: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
        let v47: Rc<list_string> = Rc::new(list_string::list_string_Cons(v45, v46));
        let v48: Rc<list_string> = Rc::new(list_string::list_string_Cons(v44, v47));
        let v49: Rc<list_string> = Rc::new(list_string::list_string_Cons(v43, v48));
        let v50: Rc<list_string> = Rc::new(list_string::list_string_Cons(v42, v49));
        let v51: Rc<list_string> = Rc::new(list_string::list_string_Cons(v41, v50));
        let v52: Rc<list_string> = Rc::new(list_string::list_string_Cons(v40, v51));
        let v53: Rc<list_string> = Rc::new(list_string::list_string_Cons(v39, v52));
        let v54: Rc<list_string> = Rc::new(list_string::list_string_Cons(v38, v53));
        let v55: Rc<list_string> = Rc::new(list_string::list_string_Cons(v37, v54));
        let v56: Rc<list_string> = Rc::new(list_string::list_string_Cons(v36, v55));
        let v57: Rc<list_string> = Rc::new(list_string::list_string_Cons(v35, v56));
        let v58: Rc<list_string> = Rc::new(list_string::list_string_Cons(v34, v57));
        let v59: Rc<list_string> = Rc::new(list_string::list_string_Cons(v33, v58));
        let v60: Rc<list_string> = Rc::new(list_string::list_string_Cons(v32, v59));
        let v61: Rc<list_string> = Rc::new(list_string::list_string_Cons(v31, v60));
        let v62: Rc<list_string> = Rc::new(list_string::list_string_Cons(v30, v61));
        let v63: Rc<list_string> = Rc::new(list_string::list_string_Cons(v29, v62));
        let v64: Rc<list_string> = Rc::new(list_string::list_string_Cons(v28, v63));
        let v65: Rc<list_string> = Rc::new(list_string::list_string_Cons(v27, v64));
        let v66: Rc<list_string> = Rc::new(list_string::list_string_Cons(v26, v65));
        let v67: Rc<list_string> = Rc::new(list_string::list_string_Cons(v25, v66));
        let v68: Rc<list_string> = Rc::new(list_string::list_string_Cons(v24, v67));
        let v69: Rc<list_string> = Rc::new(list_string::list_string_Cons(v23, v68));
        let v70: Rc<list_string> = Rc::new(list_string::list_string_Cons(v22, v69));
        let v71: Rc<list_string> = Rc::new(list_string::list_string_Cons(v21, v70));
        let v72: Rc<list_string> = Rc::new(list_string::list_string_Cons(v20, v71));
        let v73: Rc<list_string> = Rc::new(list_string::list_string_Cons(v19, v72));
        let v74: Rc<list_string> = Rc::new(list_string::list_string_Cons(v18, v73));
        let v75: Rc<list_string> = Rc::new(list_string::list_string_Cons(v17, v74));
        let v76: bool = listed_5(v1.clone(), v75);
        if v76 {
            viewer::viewer_Source
        } else {
            by_extension_1(v1)
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> viewer> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> viewer> = Rc::new(move |mut v0: Rc<str>| -> viewer {
        viewer_of_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn folder_rank_10(mut v0: &Rc<node>) -> i32 {
    match &**v0 {
        node::node_File(v4, v5, v6) => {
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: i64 = *v6;
            1i32
        }
        node::node_Folder(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<list_node> = v3.clone();
            0i32
        }
    }
}
fn node_name_11(mut v0: &Rc<node>) -> Rc<str> {
    match &**v0 {
        node::node_File(v4, v5, v6) => {
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: i64 = *v6;
            v4
        }
        node::node_Folder(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let v3: Rc<list_node> = v3.clone();
            v1
        }
    }
}
fn length_order_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let v3: i32 = (v0.len() as i32);
    let v4: bool = v2 == v3;
    let v7: bool = if v4 {
        let v5: i32 = (v1.len() as i32);
        let v6: bool = v2 == v5;
        v6
    } else {
        false
    };
    if v7 {
        0i32
    } else {
        if v4 {
            -1i32
        } else {
            1i32
        }
    }
}
fn char_order_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    let v3: u8 = v0.as_bytes()[v2 as usize];
    let v4: u32 = u32::from(v3);
    let v5: u8 = v1.as_bytes()[v2 as usize];
    let v6: u32 = u32::from(v5);
    let v7: bool = v4 < v6;
    if v7 {
        -1i32
    } else {
        1i32
    }
}
fn compare_from_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v2 == v3;
        let v7: bool = if v4 {
            true
        } else {
            let v5: i32 = (v1.clone().len() as i32);
            let v6: bool = v2 == v5;
            v6
        };
        if v7 {
            return length_order_13(v0, v1, v2);
        } else {
            let v9: u8 = v0.clone().as_bytes()[v2 as usize];
            let v10: u8 = v1.clone().as_bytes()[v2 as usize];
            let v11: bool = v9 == v10;
            if v11 {
                let v12: i32 = v2.wrapping_add(1i32);
                v2 = v12;
                continue;
            } else {
                return char_order_14(v0, v1, v2);
            }
        }
    }
}
fn before_9(mut v0: &Rc<node>, mut v1: &Rc<node>) -> bool {
    let mut v2: i32 = folder_rank_10(v0);
    let v3: i32 = folder_rank_10(v1);
    let v4: bool = v2 == v3;
    if v4 {
        let v5: Rc<str> = node_name_11(v0);
        let v6: Rc<str> = node_name_11(v1);
        let v7: i32 = 0i32;
        let v8: i32 = compare_from_12(v5, v6, v7);
        let v9: bool = v8 < 0i32;
        v9
    } else {
        let v10: i32 = folder_rank_10(v0);
        let v11: i32 = folder_rank_10(v1);
        let v12: bool = v10 < v11;
        v12
    }
}
fn insert_by_8(mut v0: Rc<node>, mut v1: Rc<list_node>) -> Rc<list_node> {
    match &*v1 {
        list_node::list_node_Cons(v2, v3) => {
            let mut v2: Rc<node> = v2.clone();
            let v3: Rc<list_node> = v3.clone();
            let v4: bool = before_9(&v0, &v2);
            if v4 {
                Rc::new(list_node::list_node_Cons(v0, v1))
            } else {
                let v6: Rc<list_node> = insert_by_8(v0, v3);
                Rc::new(list_node::list_node_Cons(v2, v6))
            }
        }
        list_node::list_node_Nil => {
            let v9: Rc<list_node> = { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(list_node::list_node_Cons(v0, v9))
        }
    }
}
fn sort_by_7(mut v0: Rc<list_node>) -> Rc<list_node> {
    match Rc::unwrap_or_clone(v0) {
        list_node::list_node_Cons(v1, v2) => {
            let mut v1: Rc<node> = v1;
            let mut v2: Rc<list_node> = v2;
            let v3: Rc<list_node> = sort_by_7(v2);
            insert_by_8(v1, v3)
        }
        list_node::list_node_Nil => {
            { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn sort_nodes_6(mut v0: Rc<list_node>) -> Rc<list_node> {
    sort_by_7(v0)
}
fn closure1() -> Rc<dyn Fn(Rc<list_node>) -> Rc<list_node>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<list_node>) -> Rc<list_node>> = Rc::new(move |mut v0: Rc<list_node>| -> Rc<list_node> {
        sort_nodes_6(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn join_rel_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v0 == "";
    if v2 {
        v1
    } else {
        let v4: Rc<str> = string_concat(&[&*v0, "/", &*v1]);
        v4
    }
}
fn closure2() -> Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: Rc<str>| -> Rc<str> {
        join_rel_15(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn markup_text_18(mut v0: markup) -> Rc<str> {
    match v0 {
        markup::markup_Markup(v1) => {
            let mut v1: Rc<str> = v1;
            v1
        }
    }
}
fn joined_17(mut v0: markup, mut v1: markup) -> markup {
    let mut v2: Rc<str> = markup_text_18(v0);
    let v3: Rc<str> = markup_text_18(v1);
    let v4: Rc<str> = string_concat(&[&*v2, &*v3]);
    markup::markup_Markup(v4)
}
fn trusted_19(mut v0: Rc<str>) -> markup {
    markup::markup_Markup(v0)
}
fn closure4(mut v0: bool) -> Rc<dyn Fn(Rc<list_node>) -> markup> {
    Rc::new(move |mut v1: Rc<list_node>| -> markup {
        nodes_html_16(v0, v1)
    })
}
fn attribute_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v4: markup = html_model::html_model_escape(&v0);
    let v5: Rc<str> = markup_text_18(v4);
    let v7: Rc<str> = string_concat(&[" ", &*v1, "=\"", &*v5, "\""]);
    v7
}
fn attributes_23(mut v0: Rc<UH3>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH3::UH3_Cons(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: Rc<UH3> = v3;
            let v4: Rc<str> = attribute_24(v2, v1);
            let v5: Rc<str> = attributes_23(v3);
            let v6: Rc<str> = string_concat(&[&*v4, &*v5]);
            v6
        }
        UH3::UH3_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
    }
}
fn element_22(mut v0: Rc<str>, mut v1: Rc<UH3>, mut v2: markup) -> markup {
    let v4: Rc<str> = attributes_23(v1);
    let v7: Rc<str> = markup_text_18(v2);
    let v11: Rc<str> = string_concat(&["<", &*v0, &*v4, ">", &*v7, "</", &*v0, ">"]);
    markup::markup_Markup(v11)
}
fn wrap_21(mut v0: Rc<str>, mut v1: markup) -> markup {
    let mut v2: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    element_22(v0, v2, v1)
}
fn anchor_27(mut v0: Rc<str>, mut v1: markup) -> markup {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("href"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH3> = Rc::new(UH3::UH3_Cons(v3, v0, v4));
    element_22(v2, v5, v1)
}
fn digit_31(mut v0: i64) -> Rc<str> {
    let mut v1: i32 = (v0 as i32);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = v1.wrapping_add(1i32);
    run_of_3(v2, v1, v3)
}
fn show_i64_30(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 10i64;
    if v1 {
        digit_31(v0)
    } else {
        let v3: i64 = v0.wrapping_div(10i64);
        let v4: Rc<str> = show_i64_30(v3);
        let v5: i64 = v0.wrapping_rem(10i64);
        let v6: Rc<str> = digit_31(v5);
        let v7: Rc<str> = string_concat(&[&*v4, &*v6]);
        v7
    }
}
fn pad2_32(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 10i64;
    if v1 {
        let mut v2: Rc<str> = show_i64_30(v0);
        let v3: Rc<str> = string_concat(&["0", &*v2]);
        v3
    } else {
        show_i64_30(v0)
    }
}
fn hundredths_29(mut v0: i64, mut v1: i64) -> Rc<str> {
    let mut v2: i64 = v0.wrapping_mul(100i64);
    let v3: i64 = v1.wrapping_div(2i64);
    let v4: i64 = v2.wrapping_add(v3);
    let v5: i64 = v4.wrapping_div(v1);
    let v6: i64 = v5.wrapping_div(100i64);
    let v7: Rc<str> = show_i64_30(v6);
    let v9: i64 = v5.wrapping_rem(100i64);
    let v10: Rc<str> = pad2_32(v9);
    let v11: Rc<str> = string_concat(&[&*v7, ".", &*v10]);
    v11
}
fn bytes_label_28(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 > 1048576i64;
    if v1 {
        let mut v2: i64 = 1048576i64;
        let v3: Rc<str> = hundredths_29(v0, v2);
        let v4: Rc<str> = string_concat(&[&*v3, " MB"]);
        v4
    } else {
        let v5: bool = v0 > 1024i64;
        if v5 {
            let v6: i64 = 1024i64;
            let v7: Rc<str> = hundredths_29(v0, v6);
            let v8: Rc<str> = string_concat(&[&*v7, " KB"]);
            v8
        } else {
            let v9: i64 = 1i64;
            let v10: Rc<str> = hundredths_29(v0, v9);
            let v11: Rc<str> = string_concat(&[&*v10, " B"]);
            v11
        }
    }
}
fn entry_link_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i64) -> markup {
    let v4: Rc<str> = string_concat(&[&*v0, " "]);
    let v5: markup = trusted_19(v4);
    let v6: markup = html_model::html_model_escape(&v2);
    let v7: markup = anchor_27(v1, v6);
    let v8: markup = joined_17(v5, v7);
    let v9: Rc<str> = bytes_label_28(v3);
    let v11: Rc<str> = string_concat(&["<span> (", &*v9, ")</span>"]);
    let v12: markup = trusted_19(v11);
    joined_17(v8, v12)
}
fn folder_html_25(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i64, mut v3: markup) -> markup {
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("summary"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128194;"); } LIT.with(|lit| lit.clone()) };
    let v6: markup = entry_link_26(v5, v0, v1, v2);
    let v7: markup = wrap_21(v4, v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\">"); } LIT.with(|lit| lit.clone()) };
    let v9: markup = trusted_19(v8);
    let v10: markup = joined_17(v9, v7);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("div"); } LIT.with(|lit| lit.clone()) };
    let v12: markup = wrap_21(v11, v3);
    let v13: markup = joined_17(v10, v12);
    let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</details>"); } LIT.with(|lit| lit.clone()) };
    let v15: markup = trusted_19(v14);
    joined_17(v13, v15)
}
fn has_viewer_34(mut v0: Rc<str>) -> bool {
    let mut v1: viewer = viewer_of_0(v0);
    match v1 {
        viewer::viewer_Direct => {
            false
        }
        _ => {
            true
        }
    }
}
fn raw_link_35(mut v0: Rc<str>) -> markup {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let mut v2: markup = trusted_19(v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<span>raw</span>"); } LIT.with(|lit| lit.clone()) };
    let v4: markup = trusted_19(v3);
    let v5: markup = anchor_27(v0, v4);
    joined_17(v2, v5)
}
fn file_html_33(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i64) -> markup {
    let v5: bool = if v0 {
        has_viewer_34(v1.clone())
    } else {
        false
    };
    if v5 {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196;"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = string_concat(&[&*v2, ".html"]);
        let v8: markup = entry_link_26(v6, v7, v1, v3);
        let v9: markup = raw_link_35(v2);
        joined_17(v8, v9)
    } else {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196;"); } LIT.with(|lit| lit.clone()) };
        entry_link_26(v11, v2, v1, v3)
    }
}
fn node_bytes_37(mut v0: &Rc<node>) -> i64 {
    match &**v0 {
        node::node_File(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let v3: i64 = *v3;
            v3
        }
        node::node_Folder(v4, v5, v6) => {
            let v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let v6: Rc<list_node> = v6.clone();
            nodes_bytes_36(v6)
        }
    }
}
fn nodes_bytes_36(mut v0: Rc<list_node>) -> i64 {
    match Rc::unwrap_or_clone(v0) {
        list_node::list_node_Cons(v1, v2) => {
            let mut v1: Rc<node> = v1;
            let mut v2: Rc<list_node> = v2;
            let v3: i64 = node_bytes_37(&v1);
            let v4: i64 = nodes_bytes_36(v2);
            let v5: i64 = v3.wrapping_add(v4);
            v5
        }
        list_node::list_node_Nil => {
            0i64
        }
    }
}
fn node_by_20(mut v0: Rc<dyn Fn(Rc<list_node>) -> markup>, mut v1: bool, mut v2: &Rc<node>) -> markup {
    match &**v2 {
        node::node_File(v3, v4, v5) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: i64 = *v5;
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("div"); } LIT.with(|lit| lit.clone()) };
            let v7: markup = file_html_33(v1, v3, v4, v5);
            wrap_21(v6, v7)
        }
        node::node_Folder(v9, v10, v11) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = v10.clone();
            let v11: Rc<list_node> = v11.clone();
            let v12: i64 = nodes_bytes_36(v11.clone());
            let v13: markup = v0(v11);
            folder_html_25(v10, v9, v12, v13)
        }
    }
}
fn nodes_html_16(mut v0: bool, mut v1: Rc<list_node>) -> markup {
    match Rc::unwrap_or_clone(v1) {
        list_node::list_node_Cons(v2, v3) => {
            let mut v2: Rc<node> = v2;
            let v3: Rc<list_node> = v3;
            let v4: Rc<dyn Fn(Rc<list_node>) -> markup> = closure4(v0);
            let v5: markup = node_by_20(v4, v0, &v2);
            let v6: markup = nodes_html_16(v0, v3);
            joined_17(v5, v6)
        }
        list_node::list_node_Nil => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_19(v8)
        }
    }
}
fn closure3() -> Rc<dyn Fn(bool, Rc<list_node>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(bool, Rc<list_node>) -> Rc<str>> = Rc::new(move |mut v0: bool, mut v1: Rc<list_node>| -> Rc<str> {
        let mut v2: markup = nodes_html_16(v0, v1);
        markup_text_18(v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn head_html_39() -> markup {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}"); } LIT.with(|lit| lit.clone()) };
    let mut v1: markup = trusted_19(v0);
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
    let v3: markup = trusted_19(v2);
    joined_17(v1, v3)
}
fn readme_section_40(mut v0: US2) -> markup {
    match v0 {
        US2::US2_None => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_19(v8)
        }
        US2::US2_Some(v1) => {
            let mut v1: markup = v1;
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
            let v3: markup = trusted_19(v2);
            let v4: markup = joined_17(v3, v1);
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
            let v6: markup = trusted_19(v5);
            joined_17(v4, v6)
        }
    }
}
fn close_html_41(mut v0: markup) -> markup {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: markup = trusted_19(v1);
    let v3: markup = joined_17(v2, v0);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
    let v5: markup = trusted_19(v4);
    joined_17(v3, v5)
}
fn index_page_of_38(mut v0: bool, mut v1: Rc<list_node>, mut v2: US2) -> Rc<UH4> {
    let v3: markup = head_html_39();
    let v4: markup = nodes_html_16(v0, v1);
    let v5: markup = joined_17(v3, v4);
    let v6: markup = readme_section_40(v2);
    let v7: markup = close_html_41(v6);
    let v8: markup = joined_17(v5, v7);
    Rc::new(UH4::UH4_IndexPage(v8))
}
fn closure5() -> Rc<dyn Fn(bool, Rc<list_node>, bool, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(bool, Rc<list_node>, bool, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: bool, mut v1: Rc<list_node>, mut v2: bool, mut v3: Rc<str>| -> Rc<str> {
        let v7: US2 = if v2 {
            let v4: markup = html_render::html_render_markdown(&v3);
            US2::US2_Some(v4)
        } else {
            US2::US2_None
        };
        let v8: Rc<UH4> = index_page_of_38(v0, v1, v7);
        match Rc::unwrap_or_clone(v8) {
            UH4::UH4_IndexPage(v9) => {
                let v9: markup = v9;
                markup_text_18(v9)
            }
        }
    }); } CLOSURE.with(|closure| closure.clone())
}
fn title_html_42(mut v0: Rc<str>) -> markup {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) };
    let mut v2: markup = trusted_19(v1);
    let v3: markup = html_model::html_model_escape(&v0);
    let v4: markup = anchor_27(v0, v3);
    let v5: markup = joined_17(v2, v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <span>raw</span></p>\n"); } LIT.with(|lit| lit.clone()) };
    let v7: markup = trusted_19(v6);
    joined_17(v5, v7)
}
fn method44(mut v0: Rc<list_string>, mut v1: Rc<list_string>) -> Rc<list_string> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<list_string> = v3;
                let v4: Rc<list_string> = Rc::new(list_string::list_string_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            list_string::list_string_Nil => {
                return v1;
            }
        }
    }
}
fn lines_from_43(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<list_string>) -> Rc<list_string> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = run_of_3(v0, v2, v1);
            let v7: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<list_string> = Rc::new(list_string::list_string_Cons(v6, v7));
            return method44(v3, v8);
        } else {
            let v10: u8 = v0.clone().as_bytes()[v1 as usize];
            let v11: bool = v10 == b'\n';
            if v11 {
                let v12: i32 = v1.wrapping_add(1i32);
                let v13: Rc<str> = run_of_3(v0.clone(), v2, v1);
                let v14: Rc<list_string> = Rc::new(list_string::list_string_Cons(v13, v3));
                (v1, v2, v3) = (v12, v12, v14);
                continue;
            } else {
                let v16: i32 = v1.wrapping_add(1i32);
                v1 = v16;
                continue;
            }
        }
    }
}
fn cell_of_47(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> markup {
    let v4: Rc<str> = run_of_3(v1, v2, v3);
    let v5: markup = html_model::html_model_escape(&v4);
    wrap_21(v0, v5)
}
fn tab_cells_46(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> markup {
    loop {
        let v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            return cell_of_47(v0, v1, v3, v2);
        } else {
            let v7: u8 = v1.clone().as_bytes()[v2 as usize];
            let v8: bool = v7 == b'\t';
            if v8 {
                let v9: markup = cell_of_47(v0.clone(), v1.clone(), v3, v2);
                let v10: i32 = v2.wrapping_add(1i32);
                let v11: markup = tab_cells_46(v0, v1, v10, v10);
                return joined_17(v9, v11);
            } else {
                let v13: i32 = v2.wrapping_add(1i32);
                v2 = v13;
                continue;
            }
        }
    }
}
fn tab_rows_45(mut v0: Rc<list_string>, mut v1: Rc<str>) -> markup {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<list_string> = v3;
                let v4: bool = &*v2 == "";
                if v4 {
                    v0 = v3;
                    continue;
                } else {
                    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<tr>"); } LIT.with(|lit| lit.clone()) };
                    let v7: markup = trusted_19(v6);
                    let v8: i32 = 0i32;
                    let v9: i32 = 0i32;
                    let v10: markup = tab_cells_46(v1, v2, v8, v9);
                    let v11: markup = joined_17(v7, v10);
                    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tr>\n"); } LIT.with(|lit| lit.clone()) };
                    let v13: markup = trusted_19(v12);
                    let v14: markup = joined_17(v11, v13);
                    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
                    let v16: markup = tab_rows_45(v3, v15);
                    return joined_17(v14, v16);
                }
            }
            list_string::list_string_Nil => {
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return trusted_19(v19);
            }
        }
    }
}
fn closure6() -> Rc<dyn Fn(Rc<str>, viewer, Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>, viewer, Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>, mut v1: viewer, mut v2: Rc<str>| -> Rc<str> {
        let v3: markup = head_html_39();
        let v4: markup = title_html_42(v0.clone());
        let v5: markup = joined_17(v3, v4);
        let v82: markup = match v1 {
            viewer::viewer_Delimited => {
                let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n"); } LIT.with(|lit| lit.clone()) };
                let v14: markup = trusted_19(v13);
                let v15: i32 = 0i32;
                let v16: i32 = 0i32;
                let v17: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
                let v18: Rc<list_string> = lines_from_43(v2, v15, v16, v17);
                let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
                let v20: markup = tab_rows_45(v18, v19);
                let v21: markup = joined_17(v14, v20);
                let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</table>\n"); } LIT.with(|lit| lit.clone()) };
                let v23: markup = trusted_19(v22);
                joined_17(v21, v23)
            }
            viewer::viewer_Diagram => {
                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("pre"); } LIT.with(|lit| lit.clone()) };
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("class"); } LIT.with(|lit| lit.clone()) };
                let v57: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("mermaid"); } LIT.with(|lit| lit.clone()) };
                let v58: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v59: Rc<UH3> = Rc::new(UH3::UH3_Cons(v56, v57, v58));
                let v60: markup = html_model::html_model_escape(&v2);
                let v61: markup = element_22(v55, v59, v60);
                let v62: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v63: markup = trusted_19(v62.clone());
                let v64: markup = joined_17(v61, v63);
                let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("script"); } LIT.with(|lit| lit.clone()) };
                let v66: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("type"); } LIT.with(|lit| lit.clone()) };
                let v67: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("module"); } LIT.with(|lit| lit.clone()) };
                let v68: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v69: Rc<UH3> = Rc::new(UH3::UH3_Cons(v66, v67, v68));
                let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs'; mermaid.initialize({ startOnLoad: true, maxEdges: 5000, maxTextSize: 2000000 });"); } LIT.with(|lit| lit.clone()) };
                let v71: markup = trusted_19(v70);
                let v72: markup = element_22(v65, v69, v71);
                let v73: markup = trusted_19(v62);
                let v74: markup = joined_17(v72, v73);
                joined_17(v64, v74)
            }
            viewer::viewer_Picture => {
                let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("src"); } LIT.with(|lit| lit.clone()) };
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("alt"); } LIT.with(|lit| lit.clone()) };
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("style"); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("max-width:100%"); } LIT.with(|lit| lit.clone()) };
                let v36: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v37: Rc<UH3> = Rc::new(UH3::UH3_Cons(v34, v35, v36));
                let v38: Rc<UH3> = Rc::new(UH3::UH3_Cons(v33, v0.clone(), v37));
                let v39: Rc<UH3> = Rc::new(UH3::UH3_Cons(v32, v0.clone(), v38));
                let v40: Rc<str> = attributes_23(v39);
                let v42: Rc<str> = string_concat(&["<img", &*v40, ">"]);
                let v43: markup = trusted_19(v42);
                let v44: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("p"); } LIT.with(|lit| lit.clone()) };
                let v45: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a"); } LIT.with(|lit| lit.clone()) };
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("href"); } LIT.with(|lit| lit.clone()) };
                let v48: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
                let v49: Rc<UH3> = Rc::new(UH3::UH3_Cons(v47, v0, v48));
                let v50: markup = element_22(v46, v49, v43);
                let v51: markup = element_22(v44, v45, v50);
                let v52: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
                let v53: markup = trusted_19(v52);
                joined_17(v51, v53)
            }
            viewer::viewer_Prose => {
                let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
                let v26: markup = trusted_19(v25);
                let v27: markup = html_render::html_render_markdown(&v2);
                let v28: markup = joined_17(v26, v27);
                let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
                let v30: markup = trusted_19(v29);
                joined_17(v28, v30)
            }
            viewer::viewer_Source => {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
                let v7: markup = trusted_19(v6);
                let v8: markup = html_model::html_model_escape(&v2);
                let v9: markup = joined_17(v7, v8);
                let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
                let v11: markup = trusted_19(v10);
                joined_17(v9, v11)
            }
            _ => {
                let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                trusted_19(v76)
            }
        };
        let v83: markup = joined_17(v5, v82);
        let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        let v85: markup = trusted_19(v84);
        let v86: markup = close_html_41(v85);
        let v87: markup = joined_17(v83, v86);
        markup_text_18(v87)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn() -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn() -> Rc<str>> = Rc::new(move || -> Rc<str> {
        let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details><div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
        v0
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure8() -> Rc<dyn Fn(i64) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(i64) -> Rc<str>> = Rc::new(move |mut v0: i64| -> Rc<str> {
        digit_31(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure9() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        let mut v1: Rc<str> = html_render::html_render_md(&v0);
        v1
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_pages_viewer(v0: &str) -> viewer {
    closure0()(Rc::<str>::from(v0))
}
pub fn html_pages_sorted(v0: Rc<list_node>) -> Rc<list_node> {
    closure1()(v0)
}
pub fn html_pages_rel(v0: &str, v1: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0), Rc::<str>::from(v1))
}
pub fn html_pages_tree(v0: bool, v1: Rc<list_node>) -> Rc<str> {
    closure3()(v0, v1)
}
pub fn html_pages_index(v0: bool, v1: Rc<list_node>, v2: bool, v3: &str) -> Rc<str> {
    closure5()(v0, v1, v2, Rc::<str>::from(v3))
}
pub fn html_pages_file(v0: &str, v1: viewer, v2: &str) -> Rc<str> {
    closure6()(Rc::<str>::from(v0), v1, Rc::<str>::from(v2))
}
pub fn html_pages_fixture() -> Rc<str> {
    closure7()()
}
pub fn html_pages_digit(v0: i64) -> Rc<str> {
    closure8()(v0)
}
pub fn html_pages_md(v0: &str) -> Rc<str> {
    closure9()(Rc::<str>::from(v0))
}
