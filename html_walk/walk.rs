#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{list_string, node, list_node};
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
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
#[derive(Clone)]
enum US0 {
    US0_Listed(Rc<list_string>),
    US0_Unreadable(Rc<str>),
}
enum US1 {
    US1_Ok(Vec<std::string::String>),
    US1_Error(std::io::Error),
}
fn ok_c1() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US1> = Rc::new(move |mut v0: Vec<std::string::String>| -> US1 {
        US1::US1_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method2() -> Rc<dyn Fn(Vec<std::string::String>) -> US1> {
    ok_c1()
}
fn error_c2() -> Rc<dyn Fn(std::io::Error) -> US1> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US1> = Rc::new(move |mut v0: std::io::Error| -> US1 {
        US1::US1_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method3() -> Rc<dyn Fn(std::io::Error) -> US1> {
    error_c2()
}
fn names_of_1(mut v0: Rc<str>) -> US0 {
    let v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v3: Rc<dyn Fn(Vec<std::string::String>) -> US1> = method2();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US1> = method3();
    let v5: US1 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    match v5 {
        US1::US1_Error(v22) => {
            let v22: std::io::Error = v22;
            let v23: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v22)).s() });
            US0::US0_Unreadable(v23)
        }
        US1::US1_Ok(v6) => {
            let mut v6: Vec<std::string::String> = v6;
            let mut v7: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
            let v9: bool = true; let _vec_fold_ = v6.into_iter().fold(v7, |acc, x| { //;
            let v11: Rc<list_string> = acc;
            let v13: std::string::String = x;
            let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            let v17: Rc<list_string> = Rc::new(list_string::list_string_Cons(v15, v11));
            let v18: bool = true; v17 });
            let v20: Rc<list_string> = _vec_fold_;
            US0::US0_Listed(v20)
        }
    }
}
fn closure4(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<list_node> {
        walk_0(v0.clone(), v1)
    })
}
fn closure3() -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>> {
        closure4(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method6(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method5(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method6(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn is_link_8(mut v0: Rc<str>) -> bool {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v0);
    let v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        false
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let v5: bool = v4.file_type().is_symlink();
        v5
    }
}
fn directory_exists_10(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_dir();
        v10
    } else {
        false
    }
}
fn is_directory_9(mut v0: Rc<str>) -> bool {
    directory_exists_10(v0)
}
fn folder_of_11(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>) -> Rc<node> {
    let mut v4: Rc<dyn Fn(Rc<str>) -> Rc<list_node>> = v0(v1);
    let v5: Rc<list_node> = v4(v2.clone());
    Rc::new(node::node_Folder(v3, v2, v5))
}
fn file_exists_12(mut v0: Rc<str>) -> bool {
    let v2: &str = &*v0;
    let mut v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn bytes_of_14(mut v0: Rc<str>) -> i64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        0i64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let v5: u64 = v4.len();
        let mut v6: i64 = (v5 as i64);
        v6
    }
}
fn file_of_13(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> Rc<node> {
    let mut v3: i64 = bytes_of_14(v0);
    Rc::new(node::node_File(v2, v1, v3))
}
fn node_at_7(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<list_node>) -> Rc<list_node> {
    let v5: bool = is_link_8(v1.clone());
    if v5 {
        v4
    } else {
        let mut v6: bool = is_directory_9(v1.clone());
        if v6 {
            let mut v7: Rc<node> = folder_of_11(v0, v1, v2, v3);
            Rc::new(list_node::list_node_Cons(v7, v4))
        } else {
            let v9: bool = file_exists_12(v1.clone());
            if v9 {
                let v10: Rc<node> = file_of_13(v1, v2, v3);
                Rc::new(list_node::list_node_Cons(v10, v4))
            } else {
                v4
            }
        }
    }
}
fn nodes_of_4(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<list_string>, mut v4: Rc<list_node>) -> Rc<list_node> {
    loop {
        match Rc::unwrap_or_clone(v3) {
            list_string::list_string_Cons(v5, v6) => {
                let v5: Rc<str> = v5;
                let mut v6: Rc<list_string> = v6;
                let mut v7: Rc<str> = method5(v1.clone(), v5.clone());
                let v8: Rc<str> = html_pages::html_pages_rel(&v2, &v5);
                let v9: Rc<list_node> = node_at_7(v0.clone(), v7, v8, v5, v4);
                (v3, v4) = (v6, v9);
                continue;
            }
            list_string::list_string_Nil => {
                return v4;
            }
        }
    }
}
fn unreadable_text_15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v4: Rc<str> = string_concat(&["html: cannot read ", &*v0, ": ", &*v1]);
    v4
}
fn walk_0(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<list_node> {
    let v2: US0 = names_of_1(v0.clone());
    match v2 {
        US0::US0_Listed(v3) => {
            let mut v3: Rc<list_string> = v3;
            let mut v4: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>> = closure3();
            let v5: Rc<list_node> = { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) };
            let mut v6: Rc<list_node> = nodes_of_4(v4, v0, v1, v3, v5);
            let mut v7: Rc<list_node> = html_pages::html_pages_sorted(v6);
            v7
        }
        US0::US0_Unreadable(v8) => {
            let v8: Rc<str> = v8;
            let v9: Rc<str> = unreadable_text_15(v0, v8);
            eprintln!("{}", v9);
            { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<list_node>> = Rc::new(move |mut v0: Rc<str>| -> Rc<list_node> {
        let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        walk_0(v0, v1)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn run_of_20(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: bool = v2 <= v1;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let v5: i32 = v2.wrapping_sub(1i32);
        let mut v6: Rc<str> = string_slice(&v0, v1 as i64, v5 as i64);
        v6
    }
}
fn drop_final_newline_19(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v2: bool = v0.ends_with(&*v1);
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: i32 = v4.wrapping_sub(1i32);
        run_of_20(v0, v3, v5)
    } else {
        v0
    }
}
fn read_lossy_21(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = std::rc::Rc::<str>::from(String::from_utf8_lossy(&std::fs::read(&*v0).unwrap()).as_ref());
    v1
}
fn slurp_18(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = read_lossy_21(v0);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(v1.replace(&*v2, &*v3));
    drop_final_newline_19(v4)
}
fn readme_in_17(mut v0: Rc<str>, mut v1: Rc<list_string>) -> (bool, Rc<str>) {
    loop {
        match Rc::unwrap_or_clone(v1) {
            list_string::list_string_Cons(v3, v4) => {
                let mut v3: Rc<str> = v3;
                let mut v4: Rc<list_string> = v4;
                let v5: Rc<str> = Rc::<str>::from(v3.to_lowercase());
                let mut v6: bool = &*v5 == "readme.md";
                if v6 {
                    let mut v7: Rc<str> = method5(v0, v3);
                    let v8: Rc<str> = slurp_18(v7);
                    return (true, v8);
                } else {
                    v1 = v4;
                    continue;
                }
            }
            list_string::list_string_Nil => {
                let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return (false, v2);
            }
        }
    }
}
fn readme_of_16(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: US0 = names_of_1(v0.clone());
    match v1 {
        US0::US0_Listed(v2) => {
            let v2: Rc<list_string> = v2;
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = method5(v0.clone(), v3.clone());
            let v5: bool = file_exists_12(v4);
            let v8: Rc<list_string> = if v5 {
                let mut v6: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
                Rc::new(list_string::list_string_Cons(v3, v6))
            } else {
                v2
            };
            readme_in_17(v0, v8)
        }
        US0::US0_Unreadable(v11) => {
            let v11: Rc<str> = v11;
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            (false, v12)
        }
    }
}
fn closure5() -> Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> (bool, Rc<str>)> = Rc::new(move |mut v0: Rc<str>| -> (bool, Rc<str>) {
        readme_of_16(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn refusal_of_22(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: US0 = names_of_1(v0.clone());
    match v1 {
        US0::US0_Listed(v2) => {
            let v2: Rc<list_string> = v2;
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US0::US0_Unreadable(v4) => {
            let mut v4: Rc<str> = v4;
            unreadable_text_15(v0, v4)
        }
    }
}
fn closure6() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        refusal_of_22(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn closure7() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        slurp_18(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_walk_walked(v0: &str) -> Rc<list_node> {
    closure0()(Rc::<str>::from(v0))
}
pub fn html_walk_readme(v0: &str) -> (bool, Rc<str>) {
    closure5()(Rc::<str>::from(v0))
}
pub fn html_walk_refusal(v0: &str) -> Rc<str> {
    closure6()(Rc::<str>::from(v0))
}
pub fn html_walk_slurped(v0: &str) -> Rc<str> {
    closure7()(Rc::<str>::from(v0))
}
