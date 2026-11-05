#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{list_string, node, list_node, viewer};
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
#[derive(Clone)]
enum US6 {
    US6_Listed(Rc<list_string>),
    US6_Unreadable(Rc<str>),
}
enum US7 {
    US7_Ok(Vec<std::string::String>),
    US7_Error(std::io::Error),
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
    let mut v3: Rc<str> = Rc::<str>::from(std::env::args().nth(v2).unwrap_or_default());
    v3
}
fn arguments_from_0(mut v0: i32) -> Rc<list_string> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_1();
        let mut v3: bool = v0 < v2;
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
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<list_string> = arguments_from_0(v11);
            Rc::new(list_string::list_string_Cons(v10, v12))
        }
    }
}
fn walk_5(mut v0: Rc<str>, mut v1: Rc<list_string>) -> bool {
    loop {
        match &*v1 {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<list_string> = v3.clone();
                let mut v4: bool = &*v2 == &*v0;
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
fn value_after_6(mut v0: Rc<str>, mut v1: Rc<list_string>) -> US0 {
    loop {
        match &*v1 {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<list_string> = v3.clone();
                match &*v3 {
                    list_string::list_string_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4.clone();
                        let mut v5: Rc<list_string> = v5.clone();
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
        match &*v0 {
            list_string::list_string_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<list_string> = v3.clone();
                let mut v4: bool = &*v1 == "";
                let v8: Rc<str> = if v4 {
                    v2
                } else {
                    let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | "); } LIT.with(|lit| lit.clone()) };
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v5)), v2));
                    v7
                };
                (v0, v1) = (v3, v8);
                continue;
            }
            list_string::list_string_Nil => {
                return v1;
            }
        }
    }
}
fn render_mode_4(mut v0: Rc<list_string>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = walk_5(v3, v0.clone());
    let v67: US2 = if v4 {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <text>"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir <text>"); } LIT.with(|lit| lit.clone()) };
        let v9: bool = &*v8 == "";
        let v11: Rc<str> = if v9 {
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v10
        } else {
            v8
        };
        let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
        let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v12)), v7));
        let v15: bool = &*v14 == "";
        let v21: Rc<str> = if v15 {
            v11
        } else {
            let v16: bool = &*v11 == "";
            if v16 {
                v14
            } else {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v11, v17)), v14));
                v19
            }
        };
        let v22: US3 = US3::US3_Help(v21);
        US2::US2_Error(v22)
    } else {
        let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
        let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
        let v26: US0 = value_after_6(v25, v0.clone());
        let v32: US4 = match &v26 {
            US0::US0_None => {
                let v29: US3 = US3::US3_MissingFlag(v24);
                US4::US4_Some(v29)
            }
            US0::US0_Some(v27) => {
                let v27: Rc<str> = v27.clone();
                US4::US4_None
            }
        };
        let v44: US4 = match &v32 {
            US4::US4_None => {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                let v35: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                let v36: US0 = value_after_6(v35, v0.clone());
                match &v36 {
                    US0::US0_None => {
                        let v39: US3 = US3::US3_MissingFlag(v34);
                        US4::US4_Some(v39)
                    }
                    US0::US0_Some(v37) => {
                        let v37: Rc<str> = v37.clone();
                        US4::US4_None
                    }
                }
            }
            US4::US4_Some(v33) => {
                let v33: US3 = v33.clone();
                v32
            }
        };
        match &v44 {
            US4::US4_None => {
                let v47: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
                let v48: US0 = value_after_6(v47, v0.clone());
                let v54: Rc<str> = match &v48 {
                    US0::US0_None => {
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                        let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --dir"); } LIT.with(|lit| lit.clone()) };
                        std::panic::panic_any::<std::string::String>(format!("{}", v51))
                    }
                    US0::US0_Some(v49) => {
                        let v49: Rc<str> = v49.clone();
                        v49
                    }
                };
                let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                let v57: US0 = value_after_6(v56, v0.clone());
                let v63: Rc<str> = match &v57 {
                    US0::US0_None => {
                        let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                        let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --html"); } LIT.with(|lit| lit.clone()) };
                        std::panic::panic_any::<std::string::String>(format!("{}", v60))
                    }
                    US0::US0_Some(v58) => {
                        let v58: Rc<str> = v58.clone();
                        v58
                    }
                };
                US2::US2_Ok(v54, v63)
            }
            US4::US4_Some(v45) => {
                let v45: US3 = v45.clone();
                US2::US2_Error(v45)
            }
        }
    };
    match &v67 {
        US2::US2_Error(v74) => {
            let v74: US3 = v74.clone();
            let v108: Rc<str> = match &v74 {
                US3::US3_BadNumber(v80, v81) => {
                    let v80: Rc<str> = v80.clone();
                    let v81: Rc<str> = v81.clone();
                    let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: --"); } LIT.with(|lit| lit.clone()) };
                    let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v82, v80));
                    let v84: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects an integer, got '"); } LIT.with(|lit| lit.clone()) };
                    let v86: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v83, v84)), v81));
                    let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                    let v88: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v87));
                    v88
                }
                US3::US3_Help(v103) => {
                    let v103: Rc<str> = v103.clone();
                    v103
                }
                US3::US3_MissingFlag(v75) => {
                    let v75: Rc<str> = v75.clone();
                    let v76: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: missing --"); } LIT.with(|lit| lit.clone()) };
                    let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v76, v75));
                    let v78: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <value>"); } LIT.with(|lit| lit.clone()) };
                    let v79: Rc<str> = Rc::<str>::from(format!("{}{}", v77, v78));
                    v79
                }
                US3::US3_NoVerb(v98) => {
                    let v98: Rc<list_string> = v98.clone();
                    let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v100: Rc<str> = method7(v98, v99);
                    let v101: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: no command given\nvalid: "); } LIT.with(|lit| lit.clone()) };
                    let v102: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v100));
                    v102
                }
                US3::US3_UnknownVerb(v89, v90) => {
                    let v89: Rc<str> = v89.clone();
                    let v90: Rc<list_string> = v90.clone();
                    let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: unknown command '"); } LIT.with(|lit| lit.clone()) };
                    let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v91, v89));
                    let v93: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\nvalid: "); } LIT.with(|lit| lit.clone()) };
                    let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
                    let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                    let v96: Rc<str> = method7(v90, v95);
                    let v97: Rc<str> = Rc::<str>::from(format!("{}{}", v94, v96));
                    v97
                }
            };
            US1::US1_BadArgs(v108)
        }
        US2::US2_Ok(v68, v69) => {
            let v68: Rc<str> = v68.clone();
            let v69: Rc<str> = v69.clone();
            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("viewers"); } LIT.with(|lit| lit.clone()) };
            let v71: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--viewers"); } LIT.with(|lit| lit.clone()) };
            let v72: bool = walk_5(v71, v0);
            US1::US1_Render(v68, v69, v72)
        }
    }
}
fn mode_of_3(mut v0: Rc<list_string>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("self-test"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--self-test"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = walk_5(v3, v0.clone());
    if v4 {
        US1::US1_SelfTest
    } else {
        render_mode_4(v0)
    }
}
fn ok_c0() -> Rc<dyn Fn(Vec<std::string::String>) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US7> = Rc::new(move |mut v0: Vec<std::string::String>| -> US7 {
        US7::US7_Ok(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method11() -> Rc<dyn Fn(Vec<std::string::String>) -> US7> {
    ok_c0()
}
fn error_c1() -> Rc<dyn Fn(std::io::Error) -> US7> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US7> = Rc::new(move |mut v0: std::io::Error| -> US7 {
        US7::US7_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method12() -> Rc<dyn Fn(std::io::Error) -> US7> {
    error_c1()
}
fn names_of_10(mut v0: Rc<str>) -> US6 {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v3: Rc<dyn Fn(Vec<std::string::String>) -> US7> = method11();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US7> = method12();
    let mut v5: US7 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    match v5 {
        US7::US7_Error(v22) => {
            let v22: std::io::Error = v22;
            let v23: Rc<str> = Rc::<str>::from({ struct W<T>(T); trait D { fn s(&self) -> String; } impl<T: std::fmt::Debug> D for &W<T> { fn s(&self) -> String { format!("{:?}", self.0) } } trait P { fn s(&self) -> String; } impl<T> P for W<T> { fn s(&self) -> String { String::from("<std::io::Error>") } } (&&W(&v22)).s() });
            US6::US6_Unreadable(v23)
        }
        US7::US7_Ok(v6) => {
            let mut v6: Vec<std::string::String> = v6.clone();
            let mut v7: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
            let v9: bool = true; let _vec_fold_ = v6.into_iter().fold(v7, |acc, x| { //;
            let v11: Rc<list_string> = acc;
            let v13: std::string::String = x;
            let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            let v17: Rc<list_string> = Rc::new(list_string::list_string_Cons(v15, v11));
            let v18: bool = true; v17 });
            let v20: Rc<list_string> = _vec_fold_;
            US6::US6_Listed(v20)
        }
    }
}
fn closure3(mut v0: Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>> {
    Rc::new(move |mut v1: Rc<str>| -> Rc<list_node> {
        walk_13(v0.clone(), v1)
    })
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>> = Rc::new(move |mut v0: Rc<str>| -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>> {
        closure3(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method16(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method15(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let mut v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method16(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn is_link_18(mut v0: Rc<str>) -> bool {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        false
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: bool = v4.file_type().is_symlink();
        v5
    }
}
fn directory_exists_20(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn is_directory_19(mut v0: Rc<str>) -> bool {
    directory_exists_20(v0)
}
fn file_exists_22(mut v0: Rc<str>) -> bool {
    let mut v2: &str = &*v0;
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
fn present_21(mut v0: Rc<str>) -> bool {
    file_exists_22(v0)
}
fn bytes_of_23(mut v0: Rc<str>) -> i64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        0i64
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let mut v5: u64 = v4.len();
        let mut v6: i64 = (v5 as i64);
        v6
    }
}
fn node_at_17(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<str>, mut v4: Rc<list_node>) -> Rc<list_node> {
    let mut v5: bool = is_link_18(v1.clone());
    if v5 {
        v4
    } else {
        let mut v6: bool = is_directory_19(v1.clone());
        if v6 {
            let mut v7: Rc<dyn Fn(Rc<str>) -> Rc<list_node>> = v0(v1);
            let v8: Rc<list_node> = v7(v2.clone());
            let v9: Rc<node> = Rc::new(node::node_Folder(v3, v2, v8));
            Rc::new(list_node::list_node_Cons(v9, v4))
        } else {
            let v11: bool = present_21(v1.clone());
            if v11 {
                let v12: i64 = bytes_of_23(v1);
                let v13: Rc<node> = Rc::new(node::node_File(v3, v2, v12));
                Rc::new(list_node::list_node_Cons(v13, v4))
            } else {
                v4
            }
        }
    }
}
fn nodes_of_14(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<list_string>, mut v4: Rc<list_node>) -> Rc<list_node> {
    loop {
        match &*v3 {
            list_string::list_string_Cons(v5, v6) => {
                let mut v5: Rc<str> = v5.clone();
                let mut v6: Rc<list_string> = v6.clone();
                let mut v7: Rc<str> = method15(v1.clone(), v5.clone());
                let v8: Rc<str> = html_pages::html_pages_rel(&v2, &v5);
                let v9: Rc<list_node> = node_at_17(v0.clone(), v7, v8, v5, v4);
                (v3, v4) = (v6, v9);
                continue;
            }
            list_string::list_string_Nil => {
                return v4;
            }
        }
    }
}
fn unreadable_text_24(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html: cannot read "); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(": "); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v4)), v1));
    v6
}
fn walk_13(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<list_node> {
    let mut v2: US6 = names_of_10(v0.clone());
    match &v2 {
        US6::US6_Listed(v3) => {
            let mut v3: Rc<list_string> = v3.clone();
            let mut v4: Rc<dyn Fn(Rc<str>) -> Rc<dyn Fn(Rc<str>) -> Rc<list_node>>> = closure2();
            let mut v5: Rc<list_node> = { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) };
            let mut v6: Rc<list_node> = nodes_of_14(v4, v0, v1, v3, v5);
            let mut v7: Rc<list_node> = html_pages::html_pages_sorted(v6);
            v7
        }
        US6::US6_Unreadable(v8) => {
            let v8: Rc<str> = v8.clone();
            let v9: Rc<str> = unreadable_text_24(v0, v8);
            eprintln!("{}", v9);
            { thread_local!{ static CASE: Rc<list_node> = Rc::new(list_node::list_node_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn run_of_31(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
    let mut v3: bool = v2 <= v1;
    if v3 {
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v4
    } else {
        let mut v5: i32 = v2.wrapping_sub(1i32);
        let mut v6: Rc<str> = string_slice(&v0, v1 as i64, v5 as i64);
        v6
    }
}
fn drop_final_newline_30(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.ends_with(&*v1);
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = (v0.clone().len() as i32);
        let mut v5: i32 = v4.wrapping_sub(1i32);
        run_of_31(v0, v3, v5)
    } else {
        v0
    }
}
fn method32(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn slurp_29(mut v0: Rc<str>) -> Rc<str> {
    let mut v2: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let mut v4: Vec<u8> = v2.unwrap();
    let mut v5: Vec<u8> = method32(v4);
    let mut v7: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v5);
    let v9: std::string::String = v7.unwrap();
    let v11: Rc<str> = Rc::<str>::from(String::as_str(&v9));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v13));
    drop_final_newline_30(v14)
}
fn put_33(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    std::fs::write(&*v0, &*v1).unwrap();
    ()
}
fn put_page_28(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: viewer) -> i32 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
    let mut v5: Rc<str> = slurp_29(v0);
    let mut v6: Rc<str> = html_pages::html_pages_file(&v1, v2, &v5);
    put_33(v4, v6);
    1i32
}
fn write_file_page_27(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: viewer = html_pages::html_pages_viewer(&v1);
    match &v2 {
        viewer::viewer_Direct => {
            0i32
        }
        _ => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
            let mut v5: bool = present_21(v4);
            if v5 {
                0i32
            } else {
                put_page_28(v0, v1, v2)
            }
        }
    }
}
fn page_at_26(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> i32 {
    let mut v3: Rc<str> = method15(v0, v1);
    write_file_page_27(v3, v2)
}
fn write_file_pages_25(mut v0: Rc<str>, mut v1: Rc<list_node>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            list_node::list_node_Cons(v3, v4) => {
                let mut v3: Rc<node> = v3.clone();
                let mut v4: Rc<list_node> = v4.clone();
                match &*v3 {
                    node::node_File(v5, v6, v7) => {
                        let mut v5: Rc<str> = v5.clone();
                        let mut v6: Rc<str> = v6.clone();
                        let mut v7: i64 = *v7;
                        let v8: i32 = page_at_26(v0.clone(), v6, v5);
                        let v9: i32 = v2.wrapping_add(v8);
                        (v1, v2) = (v4, v9);
                        continue;
                    }
                    node::node_Folder(v11, v12, v13) => {
                        let v11: Rc<str> = v11.clone();
                        let v12: Rc<str> = v12.clone();
                        let v13: Rc<list_node> = v13.clone();
                        let mut v14: i32 = write_file_pages_25(v0.clone(), v13, v2);
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
fn readme_of_34(mut v0: Rc<str>) -> (bool, Rc<str>) {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method15(v0, v1);
    let mut v3: bool = present_21(v2.clone());
    if v3 {
        let mut v4: Rc<str> = slurp_29(v2);
        (true, v4)
    } else {
        let mut v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        (false, v5)
    }
}
fn render_tree_9(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> US5 {
    let mut v3: US6 = names_of_10(v0.clone());
    match &v3 {
        US6::US6_Listed(v7) => {
            let mut v7: Rc<list_string> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<list_node> = walk_13(v0.clone(), v8);
            let v12: i32 = if v2 {
                let v10: i32 = 0i32;
                write_file_pages_25(v0.clone(), v9.clone(), v10)
            } else {
                0i32
            };
            let (mut v13, mut v14): (bool, Rc<str>) = readme_of_34(v0);
            let v15: Rc<str> = html_pages::html_pages_index(v2, v9, v13, &v14);
            put_33(v1, v15);
            US5::US5_Wrote(v12)
        }
        US6::US6_Unreadable(v4) => {
            let mut v4: Rc<str> = v4.clone();
            let mut v5: Rc<str> = unreadable_text_24(v0, v4);
            US5::US5_Refused(v5)
        }
    }
}
fn get_temp_path_37() -> Rc<str> {
    let mut v1: std::path::PathBuf = std::env::temp_dir();
    let mut v2: std::path::PathBuf = method16(v1);
    let mut v4: std::path::Display = v2.display();
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
fn go_38(mut v0: i32, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 <= 0i32;
        if v3 {
            return v2;
        } else {
            let mut v4: u64 = v1 & 15u64;
            let mut v5: i32 = (v4 as i32);
            let mut v6: i32 = v0.wrapping_sub(1i32);
            let mut v7: u64 = v1.wrapping_shr((4i32) as u32);
            let v8: Rc<str> = string_slice(&{ thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789abcdef"); } LIT.with(|lit| lit.clone()) }, v5 as i64, v5 as i64);
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
            (v0, v1, v2) = (v6, v7, v9);
            continue;
        }
    }
}
fn fixture_directory_36() -> Rc<str> {
    let mut v0: Rc<str> = get_temp_path_37();
    let mut v1: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
    let mut v3: bool = true; // 1;
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
    let v17: Rc<str> = go_38(v15, v14, v16.clone());
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: u64 = v12.wrapping_shr((32i32) as u32);
    let v21: u64 = v20 & 65535u64;
    let v22: i32 = 4i32;
    let v23: Rc<str> = go_38(v22, v21, v16.clone());
    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v23));
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-4"); } LIT.with(|lit| lit.clone()) };
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", v24, v25));
    let v27: u64 = v12.wrapping_shr((48i32) as u32);
    let v28: u64 = v27 & 4095u64;
    let v29: i32 = 3i32;
    let v30: Rc<str> = go_38(v29, v28, v16.clone());
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v26, v30)), v18));
    let v33: u64 = v13 & 16383u64;
    let v34: u64 = v33 | 32768u64;
    let v35: i32 = 4i32;
    let v36: Rc<str> = go_38(v35, v34, v16.clone());
    let v38: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v32, v36)), v18));
    let v39: u64 = v13.wrapping_shr((16i32) as u32);
    let v40: u64 = v39 & 281474976710655u64;
    let v41: i32 = 12i32;
    let v42: Rc<str> = go_38(v41, v40, v16);
    let v43: Rc<str> = Rc::<str>::from(format!("{}{}", v38, v42));
    let v44: Rc<str> = Rc::<str>::from(v43.to_lowercase());
    let v45: Rc<str> = v44 ;
    let v46: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html-self-test-"); } LIT.with(|lit| lit.clone()) };
    let v47: Rc<str> = Rc::<str>::from(format!("{}{}", v46, v45));
    method15(v0, v47)
}
fn make_directory_40(mut v0: Rc<str>) -> () {
    std::fs::create_dir_all(&*v0).unwrap();
    ()
}
fn times_41(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let mut v2: bool = v1 <= 0i64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let mut v4: i64 = v1.wrapping_sub(1i64);
        let mut v5: Rc<str> = times_41(v0.clone(), v4);
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
        v6
    }
}
fn lay_39(mut v0: Rc<str>, mut v1: i64) -> () {
    loop {
        let mut v2: bool = v1 >= 0i64;
        if v2 {
            make_directory_40(v0.clone());
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = method15(v0.clone(), v3);
            let mut v5: Rc<str> = html_pages::html_pages_digit(v1);
            let mut v6: i64 = v1.wrapping_add(1i64);
            let mut v7: Rc<str> = times_41(v5, v6);
            put_33(v4, v7);
            let v8: Rc<str> = html_pages::html_pages_digit(v1);
            let v9: Rc<str> = method15(v0, v8);
            let v10: i64 = v1.wrapping_sub(1i64);
            (v0, v1) = (v9, v10);
            continue;
        }
        return ();
    }
}
fn viewer_fixture_42() -> Rc<str> {
    let mut v0: Rc<str> = fixture_directory_36();
    make_directory_40(v0.clone());
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method15(v0.clone(), v1);
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("x < 1"); } LIT.with(|lit| lit.clone()) };
    put_33(v2, v3);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi.html"); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<str> = method15(v0.clone(), v4);
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("kept"); } LIT.with(|lit| lit.clone()) };
    put_33(v5, v6);
    let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = method15(v0.clone(), v7);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1\th2\nv1\tv2"); } LIT.with(|lit| lit.clone()) };
    put_33(v8, v9);
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c.txt"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = method15(v0.clone(), v10);
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("t"); } LIT.with(|lit| lit.clone()) };
    put_33(v11, v12);
    v0
}
fn viewers_kept_43(mut v0: Rc<str>) -> bool {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<list_node> = walk_13(v0.clone(), v1);
    let mut v3: i32 = 0i32;
    let mut v4: i32 = write_file_pages_25(v0.clone(), v2, v3);
    let mut v5: bool = v4 == 1i32;
    let v9: bool = if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("b.tsv.html"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = method15(v0.clone(), v6);
        present_21(v7)
    } else {
        false
    };
    let mut v14: bool = if v9 {
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a.spi.html"); } LIT.with(|lit| lit.clone()) };
        let v11: Rc<str> = method15(v0.clone(), v10);
        let v12: Rc<str> = slurp_29(v11);
        let v13: bool = &*v12 == "kept";
        v13
    } else {
        false
    };
    if v14 {
        let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c.txt.html"); } LIT.with(|lit| lit.clone()) };
        let v16: Rc<str> = method15(v0, v15);
        let v17: bool = present_21(v16);
        let v18: bool = v17 == false;
        v18
    } else {
        false
    }
}
fn missing_refused_44() -> bool {
    let mut v0: Rc<str> = fixture_directory_36();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
    let mut v3: bool = false;
    let mut v4: US5 = render_tree_9(v0.clone(), v2, v3);
    match &v4 {
        US5::US5_Refused(v5) => {
            let mut v5: Rc<str> = v5.clone();
            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v1));
            let mut v7: bool = present_21(v6);
            let v8: bool = v7 == false;
            v8
        }
        _ => {
            false
        }
    }
}
fn self_test_35() -> US5 {
    let mut v0: Rc<str> = fixture_directory_36();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_.root"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method15(v0.clone(), v1);
    let mut v3: i64 = 3i64;
    lay_39(v2, v3);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v5: Rc<list_node> = walk_13(v0, v4);
    let mut v6: Rc<str> = html_pages::html_pages_tree(false, v5);
    let mut v7: Rc<str> = html_pages::html_pages_fixture();
    let v8: bool = &*v6 != &*v7;
    if v8 {
        US5::US5_Mismatch(v6)
    } else {
        let v10: Rc<str> = viewer_fixture_42();
        let v11: bool = viewers_kept_43(v10);
        let v12: bool = v11 == false;
        if v12 {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("viewers: want b.tsv.html written, a.spi.html kept, no c.txt.html"); } LIT.with(|lit| lit.clone()) };
            US5::US5_Mismatch(v13)
        } else {
            let v15: bool = missing_refused_44();
            let v16: bool = v15 == false;
            if v16 {
                let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("walk: an unreadable folder must be refused, not drawn empty"); } LIT.with(|lit| lit.clone()) };
                US5::US5_Mismatch(v17)
            } else {
                US5::US5_Wrote(0i32)
            }
        }
    }
}
fn run_8(mut v0: US1) -> US5 {
    match &v0 {
        US1::US1_BadArgs(v6) => {
            let mut v6: Rc<str> = v6.clone();
            US5::US5_Refused(v6)
        }
        US1::US1_Render(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let mut v3: bool = *v3;
            render_tree_9(v1, v2, v3)
        }
        US1::US1_SelfTest => {
            self_test_35()
        }
    }
}
fn exit_of_45(mut v0: US5) -> i32 {
    match &v0 {
        US5::US5_Mismatch(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html self-test mismatch: "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
            eprintln!("{}", v4);
            1i32
        }
        US5::US5_Refused(v5) => {
            let mut v5: Rc<str> = v5.clone();
            eprintln!("{}", v5);
            2i32
        }
        US5::US5_Wrote(v1) => {
            let mut v1: i32 = *v1;
            0i32
        }
    }
}
fn spiral_main() -> i32 {
    let mut v0: i32 = 0i32;
    let mut v1: Rc<list_string> = arguments_from_0(v0);
    let mut v2: US1 = mode_of_3(v1);
    let mut v3: US5 = run_8(v2);
    exit_of_45(v3)
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
