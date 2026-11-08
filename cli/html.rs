#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{list_string, node, list_node, viewer};
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
    US0_Some(Rc<str>),
    US0_None,
}
#[derive(Clone)]
enum US1 {
    US1_Render(Rc<str>, Rc<str>, bool),
    US1_SelfTest,
    US1_BadArgs(Rc<str>),
}
#[derive(Clone)]
enum US3 {
    US3_MissingFlag(Rc<str>),
    US3_BadNumber(Rc<str>, Rc<str>),
    US3_UnknownVerb(Rc<str>, Rc<list_string>),
    US3_NoVerb(Rc<list_string>),
    US3_Help(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_Ok(Rc<str>, Rc<str>),
    US2_Error(US3),
}
#[derive(Clone)]
enum US4 {
    US4_Some(US3),
    US4_None,
}
#[derive(Clone)]
enum US5 {
    US5_Wrote(i32),
    US5_Mismatch(Rc<str>),
    US5_Refused(Rc<str>),
}
fn command_line_argument_count_1() -> i32 {
    let mut v0: usize = std::env::args().count();
    let mut v1: i32 = ((v0) as i32);
    let mut v2: i32 = v1.wrapping_sub(1i32);
    v2
}
fn command_line_argument_2(mut v0: i32) -> Rc<str> {
    let mut v1: i32 = v0.wrapping_add(1i32);
    let mut v2: usize = ((v1) as usize);
    let v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_0(mut v0: i32) -> Rc<list_string> {
    let mut v1: bool = v0 >= 0i32;
    let v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_1();
        let v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let mut v5: Rc<str> = command_line_argument_2(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10;
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<list_string> = arguments_from_0(v11);
            Rc::new(list_string::list_string_Cons(v10, v12))
        }
    }
}
fn walk_5(mut v0: Rc<list_string>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<list_string> = v2;
                let v3: bool = &*v1 == "--help";
                if v3 {
                    return true;
                } else {
                    v0 = v2;
                    continue;
                }
            }
            list_string::list_string_Nil => {
                return false;
            }
        }
    }
}
fn value_after_6(mut v0: Rc<str>, mut v1: Rc<list_string>) -> US0 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<list_string> = v3;
                match Rc::unwrap_or_clone(v3) {
                    list_string::list_string_Cons(v4, v5) => {
                        let v4: Rc<str> = v4;
                        let mut v5: Rc<list_string> = v5;
                        let mut v6: bool = &*v2 == &*v0;
                        if v6 {
                            return US0::US0_Some(v4);
                        } else {
                            let v8: Rc<list_string> = Rc::new(list_string::list_string_Cons(v4, v5));
                            v1 = v8;
                            continue;
                        }
                    }
                    _ => {
                        return US0::US0_None;
                    }
                }
            }
            _ => {
                return US0::US0_None;
            }
        }
    }
}
fn method7(mut v0: Rc<list_string>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2;
                let v3: Rc<list_string> = v3;
                let v4: bool = &*v1 == "";
                let mut v7: Rc<str> = if v4 {
                    v2
                } else {
                    let mut v6: Rc<str> = string_concat(&[&*v1, " | ", &*v2]);
                    v6
                };
                (v0, v1) = (v3, v7);
                continue;
            }
            list_string::list_string_Nil => {
                return v1;
            }
        }
    }
}
fn walk_8(mut v0: Rc<list_string>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<list_string> = v2;
                let v3: bool = &*v1 == "--viewers";
                if v3 {
                    return true;
                } else {
                    v0 = v2;
                    continue;
                }
            }
            list_string::list_string_Nil => {
                return false;
            }
        }
    }
}
fn render_mode_4(mut v0: Rc<list_string>) -> US1 {
    let mut v1: bool = walk_5(v0.clone());
    let v42: US2 = if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir <text> --html <text>"); } LIT.with(|lit| lit.clone()) };
        let v3: US3 = US3::US3_Help(v2);
        US2::US2_Error(v3)
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
        let mut v6: US0 = value_after_6(v5.clone(), v0.clone());
        let v13: US4 = match v6 {
            US0::US0_None => {
                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
                let v10: US3 = US3::US3_MissingFlag(v9);
                US4::US4_Some(v10)
            }
            US0::US0_Some(v7) => {
                let mut v7: Rc<str> = v7;
                US4::US4_None
            }
        };
        let v25: US4 = match &v13 {
            US4::US4_None => {
                let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                let v16: US0 = value_after_6(v15, v0.clone());
                match v16 {
                    US0::US0_None => {
                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                        let v20: US3 = US3::US3_MissingFlag(v19);
                        US4::US4_Some(v20)
                    }
                    US0::US0_Some(v17) => {
                        let v17: Rc<str> = v17;
                        US4::US4_None
                    }
                }
            }
            US4::US4_Some(v14) => {
                let mut v14: US3 = v14.clone();
                v13
            }
        };
        match v25 {
            US4::US4_None => {
                let v28: US0 = value_after_6(v5, v0.clone());
                let v32: Rc<str> = match v28 {
                    US0::US0_None => {
                        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --dir"); } LIT.with(|lit| lit.clone()) }))
                    }
                    US0::US0_Some(v29) => {
                        let v29: Rc<str> = v29;
                        v29
                    }
                };
                let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                let v34: US0 = value_after_6(v33, v0.clone());
                let v38: Rc<str> = match v34 {
                    US0::US0_None => {
                        std::panic::panic_any::<std::string::String>(format!("{}", { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --html"); } LIT.with(|lit| lit.clone()) }))
                    }
                    US0::US0_Some(v35) => {
                        let v35: Rc<str> = v35;
                        v35
                    }
                };
                US2::US2_Ok(v32, v38)
            }
            US4::US4_Some(v26) => {
                let v26: US3 = v26;
                US2::US2_Error(v26)
            }
        }
    };
    match v42 {
        US2::US2_Error(v47) => {
            let v47: US3 = v47;
            let v73: Rc<str> = match v47 {
                US3::US3_BadNumber(v51, v52) => {
                    let v51: Rc<str> = v51;
                    let v52: Rc<str> = v52;
                    let v56: Rc<str> = string_concat(&["error: --", &*v51, " expects an integer, got '", &*v52, "'"]);
                    v56
                }
                US3::US3_Help(v68) => {
                    let v68: Rc<str> = v68;
                    v68
                }
                US3::US3_MissingFlag(v48) => {
                    let v48: Rc<str> = v48;
                    let v50: Rc<str> = string_concat(&["error: missing --", &*v48, " <value>"]);
                    v50
                }
                US3::US3_NoVerb(v64) => {
                    let v64: Rc<list_string> = v64;
                    let v65: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v66: Rc<str> = method7(v64, v65);
                    let v67: Rc<str> = string_concat(&["error: no command given\nvalid: ", &*v66]);
                    v67
                }
                US3::US3_UnknownVerb(v57, v58) => {
                    let v57: Rc<str> = v57;
                    let v58: Rc<list_string> = v58;
                    let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v62: Rc<str> = method7(v58, v61);
                    let v63: Rc<str> = string_concat(&["error: unknown command '", &*v57, "'\nvalid: ", &*v62]);
                    v63
                }
            };
            US1::US1_BadArgs(v73)
        }
        US2::US2_Ok(v43, v44) => {
            let v43: Rc<str> = v43;
            let v44: Rc<str> = v44;
            let v45: bool = walk_8(v0);
            US1::US1_Render(v43, v44, v45)
        }
    }
}
fn walk_9(mut v0: Rc<list_string>) -> bool {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v1, v2) => {
                let mut v1: Rc<str> = v1;
                let mut v2: Rc<list_string> = v2;
                let v3: bool = &*v1 == "--self-test";
                if v3 {
                    return true;
                } else {
                    v0 = v2;
                    continue;
                }
            }
            list_string::list_string_Nil => {
                return false;
            }
        }
    }
}
fn mode_of_3(mut v0: Rc<list_string>) -> US1 {
    let mut v1: bool = walk_9(v0.clone());
    if v1 {
        US1::US1_SelfTest
    } else {
        render_mode_4(v0)
    }
}
fn put_17(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    std::fs::write(&*v0, &*v1).unwrap();
    ()
}
fn put_page_16(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: viewer) -> i32 {
    let v3: Rc<str> = string_concat(&[&*v0, ".html"]);
    let v4: Rc<str> = html_walk::html_walk_slurped(&v0);
    let mut v5: Rc<str> = html_pages::html_pages_file(&v1, v2, &v4);
    put_17(v3, v5);
    1i32
}
fn file_exists_18(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
    let v4: std::string::String = String::from(v2);
    let mut v6: std::path::PathBuf = std::path::PathBuf::from(v4);
    let v8: bool = v6.exists();
    if v8 {
        let v10: bool = v6.is_file();
        v10
    } else {
        false
    }
}
fn write_file_page_15(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: viewer = html_pages::html_pages_viewer(&v1);
    match &v2 {
        viewer::viewer_Direct => {
            0i32
        }
        _ => {
            let v3: Rc<str> = string_concat(&[&*v0, ".html"]);
            let v4: bool = file_exists_18(v3);
            if v4 {
                0i32
            } else {
                put_page_16(v0, v1, v2)
            }
        }
    }
}
fn method20(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method19(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method20(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn page_at_14(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let v3: Rc<str> = method19(v0, v1);
    write_file_page_15(v3, v2)
}
fn write_file_pages_13(mut v0: Rc<str>, mut v1: Rc<list_node>, mut v2: i32) -> i32 {
    loop {
        match Rc::unwrap_or_clone(v1) {
            list_node::list_node_Cons(v3, v4) => {
                let v3: Rc<node> = v3;
                let v4: Rc<list_node> = v4;
                match Rc::unwrap_or_clone(v3) {
                    node::node_File(v5, v6, v7) => {
                        let mut v5: Rc<str> = v5;
                        let mut v6: Rc<str> = v6;
                        let mut v7: i64 = v7;
                        let v8: i32 = page_at_14(v0.clone(), v6, v5);
                        let v9: i32 = v2.wrapping_add(v8);
                        (v1, v2) = (v4, v9);
                        continue;
                    }
                    node::node_Folder(v11, v12, v13) => {
                        let v11: Rc<str> = v11;
                        let v12: Rc<str> = v12;
                        let v13: Rc<list_node> = v13;
                        let mut v14: i32 = write_file_pages_13(v0.clone(), v13, v2);
                        (v1, v2) = (v4, v14);
                        continue;
                    }
                }
            }
            list_node::list_node_Nil => {
                return v2;
            }
        }
    }
}
fn write_index_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool, mut v3: Rc<list_node>) -> () {
    let (mut v4, mut v5): (bool, Rc<str>) = html_walk::html_walk_readme(&v0);
    let mut v6: Rc<str> = html_pages::html_pages_index(v2, v3, v4, &v5);
    put_17(v1, v6)
}
fn render_listed_12(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> US5 {
    let v3: Rc<list_node> = html_walk::html_walk_walked(&v0);
    let mut v6: i32 = if v2 {
        let v4: i32 = 0i32;
        write_file_pages_13(v0.clone(), v3.clone(), v4)
    } else {
        0i32
    };
    write_index_21(v0, v1, v2, v3);
    US5::US5_Wrote(v6)
}
fn render_tree_11(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> US5 {
    let v3: Rc<str> = html_walk::html_walk_refusal(&v0);
    let v4: bool = &*v3 == "";
    if v4 {
        render_listed_12(v0, v1, v2)
    } else {
        US5::US5_Refused(v3)
    }
}
fn get_temp_path_25() -> Rc<str> {
    let mut v1: std::path::PathBuf = std::env::temp_dir();
    let mut v2: std::path::PathBuf = method20(v1);
    let v4: std::path::Display = v2.display();
    let mut v6: std::string::String = format!("{}", v4);
    let v8: Rc<str> = Rc::<str>::from(String::as_str(&v6));
    let v9: i32 = (v8.clone().len() as i32);
    let v10: bool = v9 > 1i32;
    let v16: bool = if v10 {
        let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v12: bool = v8.ends_with(&*v11);
        if v12 {
            true
        } else {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\\"); } LIT.with(|lit| lit.clone()) };
            let mut v14: bool = v8.ends_with(&*v13);
            v14
        }
    } else {
        false
    };
    if v16 {
        let v17: i32 = v9.wrapping_sub(1i32);
        let v18: usize = ((0i32) as usize);
        let v19: usize = ((v17) as usize);
        let v20: Rc<str> = Rc::<str>::from(&(v8)[v18..v19]);
        v20
    } else {
        v8
    }
}
fn go_26(mut v0: i32, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let v3: bool = v0 <= 0i32;
        if v3 {
            return v2;
        } else {
            let v4: u64 = v1 & 15u64;
            let mut v5: i32 = (v4 as i32);
            let mut v6: i32 = v0.wrapping_sub(1i32);
            let mut v7: u64 = v1.wrapping_shr((4i32) as u32);
            let v8: Rc<str> = string_slice(&{ thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789abcdef"); } LIT.with(|lit| lit.clone()) }, v5 as i64, v5 as i64);
            let v9: Rc<str> = string_concat(&[&*v8, &*v2]);
            (v0, v1, v2) = (v6, v7, v9);
            continue;
        }
    }
}
fn fixture_directory_24() -> Rc<str> {
    let mut v0: Rc<str> = get_temp_path_25();
    let mut v1: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
    let v3: bool = true; // 1;
    let mut v1 = v1;
    let mut v5: std::time::SystemTime = std::time::SystemTime::now();
    let mut v6: std::time::Duration = v5.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let mut v7: u64 = v6.as_secs();
    let v8: u64 = v7.wrapping_mul(1000000000u64);
    let v9: u32 = v6.subsec_nanos();
    let v10: u64 = (v9 as u64);
    let v11: u64 = v8.wrapping_add(v10);
    std::hash::Hasher::write_u64(&mut v1, v11);
    let v12: u64 = std::hash::Hasher::finish(&v1);
    std::hash::Hasher::write_u64(&mut v1, v12);
    let v13: u64 = std::hash::Hasher::finish(&v1);
    let mut v14: u64 = v12 & 4294967295u64;
    let v15: i32 = 8i32;
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = go_26(v15, v14, v16.clone());
    let v19: u64 = v12.wrapping_shr((32i32) as u32);
    let v20: u64 = v19 & 65535u64;
    let v21: i32 = 4i32;
    let v22: Rc<str> = go_26(v21, v20, v16.clone());
    let v25: u64 = v12.wrapping_shr((48i32) as u32);
    let v26: u64 = v25 & 4095u64;
    let v27: i32 = 3i32;
    let v28: Rc<str> = go_26(v27, v26, v16.clone());
    let v31: u64 = v13 & 16383u64;
    let v32: u64 = v31 | 32768u64;
    let v33: i32 = 4i32;
    let v34: Rc<str> = go_26(v33, v32, v16.clone());
    let v37: u64 = v13.wrapping_shr((16i32) as u32);
    let v38: u64 = v37 & 281474976710655u64;
    let v39: i32 = 12i32;
    let v40: Rc<str> = go_26(v39, v38, v16);
    let v41: Rc<str> = string_concat(&[&*v17, "-", &*v22, "-4", &*v28, "-", &*v34, "-", &*v40]);
    let v42: Rc<str> = Rc::<str>::from(v41.to_lowercase());
    let v43: Rc<str> = v42 ;
    let v44: Rc<str> = string_concat(&["html-self-test-", &*v43]);
    method19(v0, v44)
}
fn make_directory_28(mut v0: Rc<str>) -> () {
    std::fs::create_dir_all(&*v0).unwrap();
    ()
}
fn times_29(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let mut v2: bool = v1 <= 0i64;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let v4: i64 = v1.wrapping_sub(1i64);
        let mut v5: Rc<str> = times_29(v0.clone(), v4);
        let mut v6: Rc<str> = string_concat(&[&*v0, &*v5]);
        v6
    }
}
fn lay_27(mut v0: Rc<str>, mut v1: i64) -> () {
    loop {
        let mut v2: bool = v1 >= 0i64;
        if v2 {
            make_directory_28(v0.clone());
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let v4: Rc<str> = method19(v0.clone(), v3);
            let mut v5: Rc<str> = html_pages::html_pages_digit(v1);
            let mut v6: i64 = v1.wrapping_add(1i64);
            let mut v7: Rc<str> = times_29(v5, v6);
            put_17(v4, v7);
            let v8: Rc<str> = html_pages::html_pages_digit(v1);
            let v9: Rc<str> = method19(v0, v8);
            let v10: i64 = v1.wrapping_sub(1i64);
            (v0, v1) = (v9, v10);
            continue;
        }
        return ();
    }
}
fn tree_check_23() -> US0 {
    let mut v0: Rc<str> = fixture_directory_24();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_.root"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method19(v0.clone(), v1);
    let v3: i64 = 3i64;
    lay_27(v2, v3);
    let v4: Rc<list_node> = html_walk::html_walk_walked(&v0);
    let mut v5: Rc<str> = html_pages::html_pages_tree(false, v4);
    let mut v6: Rc<str> = html_pages::html_pages_fixture();
    let mut v7: bool = &*v5 != &*v6;
    if v7 {
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    }
}
fn viewer_fixture_31() -> Rc<str> {
    let mut v0: Rc<str> = fixture_directory_24();
    make_directory_28(v0.clone());
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method19(v0.clone(), v1);
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x < 1"); } LIT.with(|lit| lit.clone()) };
    put_17(v2, v3);
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi.html"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = method19(v0.clone(), v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kept"); } LIT.with(|lit| lit.clone()) };
    put_17(v5, v6);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = method19(v0.clone(), v7);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1\th2\nv1\tv2"); } LIT.with(|lit| lit.clone()) };
    put_17(v8, v9);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c.txt"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = method19(v0.clone(), v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("t"); } LIT.with(|lit| lit.clone()) };
    put_17(v11, v12);
    v0
}
fn viewers_kept_32(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<list_node> = html_walk::html_walk_walked(&v0);
    let mut v2: i32 = 0i32;
    let v3: i32 = write_file_pages_13(v0.clone(), v1, v2);
    let v4: bool = v3 == 1i32;
    let v8: bool = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv.html"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = method19(v0.clone(), v5);
        file_exists_18(v6)
    } else {
        false
    };
    let v13: bool = if v8 {
        let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi.html"); } LIT.with(|lit| lit.clone()) };
        let v10: Rc<str> = method19(v0.clone(), v9);
        let v11: Rc<str> = html_walk::html_walk_slurped(&v10);
        let v12: bool = &*v11 == "kept";
        v12
    } else {
        false
    };
    if v13 {
        let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c.txt.html"); } LIT.with(|lit| lit.clone()) };
        let v15: Rc<str> = method19(v0, v14);
        let v16: bool = file_exists_18(v15);
        let v17: bool = v16 == false;
        v17
    } else {
        false
    }
}
fn missing_refused_33() -> bool {
    let mut v0: Rc<str> = fixture_directory_24();
    let mut v1: Rc<str> = string_concat(&[&*v0, ".html"]);
    let mut v2: bool = false;
    let v3: US5 = render_tree_11(v0, v1.clone(), v2);
    match v3 {
        US5::US5_Refused(v4) => {
            let v4: Rc<str> = v4;
            let mut v5: bool = file_exists_18(v1);
            let mut v6: bool = v5 == false;
            v6
        }
        _ => {
            false
        }
    }
}
fn fixture_failure_30() -> US0 {
    let mut v0: Rc<str> = viewer_fixture_31();
    let mut v1: bool = viewers_kept_32(v0);
    let mut v2: bool = v1 == false;
    if v2 {
        let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("viewers: want b.tsv.html, a.spi.html kept, no c.txt.html"); } LIT.with(|lit| lit.clone()) };
        US0::US0_Some(v3)
    } else {
        let mut v5: bool = missing_refused_33();
        let mut v6: bool = v5 == false;
        if v6 {
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("walk: an unreadable folder must be refused"); } LIT.with(|lit| lit.clone()) };
            US0::US0_Some(v7)
        } else {
            US0::US0_None
        }
    }
}
fn self_test_22() -> US5 {
    let mut v0: US0 = tree_check_23();
    match v0 {
        US0::US0_None => {
            let v3: US0 = fixture_failure_30();
            match v3 {
                US0::US0_None => {
                    US5::US5_Wrote(0i32)
                }
                US0::US0_Some(v4) => {
                    let v4: Rc<str> = v4;
                    US5::US5_Mismatch(v4)
                }
            }
        }
        US0::US0_Some(v1) => {
            let mut v1: Rc<str> = v1;
            US5::US5_Mismatch(v1)
        }
    }
}
fn run_10(mut v0: US1) -> US5 {
    match v0 {
        US1::US1_BadArgs(v6) => {
            let mut v6: Rc<str> = v6;
            US5::US5_Refused(v6)
        }
        US1::US1_Render(v1, v2, v3) => {
            let mut v1: Rc<str> = v1;
            let mut v2: Rc<str> = v2;
            let v3: bool = v3;
            render_tree_11(v1, v2, v3)
        }
        US1::US1_SelfTest => {
            self_test_22()
        }
    }
}
fn exit_of_34(mut v0: US5) -> i32 {
    match v0 {
        US5::US5_Mismatch(v2) => {
            let mut v2: Rc<str> = v2;
            let v3: Rc<str> = string_concat(&["html self-test mismatch: ", &*v2]);
            eprintln!("{}", v3);
            1i32
        }
        US5::US5_Refused(v4) => {
            let v4: Rc<str> = v4;
            eprintln!("{}", v4);
            2i32
        }
        US5::US5_Wrote(v1) => {
            let mut v1: i32 = v1;
            0i32
        }
    }
}
fn spiral_main() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<list_string> = arguments_from_0(v0);
    let mut v2: US1 = mode_of_3(v1);
    let v3: US5 = run_10(v2);
    exit_of_34(v3)
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
