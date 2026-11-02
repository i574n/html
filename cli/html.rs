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
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(Rc<str>, Rc<UH0>),
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
    US3_UnknownVerb(Rc<str>, Rc<UH0>),
    US3_NoVerb(Rc<UH0>),
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
enum UH2 {
    UH2_Nil,
    UH2_Cons(Rc<UH1>, Rc<UH2>),
}
#[derive(Clone)]
enum UH1 {
    UH1_File(Rc<str>, Rc<str>, i64),
    UH1_Folder(Rc<str>, Rc<str>, Rc<UH2>),
}
enum US6 {
    US6_Ok(Vec<std::string::String>),
    US6_Error(std::io::Error),
}
#[derive(Clone)]
enum US7 {
    US7_Direct,
    US7_Source,
    US7_Delimited,
    US7_Prose,
}
#[derive(Clone)]
enum US8 {
    US8_Markup(Rc<str>),
}
#[derive(Clone)]
enum US9 {
    US9_Some(US8),
    US9_None,
}
#[derive(Clone)]
enum UH3 {
    UH3_IndexPage(US8),
}
fn command_line_argument_count_1() -> i32 {
    let v0: usize = std::env::args().count();
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
fn arguments_from_0(mut v0: i32) -> Rc<UH0> {
    let mut v1: bool = v0 >= 0i32;
    let mut v4: bool = if v1 {
        let mut v2: i32 = command_line_argument_count_1();
        let mut v3: bool = v0 < v2;
        v3
    } else {
        false
    };
    let v8: US0 = if v4 {
        let v5: Rc<str> = command_line_argument_2(v0);
        US0::US0_Some(v5)
    } else {
        US0::US0_None
    };
    match &v8 {
        US0::US0_None => {
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US0::US0_Some(v10) => {
            let v10: Rc<str> = v10.clone();
            let v11: i32 = v0.wrapping_add(1i32);
            let v12: Rc<UH0> = arguments_from_0(v11);
            Rc::new(UH0::UH0_Cons(v10, v12))
        }
    }
}
fn walk_4(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v2 == &*v0;
                if v4 {
                    return true;
                } else {
                    (v0, v1) = (v0.clone(), v3.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn value_after_5(mut v0: Rc<str>, mut v1: Rc<UH0>) -> US0 {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                match &*v3 {
                    UH0::UH0_Cons(v4, v5) => {
                        let mut v4: Rc<str> = v4.clone();
                        let v5: Rc<UH0> = v5.clone();
                        let mut v6: bool = &*v2 == &*v0;
                        if v6 {
                            return US0::US0_Some(v4.clone());
                        } else {
                            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v4.clone(), v5.clone()));
                            (v0, v1) = (v0.clone(), v8.clone());
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
fn method6(mut v0: Rc<UH0>, mut v1: Rc<str>) -> Rc<str> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v1 == "";
                let v8: Rc<str> = if v4 {
                    v2.clone()
                } else {
                    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" | "); } LIT.with(|lit| lit.clone()) };
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v5)), v2));
                    v7.clone()
                };
                (v0, v1) = (v3.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn mode_of_3(mut v0: Rc<UH0>) -> US1 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("self-test"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--self-test"); } LIT.with(|lit| lit.clone()) };
    let mut v4: bool = walk_4(v3, v0.clone());
    if v4 {
        US1::US1_SelfTest
    } else {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("help"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--help"); } LIT.with(|lit| lit.clone()) };
        let v8: bool = walk_4(v7, v0.clone());
        let v71: US2 = if v8 {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <text>"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir <text>"); } LIT.with(|lit| lit.clone()) };
            let v13: bool = &*v12 == "";
            let v15: Rc<str> = if v13 {
                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                v14
            } else {
                v12
            };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v1, v16)), v11));
            let v19: bool = &*v18 == "";
            let v25: Rc<str> = if v19 {
                v15.clone()
            } else {
                let v20: bool = &*v15 == "";
                if v20 {
                    v18.clone()
                } else {
                    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v15, v21)), v18));
                    v23
                }
            };
            let v26: US3 = US3::US3_Help(v25);
            US2::US2_Error(v26)
        } else {
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dir"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
            let v30: US0 = value_after_5(v29, v0.clone());
            let v36: US4 = match &v30 {
                US0::US0_None => {
                    let v33: US3 = US3::US3_MissingFlag(v28);
                    US4::US4_Some(v33)
                }
                US0::US0_Some(v31) => {
                    let v31: Rc<str> = v31.clone();
                    US4::US4_None
                }
            };
            let v48: US4 = match &v36 {
                US4::US4_None => {
                    let v38: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                    let v39: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                    let v40: US0 = value_after_5(v39, v0.clone());
                    match &v40 {
                        US0::US0_None => {
                            let v43: US3 = US3::US3_MissingFlag(v38);
                            US4::US4_Some(v43)
                        }
                        US0::US0_Some(v41) => {
                            let v41: Rc<str> = v41.clone();
                            US4::US4_None
                        }
                    }
                }
                US4::US4_Some(v37) => {
                    let v37: US3 = v37.clone();
                    v36
                }
            };
            match &v48 {
                US4::US4_None => {
                    let v51: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--dir"); } LIT.with(|lit| lit.clone()) };
                    let v52: US0 = value_after_5(v51, v0.clone());
                    let v58: Rc<str> = match &v52 {
                        US0::US0_None => {
                            let v54: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                            let v55: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --dir"); } LIT.with(|lit| lit.clone()) };
                            std::panic::panic_any::<std::string::String>(format!("{}", v55))
                        }
                        US0::US0_Some(v53) => {
                            let v53: Rc<str> = v53.clone();
                            v53
                        }
                    };
                    let v59: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html"); } LIT.with(|lit| lit.clone()) };
                    let v60: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--html"); } LIT.with(|lit| lit.clone()) };
                    let v61: US0 = value_after_5(v60, v0.clone());
                    let v67: Rc<str> = match &v61 {
                        US0::US0_None => {
                            let v63: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --"); } LIT.with(|lit| lit.clone()) };
                            let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("missing --html"); } LIT.with(|lit| lit.clone()) };
                            std::panic::panic_any::<std::string::String>(format!("{}", v64))
                        }
                        US0::US0_Some(v62) => {
                            let v62: Rc<str> = v62.clone();
                            v62
                        }
                    };
                    US2::US2_Ok(v58, v67)
                }
                US4::US4_Some(v49) => {
                    let v49: US3 = v49.clone();
                    US2::US2_Error(v49)
                }
            }
        };
        match &v71 {
            US2::US2_Error(v78) => {
                let v78: US3 = v78.clone();
                let v112: Rc<str> = match &v78 {
                    US3::US3_BadNumber(v84, v85) => {
                        let v84: Rc<str> = v84.clone();
                        let v85: Rc<str> = v85.clone();
                        let v86: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: --"); } LIT.with(|lit| lit.clone()) };
                        let v87: Rc<str> = Rc::<str>::from(format!("{}{}", v86, v84));
                        let v88: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" expects an integer, got '"); } LIT.with(|lit| lit.clone()) };
                        let v90: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v87, v88)), v85));
                        let v91: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'"); } LIT.with(|lit| lit.clone()) };
                        let v92: Rc<str> = Rc::<str>::from(format!("{}{}", v90, v91));
                        v92
                    }
                    US3::US3_Help(v107) => {
                        let v107: Rc<str> = v107.clone();
                        v107
                    }
                    US3::US3_MissingFlag(v79) => {
                        let v79: Rc<str> = v79.clone();
                        let v80: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: missing --"); } LIT.with(|lit| lit.clone()) };
                        let v81: Rc<str> = Rc::<str>::from(format!("{}{}", v80, v79));
                        let v82: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <value>"); } LIT.with(|lit| lit.clone()) };
                        let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                        v83
                    }
                    US3::US3_NoVerb(v102) => {
                        let v102: Rc<UH0> = v102.clone();
                        let v103: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v104: Rc<str> = method6(v102, v103);
                        let v105: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: no command given\nvalid: "); } LIT.with(|lit| lit.clone()) };
                        let v106: Rc<str> = Rc::<str>::from(format!("{}{}", v105, v104));
                        v106
                    }
                    US3::US3_UnknownVerb(v93, v94) => {
                        let v93: Rc<str> = v93.clone();
                        let v94: Rc<UH0> = v94.clone();
                        let v95: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("error: unknown command '"); } LIT.with(|lit| lit.clone()) };
                        let v96: Rc<str> = Rc::<str>::from(format!("{}{}", v95, v93));
                        let v97: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("'\nvalid: "); } LIT.with(|lit| lit.clone()) };
                        let v98: Rc<str> = Rc::<str>::from(format!("{}{}", v96, v97));
                        let v99: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                        let v100: Rc<str> = method6(v94, v99);
                        let v101: Rc<str> = Rc::<str>::from(format!("{}{}", v98, v100));
                        v101
                    }
                };
                US1::US1_BadArgs(v112)
            }
            US2::US2_Ok(v72, v73) => {
                let v72: Rc<str> = v72.clone();
                let v73: Rc<str> = v73.clone();
                let v74: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("viewers"); } LIT.with(|lit| lit.clone()) };
                let v75: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("--viewers"); } LIT.with(|lit| lit.clone()) };
                let v76: bool = walk_4(v75, v0);
                US1::US1_Render(v72, v73, v76)
            }
        }
    }
}
fn folder_rank_13(mut v0: Rc<UH1>) -> i32 {
    match &*v0 {
        UH1::UH1_File(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let mut v6: i64 = *v6;
            1i32
        }
        UH1::UH1_Folder(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH2> = v3.clone();
            0i32
        }
    }
}
fn node_name_14(mut v0: Rc<UH1>) -> Rc<str> {
    match &*v0 {
        UH1::UH1_File(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let mut v6: i64 = *v6;
            v4
        }
        UH1::UH1_Folder(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<UH2> = v3.clone();
            v1
        }
    }
}
fn compare_from_15(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v2 == v3;
        let mut v7: bool = if v4 {
            let v5: i32 = (v1.clone().len() as i32);
            let mut v6: bool = v2 == v5;
            v6
        } else {
            false
        };
        if v7 {
            return 0i32;
        } else {
            if v4 {
                return -1i32;
            } else {
                let v8: i32 = (v1.clone().len() as i32);
                let v9: bool = v2 == v8;
                if v9 {
                    return 1i32;
                } else {
                    let v10: u8 = v0.clone().as_bytes()[v2 as usize];
                    let v11: u8 = v1.clone().as_bytes()[v2 as usize];
                    let v12: bool = v10 == v11;
                    if v12 {
                        let v13: i32 = v2.wrapping_add(1i32);
                        (v0, v1, v2) = (v0.clone(), v1.clone(), v13);
                        continue;
                    } else {
                        let v15: u32 = u32::from(v10);
                        let v16: u32 = u32::from(v11);
                        let v17: bool = v15 < v16;
                        if v17 {
                            return -1i32;
                        } else {
                            return 1i32;
                        }
                    }
                }
            }
        }
    }
}
fn before_12(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> bool {
    let mut v2: i32 = folder_rank_13(v0.clone());
    let mut v3: i32 = folder_rank_13(v1.clone());
    let mut v4: bool = v2 == v3;
    if v4 {
        let v5: Rc<str> = node_name_14(v0.clone());
        let mut v6: Rc<str> = node_name_14(v1.clone());
        let mut v7: i32 = 0i32;
        let v8: i32 = compare_from_15(v5, v6, v7);
        let v9: bool = v8 < 0i32;
        v9
    } else {
        let v10: i32 = folder_rank_13(v0);
        let v11: i32 = folder_rank_13(v1);
        let v12: bool = v10 < v11;
        v12
    }
}
fn insert_node_11(mut v0: Rc<UH1>, mut v1: Rc<UH2>) -> Rc<UH2> {
    match &*v1 {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<UH1> = v2.clone();
            let mut v3: Rc<UH2> = v3.clone();
            let mut v4: bool = before_12(v0.clone(), v2.clone());
            if v4 {
                Rc::new(UH2::UH2_Cons(v0.clone(), v1))
            } else {
                let mut v6: Rc<UH2> = insert_node_11(v0.clone(), v3);
                Rc::new(UH2::UH2_Cons(v2, v6))
            }
        }
        UH2::UH2_Nil => {
            let v9: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
            Rc::new(UH2::UH2_Cons(v0, v9))
        }
    }
}
fn sort_nodes_10(mut v0: Rc<UH2>) -> Rc<UH2> {
    match &*v0 {
        UH2::UH2_Cons(v1, v2) => {
            let mut v1: Rc<UH1> = v1.clone();
            let mut v2: Rc<UH2> = v2.clone();
            let mut v3: Rc<UH2> = sort_nodes_10(v2);
            insert_node_11(v1, v3)
        }
        UH2::UH2_Nil => {
            { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn ok_c0() -> Rc<dyn Fn(Vec<std::string::String>) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Vec<std::string::String>) -> US6> = Rc::new(move |mut v0: Vec<std::string::String>| -> US6 {
        US6::US6_Ok(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method17() -> Rc<dyn Fn(Vec<std::string::String>) -> US6> {
    ok_c0()
}
fn error_c1() -> Rc<dyn Fn(std::io::Error) -> US6> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(std::io::Error) -> US6> = Rc::new(move |mut v0: std::io::Error| -> US6 {
        US6::US6_Error(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn method18() -> Rc<dyn Fn(std::io::Error) -> US6> {
    error_c1()
}
fn names_of_16(mut v0: Rc<str>) -> Rc<UH0> {
    let mut v2: Result<Vec<std::string::String>, std::io::Error> = std::fs::read_dir(&*v0).map(|entries| entries.filter_map(|entry| entry.ok().map(|entry| entry.file_name().to_string_lossy().into_owned())).collect::<Vec<std::string::String>>());
    let mut v3: Rc<dyn Fn(Vec<std::string::String>) -> US6> = method17();
    let mut v4: Rc<dyn Fn(std::io::Error) -> US6> = method18();
    let v5: US6 = match v2 { Ok(x) => v3(x), Err(e) => v4(e) };
    match v5 {
        US6::US6_Error(v21) => {
            let v21: std::io::Error = v21;
            { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) }
        }
        US6::US6_Ok(v6) => {
            let mut v6: Vec<std::string::String> = v6.clone();
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v9: bool = true; let _vec_fold_ = v6.into_iter().fold(v7, |acc, x| { //;
            let v11: Rc<UH0> = acc;
            let v13: std::string::String = x;
            let v15: Rc<str> = Rc::<str>::from(String::as_str(&v13));
            let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15.clone(), v11.clone()));
            let v18: bool = true; v17 });
            let v20: Rc<UH0> = _vec_fold_;
            v20.clone()
        }
    }
}
fn method21(mut v0: std::path::PathBuf) -> std::path::PathBuf {
    v0
}
fn method20(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v3: &str = &*v0;
    let v5: std::string::String = String::from(v3);
    let mut v7: std::path::PathBuf = std::path::PathBuf::from(v5);
    let v9: &str = &*v1;
    let v11: std::string::String = String::from(v9);
    let v13: std::path::PathBuf = v7.join(v11);
    let mut v14: std::path::PathBuf = method21(v13);
    let v16: std::path::Display = v14.display();
    let v18: std::string::String = format!("{}", v16);
    let v20: Rc<str> = Rc::<str>::from(String::as_str(&v18));
    v20
}
fn join_rel_22(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let mut v2: bool = &*v0 == "";
    if v2 {
        v1.clone()
    } else {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("/"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v0, v3)), v1));
        v5
    }
}
fn is_link_24(mut v0: Rc<str>) -> bool {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::symlink_metadata(&*v0);
    let mut v2: bool = v1.is_ok();
    let mut v3: bool = v2 == false;
    if v3 {
        false
    } else {
        let mut v4: std::fs::Metadata = v1.unwrap();
        let v5: bool = v4.file_type().is_symlink();
        v5
    }
}
fn directory_exists_26(mut v0: Rc<str>) -> bool {
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
fn is_dir_25(mut v0: Rc<str>) -> bool {
    directory_exists_26(v0)
}
fn file_exists_28(mut v0: Rc<str>) -> bool {
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
fn present_27(mut v0: Rc<str>) -> bool {
    file_exists_28(v0)
}
fn bytes_of_29(mut v0: Rc<str>) -> i64 {
    let mut v1: std::io::Result<std::fs::Metadata> = std::fs::metadata(&*v0);
    let mut v2: bool = v1.is_ok();
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
fn node_at_23(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: Rc<UH2>) -> Rc<UH2> {
    let mut v4: bool = is_link_24(v0.clone());
    if v4 {
        v3.clone()
    } else {
        let v5: bool = is_dir_25(v0.clone());
        if v5 {
            let mut v6: Rc<UH2> = walk_9(v0.clone(), v1.clone());
            let mut v7: Rc<UH1> = Rc::new(UH1::UH1_Folder(v2.clone(), v1.clone(), v6));
            Rc::new(UH2::UH2_Cons(v7, v3.clone()))
        } else {
            let v9: bool = present_27(v0.clone());
            if v9 {
                let v10: i64 = bytes_of_29(v0);
                let v11: Rc<UH1> = Rc::new(UH1::UH1_File(v2, v1, v10));
                Rc::new(UH2::UH2_Cons(v11, v3.clone()))
            } else {
                v3
            }
        }
    }
}
fn nodes_of_19(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH0>, mut v3: Rc<UH2>) -> Rc<UH2> {
    loop {
        match &*v2 {
            UH0::UH0_Cons(v4, v5) => {
                let mut v4: Rc<str> = v4.clone();
                let v5: Rc<UH0> = v5.clone();
                let mut v6: Rc<str> = method20(v0.clone(), v4.clone());
                let mut v7: Rc<str> = join_rel_22(v1.clone(), v4.clone());
                let v8: Rc<UH2> = node_at_23(v6.clone(), v7.clone(), v4.clone(), v3.clone());
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v5.clone(), v8.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v3.clone();
            }
        }
    }
}
fn walk_9(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<UH2> {
    let mut v2: Rc<UH0> = names_of_16(v0.clone());
    let mut v3: Rc<UH2> = { thread_local!{ static CASE: Rc<UH2> = Rc::new(UH2::UH2_Nil); } CASE.with(|case| case.clone()) };
    let mut v4: Rc<UH2> = nodes_of_19(v0, v1, v2, v3);
    sort_nodes_10(v4)
}
fn run_of_34(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
fn last_dot_35(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> i32 {
    loop {
        let mut v3: i32 = (v0.clone().len() as i32);
        let mut v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let mut v6: bool = v5 == b'.';
            if v6 {
                let mut v7: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v7, v1);
                continue;
            } else {
                let v9: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2) = (v0.clone(), v9, v2);
                continue;
            }
        }
    }
}
fn extension_33(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = -1i32;
    let mut v3: i32 = last_dot_35(v0.clone(), v1, v2);
    let mut v4: bool = v3 <= 0i32;
    if v4 {
        let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v5
    } else {
        let mut v6: i32 = v3.wrapping_add(1i32);
        let mut v7: i32 = (v0.clone().len() as i32);
        run_of_34(v0, v6, v7)
    }
}
fn listed_36(mut v0: Rc<str>, mut v1: Rc<UH0>) -> bool {
    loop {
        match &*v1 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v2 == &*v0;
                if v4 {
                    return true;
                } else {
                    (v0, v1) = (v0.clone(), v3.clone());
                    continue;
                }
            }
            UH0::UH0_Nil => {
                return false;
            }
        }
    }
}
fn viewer_of_32(mut v0: Rc<str>) -> US7 {
    let mut v1: Rc<str> = extension_33(v0);
    let mut v2: bool = &*v1 == "tsv";
    if v2 {
        US7::US7_Delimited
    } else {
        let mut v4: bool = &*v1 == "md";
        if v4 {
            US7::US7_Prose
        } else {
            let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spi"); } LIT.with(|lit| lit.clone()) };
            let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spir"); } LIT.with(|lit| lit.clone()) };
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("spiproj"); } LIT.with(|lit| lit.clone()) };
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("toml"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ps1"); } LIT.with(|lit| lit.clone()) };
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("psm1"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fs"); } LIT.with(|lit| lit.clone()) };
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsx"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("fsproj"); } LIT.with(|lit| lit.clone()) };
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("rs"); } LIT.with(|lit| lit.clone()) };
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("gleam"); } LIT.with(|lit| lit.clone()) };
            let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ex"); } LIT.with(|lit| lit.clone()) };
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("exs"); } LIT.with(|lit| lit.clone()) };
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("zig"); } LIT.with(|lit| lit.clone()) };
            let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("c"); } LIT.with(|lit| lit.clone()) };
            let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h"); } LIT.with(|lit| lit.clone()) };
            let v22: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cpp"); } LIT.with(|lit| lit.clone()) };
            let v23: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cs"); } LIT.with(|lit| lit.clone()) };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("py"); } LIT.with(|lit| lit.clone()) };
            let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sh"); } LIT.with(|lit| lit.clone()) };
            let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cmd"); } LIT.with(|lit| lit.clone()) };
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("bat"); } LIT.with(|lit| lit.clone()) };
            let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("lock"); } LIT.with(|lit| lit.clone()) };
            let v29: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("cfg"); } LIT.with(|lit| lit.clone()) };
            let v30: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("ini"); } LIT.with(|lit| lit.clone()) };
            let v31: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("sql"); } LIT.with(|lit| lit.clone()) };
            let v32: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("csv"); } LIT.with(|lit| lit.clone()) };
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("dib"); } LIT.with(|lit| lit.clone()) };
            let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("livemd"); } LIT.with(|lit| lit.clone()) };
            let v35: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v36: Rc<UH0> = Rc::new(UH0::UH0_Cons(v34, v35));
            let v37: Rc<UH0> = Rc::new(UH0::UH0_Cons(v33, v36));
            let v38: Rc<UH0> = Rc::new(UH0::UH0_Cons(v32, v37));
            let v39: Rc<UH0> = Rc::new(UH0::UH0_Cons(v31, v38));
            let v40: Rc<UH0> = Rc::new(UH0::UH0_Cons(v30, v39));
            let v41: Rc<UH0> = Rc::new(UH0::UH0_Cons(v29, v40));
            let v42: Rc<UH0> = Rc::new(UH0::UH0_Cons(v28, v41));
            let v43: Rc<UH0> = Rc::new(UH0::UH0_Cons(v27, v42));
            let v44: Rc<UH0> = Rc::new(UH0::UH0_Cons(v26, v43));
            let v45: Rc<UH0> = Rc::new(UH0::UH0_Cons(v25, v44));
            let v46: Rc<UH0> = Rc::new(UH0::UH0_Cons(v24, v45));
            let v47: Rc<UH0> = Rc::new(UH0::UH0_Cons(v23, v46));
            let v48: Rc<UH0> = Rc::new(UH0::UH0_Cons(v22, v47));
            let v49: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21, v48));
            let v50: Rc<UH0> = Rc::new(UH0::UH0_Cons(v20, v49));
            let v51: Rc<UH0> = Rc::new(UH0::UH0_Cons(v19, v50));
            let v52: Rc<UH0> = Rc::new(UH0::UH0_Cons(v18, v51));
            let v53: Rc<UH0> = Rc::new(UH0::UH0_Cons(v17, v52));
            let v54: Rc<UH0> = Rc::new(UH0::UH0_Cons(v16, v53));
            let v55: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15, v54));
            let v56: Rc<UH0> = Rc::new(UH0::UH0_Cons(v14, v55));
            let v57: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13, v56));
            let v58: Rc<UH0> = Rc::new(UH0::UH0_Cons(v12, v57));
            let v59: Rc<UH0> = Rc::new(UH0::UH0_Cons(v11, v58));
            let v60: Rc<UH0> = Rc::new(UH0::UH0_Cons(v10, v59));
            let v61: Rc<UH0> = Rc::new(UH0::UH0_Cons(v9, v60));
            let v62: Rc<UH0> = Rc::new(UH0::UH0_Cons(v8, v61));
            let v63: Rc<UH0> = Rc::new(UH0::UH0_Cons(v7, v62));
            let v64: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6, v63));
            let v65: bool = listed_36(v1, v64);
            if v65 {
                US7::US7_Source
            } else {
                US7::US7_Direct
            }
        }
    }
}
fn drop_final_newline_38(mut v0: Rc<str>) -> Rc<str> {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: bool = v0.ends_with(&*v1);
    if v2 {
        let mut v3: i32 = 0i32;
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: i32 = v4.wrapping_sub(1i32);
        run_of_34(v0.clone(), v3, v5)
    } else {
        v0
    }
}
fn method39(mut v0: Vec<u8>) -> Vec<u8> {
    v0
}
fn slurp_37(mut v0: Rc<str>) -> Rc<str> {
    let mut v2: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v0);
    let mut v4: Vec<u8> = v2.unwrap();
    let v5: Vec<u8> = method39(v4);
    let mut v7: Result<std::string::String, std::string::FromUtf8Error> = std::string::String::from_utf8(v5);
    let v9: std::string::String = v7.unwrap();
    let v11: Rc<str> = Rc::<str>::from(String::as_str(&v9));
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\r\n"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = Rc::<str>::from(v11.replace(&*v12, &*v13));
    drop_final_newline_38(v14)
}
fn trusted_40(mut v0: Rc<str>) -> US8 {
    US8::US8_Markup(v0)
}
fn entity_43(mut v0: u8) -> Rc<str> {
    let mut v1: bool = v0 == b'&';
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&amp;"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let mut v3: bool = v0 == b'<';
        if v3 {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&lt;"); } LIT.with(|lit| lit.clone()) };
            v4
        } else {
            let v5: bool = v0 == b'>';
            if v5 {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&gt;"); } LIT.with(|lit| lit.clone()) };
                v6
            } else {
                let mut v7: bool = v0 == b'"';
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
fn escape_from_42(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<str> = run_of_34(v0.clone(), v2, v1);
            let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v6));
            return v7.clone();
        } else {
            let v8: u8 = v0.clone().as_bytes()[v1 as usize];
            let v9: Rc<str> = entity_43(v8);
            let v10: bool = &*v9 == "";
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v3.clone());
                continue;
            } else {
                let v13: i32 = v1.wrapping_add(1i32);
                let mut v14: Rc<str> = run_of_34(v0.clone(), v2, v1);
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v14)), v9));
                (v0, v1, v2, v3) = (v0.clone(), v13, v13, v16.clone());
                continue;
            }
        }
    }
}
fn escape_41(mut v0: Rc<str>) -> US8 {
    let mut v1: i32 = 0i32;
    let mut v2: i32 = 0i32;
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = escape_from_42(v0, v1, v2, v3);
    US8::US8_Markup(v4)
}
fn quoted_45(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let mut v2: US8 = trusted_40(v1.clone());
    let mut v3: US8 = escape_41(v0);
    let v9: US8 = match &v2 {
        US8::US8_Markup(v4) => {
            let mut v4: Rc<str> = v4.clone();
            match &v3 {
                US8::US8_Markup(v5) => {
                    let v5: Rc<str> = v5.clone();
                    let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
                    US8::US8_Markup(v6)
                }
            }
        }
    };
    let v10: US8 = trusted_40(v1);
    match &v9 {
        US8::US8_Markup(v11) => {
            let v11: Rc<str> = v11.clone();
            match &v10 {
                US8::US8_Markup(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                    US8::US8_Markup(v13)
                }
            }
        }
    }
}
fn anchor_44(mut v0: Rc<str>, mut v1: US8) -> US8 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
    let mut v3: US8 = trusted_40(v2);
    let mut v4: US8 = quoted_45(v0);
    let v10: US8 = match &v3 {
        US8::US8_Markup(v5) => {
            let v5: Rc<str> = v5.clone();
            match &v4 {
                US8::US8_Markup(v6) => {
                    let mut v6: Rc<str> = v6.clone();
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
                    US8::US8_Markup(v7)
                }
            }
        }
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v12: US8 = trusted_40(v11);
    let v18: US8 = match &v10 {
        US8::US8_Markup(v13) => {
            let v13: Rc<str> = v13.clone();
            match &v12 {
                US8::US8_Markup(v14) => {
                    let mut v14: Rc<str> = v14.clone();
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                    US8::US8_Markup(v15)
                }
            }
        }
    };
    let v24: US8 = match &v18 {
        US8::US8_Markup(v19) => {
            let v19: Rc<str> = v19.clone();
            match &v1 {
                US8::US8_Markup(v20) => {
                    let v20: Rc<str> = v20.clone();
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                    US8::US8_Markup(v21)
                }
            }
        }
    };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
    let v26: US8 = trusted_40(v25);
    match &v24 {
        US8::US8_Markup(v27) => {
            let v27: Rc<str> = v27.clone();
            match &v26 {
                US8::US8_Markup(v28) => {
                    let v28: Rc<str> = v28.clone();
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                    US8::US8_Markup(v29)
                }
            }
        }
    }
}
fn head_html_46() -> US8 {
    let v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n"); } LIT.with(|lit| lit.clone()) };
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("  <style>\n"); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\n"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}"); } LIT.with(|lit| lit.clone()) };
    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <style>\nhtml {\n  -webkit-text-size-adjust: 100%;\n  text-size-adjust: 100%;\n}\nbody {\n    background-color: #222;\n    color: #ccc;\n    margin: 8px;\n    line-height: 1.6;\n    overflow-wrap: anywhere;\n}\na {\n  color: #777;\n  font-size: 15px;\n}\nspan {\n  font-size: 11px;\n}\ndiv > div {\n  padding-left: 10px;\n}\ndetails > div {\n  padding-left: 19px;\n}\n.readme {\n  border-top: 1px solid #444;\n  margin-top: 16px;\n  padding-top: 8px;\n  max-width: 900px;\n}\n.readme a {\n  color: #9ab;\n}\n.readme pre {\n  background-color: #1a1a1a;\n  padding: 8px;\n  overflow-x: auto;\n}\n@media (max-width: 600px) {\n  a {\n    font-size: 16px;\n  }\n  span {\n    font-size: 12px;\n  }\n  div > div {\n    padding-left: 4px;\n  }\n  details > div {\n    padding-left: 10px;\n  }\n}"); } LIT.with(|lit| lit.clone()) };
    let mut v7: US8 = trusted_40(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n  </style>\n</head>\n<body>\n  <div>"); } LIT.with(|lit| lit.clone()) };
    let v9: US8 = trusted_40(v8);
    match &v7 {
        US8::US8_Markup(v10) => {
            let v10: Rc<str> = v10.clone();
            match &v9 {
                US8::US8_Markup(v11) => {
                    let v11: Rc<str> = v11.clone();
                    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v10, v11));
                    US8::US8_Markup(v12)
                }
            }
        }
    }
}
fn method48(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn lines_from_47(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH0>) -> Rc<UH0> {
    loop {
        let mut v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let mut v6: Rc<str> = run_of_34(v0.clone(), v2, v1);
            let mut v7: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH0> = Rc::new(UH0::UH0_Cons(v6.clone(), v7.clone()));
            return method48(v3.clone(), v8.clone());
        } else {
            let v10: u8 = v0.clone().as_bytes()[v1 as usize];
            let v11: bool = v10 == b'\n';
            if v11 {
                let v12: i32 = v1.wrapping_add(1i32);
                let v13: Rc<str> = run_of_34(v0.clone(), v2, v1);
                let mut v14: Rc<UH0> = Rc::new(UH0::UH0_Cons(v13.clone(), v3.clone()));
                (v0, v1, v2, v3) = (v0.clone(), v12, v12, v14.clone());
                continue;
            } else {
                let v16: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v16, v2, v3.clone());
                continue;
            }
        }
    }
}
fn wrap_51(mut v0: Rc<str>, mut v1: US8) -> US8 {
    let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let mut v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let mut v6: US8 = trusted_40(v5);
    let v12: US8 = match &v6 {
        US8::US8_Markup(v7) => {
            let mut v7: Rc<str> = v7.clone();
            match &v1 {
                US8::US8_Markup(v8) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                    US8::US8_Markup(v9)
                }
            }
        }
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v0)), v4));
    let v16: US8 = trusted_40(v15);
    match &v12 {
        US8::US8_Markup(v17) => {
            let v17: Rc<str> = v17.clone();
            match &v16 {
                US8::US8_Markup(v18) => {
                    let v18: Rc<str> = v18.clone();
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                    US8::US8_Markup(v19)
                }
            }
        }
    }
}
fn tab_cells_50(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: i32, mut v3: i32) -> US8 {
    loop {
        let mut v4: i32 = (v1.clone().len() as i32);
        let v5: bool = v2 >= v4;
        if v5 {
            let mut v6: Rc<str> = run_of_34(v1.clone(), v3, v2);
            let mut v7: US8 = escape_41(v6.clone());
            return wrap_51(v0.clone(), v7.clone());
        } else {
            let v9: u8 = v1.clone().as_bytes()[v2 as usize];
            let v10: bool = v9 == b'\t';
            if v10 {
                let v11: Rc<str> = run_of_34(v1.clone(), v3, v2);
                let v12: US8 = escape_41(v11.clone());
                let v13: US8 = wrap_51(v0.clone(), v12.clone());
                let mut v14: i32 = v2.wrapping_add(1i32);
                let v15: US8 = tab_cells_50(v0.clone(), v1.clone(), v14, v14);
                match &v13 {
                    US8::US8_Markup(v16) => {
                        let v16: Rc<str> = v16.clone();
                        match &v15 {
                            US8::US8_Markup(v17) => {
                                let v17: Rc<str> = v17.clone();
                                let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v17));
                                return US8::US8_Markup(v18.clone());
                            }
                        }
                    }
                }
            } else {
                let v22: i32 = v2.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v1.clone(), v22, v3);
                continue;
            }
        }
    }
}
fn tab_rows_49(mut v0: Rc<UH0>, mut v1: Rc<str>) -> US8 {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let mut v2: Rc<str> = v2.clone();
                let mut v3: Rc<UH0> = v3.clone();
                let mut v4: bool = &*v2 == "";
                if v4 {
                    (v0, v1) = (v3.clone(), v1.clone());
                    continue;
                } else {
                    let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<tr>"); } LIT.with(|lit| lit.clone()) };
                    let mut v7: US8 = trusted_40(v6.clone());
                    let v8: i32 = 0i32;
                    let v9: i32 = 0i32;
                    let v10: US8 = tab_cells_50(v1.clone(), v2.clone(), v8, v9);
                    let v16: US8 = match &v7 {
                        US8::US8_Markup(v11) => {
                            let v11: Rc<str> = v11.clone();
                            match &v10 {
                                US8::US8_Markup(v12) => {
                                    let v12: Rc<str> = v12.clone();
                                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                                    US8::US8_Markup(v13.clone())
                                }
                            }
                        }
                    };
                    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tr>\n"); } LIT.with(|lit| lit.clone()) };
                    let v18: US8 = trusted_40(v17.clone());
                    let v24: US8 = match &v16 {
                        US8::US8_Markup(v19) => {
                            let v19: Rc<str> = v19.clone();
                            match &v18 {
                                US8::US8_Markup(v20) => {
                                    let v20: Rc<str> = v20.clone();
                                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                                    US8::US8_Markup(v21.clone())
                                }
                            }
                        }
                    };
                    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
                    let v26: US8 = tab_rows_49(v3.clone(), v25.clone());
                    match &v24 {
                        US8::US8_Markup(v27) => {
                            let v27: Rc<str> = v27.clone();
                            match &v26 {
                                US8::US8_Markup(v28) => {
                                    let v28: Rc<str> = v28.clone();
                                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                                    return US8::US8_Markup(v29.clone());
                                }
                            }
                        }
                    }
                }
            }
            UH0::UH0_Nil => {
                let v34: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                return trusted_40(v34.clone());
            }
        }
    }
}
fn markdown_markup_52(mut v0: Rc<str>) -> US8 {
    let mut v1: Rc<str> = html_markdown::html_markdown(&*v0);
    trusted_40(v1)
}
fn close_html_53(mut v0: US8) -> US8 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</div>\n"); } LIT.with(|lit| lit.clone()) };
    let mut v2: US8 = trusted_40(v1);
    let v8: US8 = match &v2 {
        US8::US8_Markup(v3) => {
            let mut v3: Rc<str> = v3.clone();
            match &v0 {
                US8::US8_Markup(v4) => {
                    let mut v4: Rc<str> = v4.clone();
                    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
                    US8::US8_Markup(v5)
                }
            }
        }
    };
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</body>\n</html>\n"); } LIT.with(|lit| lit.clone()) };
    let v10: US8 = trusted_40(v9);
    match &v8 {
        US8::US8_Markup(v11) => {
            let v11: Rc<str> = v11.clone();
            match &v10 {
                US8::US8_Markup(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                    US8::US8_Markup(v13)
                }
            }
        }
    }
}
fn markup_text_54(mut v0: US8) -> Rc<str> {
    match &v0 {
        US8::US8_Markup(v1) => {
            let mut v1: Rc<str> = v1.clone();
            v1
        }
    }
}
fn put_55(mut v0: Rc<str>, mut v1: Rc<str>) -> () {
    std::fs::write(&*v0, &*v1).unwrap();
    ()
}
fn write_file_page_31(mut v0: Rc<str>, mut v1: Rc<str>) -> i32 {
    let mut v2: US7 = viewer_of_32(v1.clone());
    match &v2 {
        US7::US7_Direct => {
            0i32
        }
        _ => {
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
            let v5: bool = present_27(v4);
            if v5 {
                0i32
            } else {
                let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v3));
                let mut v7: Rc<str> = slurp_37(v0);
                let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) };
                let v9: US8 = trusted_40(v8);
                let v10: US8 = escape_41(v1.clone());
                let v11: US8 = anchor_44(v1, v10);
                let v17: US8 = match &v9 {
                    US8::US8_Markup(v12) => {
                        let v12: Rc<str> = v12.clone();
                        match &v11 {
                            US8::US8_Markup(v13) => {
                                let v13: Rc<str> = v13.clone();
                                let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                                US8::US8_Markup(v14)
                            }
                        }
                    }
                };
                let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" <span>raw</span></p>\n"); } LIT.with(|lit| lit.clone()) };
                let v19: US8 = trusted_40(v18);
                let v25: US8 = match &v17 {
                    US8::US8_Markup(v20) => {
                        let v20: Rc<str> = v20.clone();
                        match &v19 {
                            US8::US8_Markup(v21) => {
                                let v21: Rc<str> = v21.clone();
                                let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                                US8::US8_Markup(v22)
                            }
                        }
                    }
                };
                let v26: US8 = head_html_46();
                let v32: US8 = match &v26 {
                    US8::US8_Markup(v27) => {
                        let v27: Rc<str> = v27.clone();
                        match &v25 {
                            US8::US8_Markup(v28) => {
                                let v28: Rc<str> = v28.clone();
                                let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                                US8::US8_Markup(v29)
                            }
                        }
                    }
                };
                let v91: US8 = match &v2 {
                    US7::US7_Delimited => {
                        let v50: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n"); } LIT.with(|lit| lit.clone()) };
                        let v51: US8 = trusted_40(v50);
                        let v52: i32 = 0i32;
                        let v53: i32 = 0i32;
                        let v54: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        let v55: Rc<UH0> = lines_from_47(v7.clone(), v52, v53, v54);
                        let v56: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
                        let v57: US8 = tab_rows_49(v55, v56);
                        let v63: US8 = match &v51 {
                            US8::US8_Markup(v58) => {
                                let v58: Rc<str> = v58.clone();
                                match &v57 {
                                    US8::US8_Markup(v59) => {
                                        let v59: Rc<str> = v59.clone();
                                        let v60: Rc<str> = Rc::<str>::from(format!("{}{}", v58, v59));
                                        US8::US8_Markup(v60)
                                    }
                                }
                            }
                        };
                        let v64: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</table>\n"); } LIT.with(|lit| lit.clone()) };
                        let v65: US8 = trusted_40(v64);
                        match &v63 {
                            US8::US8_Markup(v66) => {
                                let v66: Rc<str> = v66.clone();
                                match &v65 {
                                    US8::US8_Markup(v67) => {
                                        let v67: Rc<str> = v67.clone();
                                        let v68: Rc<str> = Rc::<str>::from(format!("{}{}", v66, v67));
                                        US8::US8_Markup(v68)
                                    }
                                }
                            }
                        }
                    }
                    US7::US7_Prose => {
                        let v72: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
                        let v73: US8 = trusted_40(v72);
                        let v74: US8 = markdown_markup_52(v7.clone());
                        let v80: US8 = match &v73 {
                            US8::US8_Markup(v75) => {
                                let v75: Rc<str> = v75.clone();
                                match &v74 {
                                    US8::US8_Markup(v76) => {
                                        let v76: Rc<str> = v76.clone();
                                        let v77: Rc<str> = Rc::<str>::from(format!("{}{}", v75, v76));
                                        US8::US8_Markup(v77)
                                    }
                                }
                            }
                        };
                        let v81: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
                        let v82: US8 = trusted_40(v81);
                        match &v80 {
                            US8::US8_Markup(v83) => {
                                let v83: Rc<str> = v83.clone();
                                match &v82 {
                                    US8::US8_Markup(v84) => {
                                        let v84: Rc<str> = v84.clone();
                                        let v85: Rc<str> = Rc::<str>::from(format!("{}{}", v83, v84));
                                        US8::US8_Markup(v85)
                                    }
                                }
                            }
                        }
                    }
                    US7::US7_Source => {
                        let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
                        let v34: US8 = trusted_40(v33);
                        let v35: US8 = escape_41(v7);
                        let v41: US8 = match &v34 {
                            US8::US8_Markup(v36) => {
                                let v36: Rc<str> = v36.clone();
                                match &v35 {
                                    US8::US8_Markup(v37) => {
                                        let v37: Rc<str> = v37.clone();
                                        let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                                        US8::US8_Markup(v38)
                                    }
                                }
                            }
                        };
                        let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
                        let v43: US8 = trusted_40(v42);
                        match &v41 {
                            US8::US8_Markup(v44) => {
                                let v44: Rc<str> = v44.clone();
                                match &v43 {
                                    US8::US8_Markup(v45) => {
                                        let v45: Rc<str> = v45.clone();
                                        let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
                                        US8::US8_Markup(v46)
                                    }
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                let v97: US8 = match &v32 {
                    US8::US8_Markup(v92) => {
                        let v92: Rc<str> = v92.clone();
                        match &v91 {
                            US8::US8_Markup(v93) => {
                                let v93: Rc<str> = v93.clone();
                                let v94: Rc<str> = Rc::<str>::from(format!("{}{}", v92, v93));
                                US8::US8_Markup(v94)
                            }
                        }
                    }
                };
                let v98: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
                let v99: US8 = trusted_40(v98);
                let v100: US8 = close_html_53(v99);
                let v106: US8 = match &v97 {
                    US8::US8_Markup(v101) => {
                        let v101: Rc<str> = v101.clone();
                        match &v100 {
                            US8::US8_Markup(v102) => {
                                let v102: Rc<str> = v102.clone();
                                let v103: Rc<str> = Rc::<str>::from(format!("{}{}", v101, v102));
                                US8::US8_Markup(v103)
                            }
                        }
                    }
                };
                let v107: Rc<str> = markup_text_54(v106);
                put_55(v6, v107);
                1i32
            }
        }
    }
}
fn write_file_pages_30(mut v0: Rc<str>, mut v1: Rc<UH2>, mut v2: i32) -> i32 {
    loop {
        match &*v1 {
            UH2::UH2_Cons(v3, v4) => {
                let mut v3: Rc<UH1> = v3.clone();
                let mut v4: Rc<UH2> = v4.clone();
                match &*v3 {
                    UH1::UH1_File(v5, v6, v7) => {
                        let v5: Rc<str> = v5.clone();
                        let mut v6: Rc<str> = v6.clone();
                        let mut v7: i64 = *v7;
                        let v8: Rc<str> = method20(v0.clone(), v6.clone());
                        let v9: i32 = write_file_page_31(v8.clone(), v5.clone());
                        let v10: i32 = v2.wrapping_add(v9);
                        (v0, v1, v2) = (v0.clone(), v4.clone(), v10);
                        continue;
                    }
                    UH1::UH1_Folder(v12, v13, v14) => {
                        let v12: Rc<str> = v12.clone();
                        let v13: Rc<str> = v13.clone();
                        let mut v14: Rc<UH2> = v14.clone();
                        let v15: i32 = write_file_pages_30(v0.clone(), v14.clone(), v2);
                        (v0, v1, v2) = (v0.clone(), v4.clone(), v15);
                        continue;
                    }
                }
            }
            UH2::UH2_Nil => {
                return v2;
            }
        }
    }
}
fn readme_of_56(mut v0: Rc<str>) -> US9 {
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("README.md"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method20(v0, v1);
    let mut v3: bool = present_27(v2.clone());
    if v3 {
        let mut v4: Rc<str> = slurp_37(v2);
        let v5: US8 = markdown_markup_52(v4);
        US9::US9_Some(v5)
    } else {
        US9::US9_None
    }
}
fn digit_65(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 == 0i64;
    if v1 {
        let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let mut v3: bool = v0 == 1i64;
        if v3 {
            let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
            v4
        } else {
            let v5: bool = v0 == 2i64;
            if v5 {
                let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
                v6
            } else {
                let mut v7: bool = v0 == 3i64;
                if v7 {
                    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("3"); } LIT.with(|lit| lit.clone()) };
                    v8
                } else {
                    let v9: bool = v0 == 4i64;
                    if v9 {
                        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("4"); } LIT.with(|lit| lit.clone()) };
                        v10
                    } else {
                        let v11: bool = v0 == 5i64;
                        if v11 {
                            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("5"); } LIT.with(|lit| lit.clone()) };
                            v12
                        } else {
                            let v13: bool = v0 == 6i64;
                            if v13 {
                                let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
                                v14
                            } else {
                                let v15: bool = v0 == 7i64;
                                if v15 {
                                    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("7"); } LIT.with(|lit| lit.clone()) };
                                    v16
                                } else {
                                    let v17: bool = v0 == 8i64;
                                    if v17 {
                                        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("8"); } LIT.with(|lit| lit.clone()) };
                                        v18
                                    } else {
                                        let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("9"); } LIT.with(|lit| lit.clone()) };
                                        v19
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
fn show_i64_64(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 10i64;
    if v1 {
        digit_65(v0)
    } else {
        let mut v3: i64 = v0.wrapping_div(10i64);
        let mut v4: Rc<str> = show_i64_64(v3);
        let v5: i64 = v0.wrapping_rem(10i64);
        let mut v6: Rc<str> = digit_65(v5);
        let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
        v7
    }
}
fn pad2_66(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 < 10i64;
    if v1 {
        let mut v2: Rc<str> = show_i64_64(v0);
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
        v4
    } else {
        show_i64_64(v0)
    }
}
fn hundredths_63(mut v0: i64, mut v1: i64) -> Rc<str> {
    let mut v2: i64 = v0.wrapping_mul(100i64);
    let mut v3: i64 = v1.wrapping_div(2i64);
    let mut v4: i64 = v2.wrapping_add(v3);
    let v5: i64 = v4.wrapping_div(v1);
    let mut v6: i64 = v5.wrapping_div(100i64);
    let mut v7: Rc<str> = show_i64_64(v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("."); } LIT.with(|lit| lit.clone()) };
    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
    let v10: i64 = v5.wrapping_rem(100i64);
    let v11: Rc<str> = pad2_66(v10);
    let v12: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v11));
    v12
}
fn size_label_62(mut v0: i64) -> Rc<str> {
    let mut v1: bool = v0 > 1048576i64;
    if v1 {
        let mut v2: i64 = 1048576i64;
        let mut v3: Rc<str> = hundredths_63(v0, v2);
        let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" MB"); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
        v5
    } else {
        let mut v6: bool = v0 > 1024i64;
        if v6 {
            let mut v7: i64 = 1024i64;
            let v8: Rc<str> = hundredths_63(v0, v7);
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" KB"); } LIT.with(|lit| lit.clone()) };
            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
            v10
        } else {
            let v11: i64 = 1i64;
            let v12: Rc<str> = hundredths_63(v0, v11);
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" B"); } LIT.with(|lit| lit.clone()) };
            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
            v14
        }
    }
}
fn entry_link_61(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i64) -> US8 {
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v4));
    let mut v6: US8 = trusted_40(v5);
    let mut v7: US8 = escape_41(v2);
    let v8: US8 = anchor_44(v1, v7);
    let mut v14: US8 = match &v6 {
        US8::US8_Markup(v9) => {
            let v9: Rc<str> = v9.clone();
            match &v8 {
                US8::US8_Markup(v10) => {
                    let v10: Rc<str> = v10.clone();
                    let v11: Rc<str> = Rc::<str>::from(format!("{}{}", v9, v10));
                    US8::US8_Markup(v11)
                }
            }
        }
    };
    let v15: Rc<str> = size_label_62(v3);
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<span> ("); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v16, v15));
    let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(")</span>"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
    let v20: US8 = trusted_40(v19);
    match &v14 {
        US8::US8_Markup(v21) => {
            let v21: Rc<str> = v21.clone();
            match &v20 {
                US8::US8_Markup(v22) => {
                    let v22: Rc<str> = v22.clone();
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                    US8::US8_Markup(v23)
                }
            }
        }
    }
}
fn has_viewer_67(mut v0: Rc<str>) -> bool {
    let mut v1: US7 = viewer_of_32(v0);
    match &v1 {
        US7::US7_Direct => {
            false
        }
        _ => {
            true
        }
    }
}
fn file_html_60(mut v0: bool, mut v1: Rc<str>, mut v2: Rc<str>, mut v3: i64) -> US8 {
    let v5: bool = if v0 {
        has_viewer_67(v1.clone())
    } else {
        false
    };
    if v5 {
        let mut v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196;"); } LIT.with(|lit| lit.clone()) };
        let mut v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(".html"); } LIT.with(|lit| lit.clone()) };
        let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v7));
        let v9: US8 = entry_link_61(v6, v8, v1.clone(), v3);
        let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
        let v11: US8 = trusted_40(v10);
        let v17: US8 = match &v9 {
            US8::US8_Markup(v12) => {
                let v12: Rc<str> = v12.clone();
                match &v11 {
                    US8::US8_Markup(v13) => {
                        let v13: Rc<str> = v13.clone();
                        let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                        US8::US8_Markup(v14)
                    }
                }
            }
        };
        let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<span>raw</span>"); } LIT.with(|lit| lit.clone()) };
        let v19: US8 = trusted_40(v18);
        let v20: US8 = anchor_44(v2.clone(), v19);
        match &v17 {
            US8::US8_Markup(v21) => {
                let v21: Rc<str> = v21.clone();
                match &v20 {
                    US8::US8_Markup(v22) => {
                        let v22: Rc<str> = v22.clone();
                        let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                        US8::US8_Markup(v23)
                    }
                }
            }
        }
    } else {
        let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128196;"); } LIT.with(|lit| lit.clone()) };
        entry_link_61(v27, v2, v1, v3)
    }
}
fn node_bytes_69(mut v0: Rc<UH1>) -> i64 {
    match &*v0 {
        UH1::UH1_File(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let mut v3: i64 = *v3;
            v3
        }
        UH1::UH1_Folder(v4, v5, v6) => {
            let mut v4: Rc<str> = v4.clone();
            let v5: Rc<str> = v5.clone();
            let mut v6: Rc<UH2> = v6.clone();
            nodes_bytes_68(v6)
        }
    }
}
fn nodes_bytes_68(mut v0: Rc<UH2>) -> i64 {
    match &*v0 {
        UH2::UH2_Cons(v1, v2) => {
            let mut v1: Rc<UH1> = v1.clone();
            let mut v2: Rc<UH2> = v2.clone();
            let mut v3: i64 = node_bytes_69(v1);
            let mut v4: i64 = nodes_bytes_68(v2);
            let v5: i64 = v3.wrapping_add(v4);
            v5
        }
        UH2::UH2_Nil => {
            0i64
        }
    }
}
fn node_html_59(mut v0: bool, mut v1: Rc<UH1>) -> US8 {
    match &*v1 {
        UH1::UH1_File(v2, v3, v4) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = v3.clone();
            let mut v4: i64 = *v4;
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("div"); } LIT.with(|lit| lit.clone()) };
            let mut v6: US8 = file_html_60(v0, v2, v3, v4);
            wrap_51(v5, v6)
        }
        UH1::UH1_Folder(v8, v9, v10) => {
            let v8: Rc<str> = v8.clone();
            let v9: Rc<str> = v9.clone();
            let v10: Rc<UH2> = v10.clone();
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("summary"); } LIT.with(|lit| lit.clone()) };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("&#128194;"); } LIT.with(|lit| lit.clone()) };
            let v13: i64 = nodes_bytes_68(v10.clone());
            let mut v14: US8 = entry_link_61(v12, v9, v8, v13);
            let v15: US8 = wrap_51(v11, v14);
            let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\">"); } LIT.with(|lit| lit.clone()) };
            let v17: US8 = trusted_40(v16);
            let v23: US8 = match &v17 {
                US8::US8_Markup(v18) => {
                    let v18: Rc<str> = v18.clone();
                    match &v15 {
                        US8::US8_Markup(v19) => {
                            let v19: Rc<str> = v19.clone();
                            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
                            US8::US8_Markup(v20)
                        }
                    }
                }
            };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("div"); } LIT.with(|lit| lit.clone()) };
            let v25: US8 = nodes_html_58(v0, v10);
            let v26: US8 = wrap_51(v24, v25);
            let v32: US8 = match &v23 {
                US8::US8_Markup(v27) => {
                    let v27: Rc<str> = v27.clone();
                    match &v26 {
                        US8::US8_Markup(v28) => {
                            let v28: Rc<str> = v28.clone();
                            let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                            US8::US8_Markup(v29)
                        }
                    }
                }
            };
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</details>"); } LIT.with(|lit| lit.clone()) };
            let v34: US8 = trusted_40(v33);
            match &v32 {
                US8::US8_Markup(v35) => {
                    let v35: Rc<str> = v35.clone();
                    match &v34 {
                        US8::US8_Markup(v36) => {
                            let v36: Rc<str> = v36.clone();
                            let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v35, v36));
                            US8::US8_Markup(v37)
                        }
                    }
                }
            }
        }
    }
}
fn nodes_html_58(mut v0: bool, mut v1: Rc<UH2>) -> US8 {
    match &*v1 {
        UH2::UH2_Cons(v2, v3) => {
            let mut v2: Rc<UH1> = v2.clone();
            let mut v3: Rc<UH2> = v3.clone();
            let mut v4: US8 = node_html_59(v0, v2);
            let v5: US8 = nodes_html_58(v0, v3);
            match &v4 {
                US8::US8_Markup(v6) => {
                    let mut v6: Rc<str> = v6.clone();
                    match &v5 {
                        US8::US8_Markup(v7) => {
                            let mut v7: Rc<str> = v7.clone();
                            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                            US8::US8_Markup(v8)
                        }
                    }
                }
            }
        }
        UH2::UH2_Nil => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_40(v12)
        }
    }
}
fn readme_section_70(mut v0: US9) -> US8 {
    match &v0 {
        US9::US9_None => {
            let v18: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_40(v18)
        }
        US9::US9_Some(v1) => {
            let mut v1: US8 = v1.clone();
            let mut v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<section class=\"readme\">\n"); } LIT.with(|lit| lit.clone()) };
            let mut v3: US8 = trusted_40(v2);
            let v9: US8 = match &v3 {
                US8::US8_Markup(v4) => {
                    let mut v4: Rc<str> = v4.clone();
                    match &v1 {
                        US8::US8_Markup(v5) => {
                            let v5: Rc<str> = v5.clone();
                            let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
                            US8::US8_Markup(v6)
                        }
                    }
                }
            };
            let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</section>\n"); } LIT.with(|lit| lit.clone()) };
            let v11: US8 = trusted_40(v10);
            match &v9 {
                US8::US8_Markup(v12) => {
                    let v12: Rc<str> = v12.clone();
                    match &v11 {
                        US8::US8_Markup(v13) => {
                            let v13: Rc<str> = v13.clone();
                            let mut v14: Rc<str> = Rc::<str>::from(format!("{}{}", v12, v13));
                            US8::US8_Markup(v14)
                        }
                    }
                }
            }
        }
    }
}
fn index_page_of_57(mut v0: bool, mut v1: Rc<UH2>, mut v2: US9) -> Rc<UH3> {
    let mut v3: US8 = head_html_46();
    let mut v4: US8 = nodes_html_58(v0, v1);
    let v10: US8 = match &v3 {
        US8::US8_Markup(v5) => {
            let v5: Rc<str> = v5.clone();
            match &v4 {
                US8::US8_Markup(v6) => {
                    let mut v6: Rc<str> = v6.clone();
                    let mut v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
                    US8::US8_Markup(v7)
                }
            }
        }
    };
    let v11: US8 = readme_section_70(v2);
    let v12: US8 = close_html_53(v11);
    let v18: US8 = match &v10 {
        US8::US8_Markup(v13) => {
            let v13: Rc<str> = v13.clone();
            match &v12 {
                US8::US8_Markup(v14) => {
                    let mut v14: Rc<str> = v14.clone();
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                    US8::US8_Markup(v15)
                }
            }
        }
    };
    Rc::new(UH3::UH3_IndexPage(v18))
}
fn render_tree_8(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: bool) -> US5 {
    let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let mut v4: Rc<UH2> = walk_9(v0.clone(), v3);
    let mut v7: i32 = if v2 {
        let v5: i32 = 0i32;
        write_file_pages_30(v0.clone(), v4.clone(), v5)
    } else {
        0i32
    };
    let v8: US9 = readme_of_56(v0);
    let v9: Rc<UH3> = index_page_of_57(v2, v4, v8);
    let v12: Rc<str> = match &*v9 {
        UH3::UH3_IndexPage(v10) => {
            let v10: US8 = v10.clone();
            markup_text_54(v10)
        }
    };
    put_55(v1, v12);
    US5::US5_Wrote(v7)
}
fn get_temp_path_73() -> Rc<str> {
    let mut v1: std::path::PathBuf = std::env::temp_dir();
    let mut v2: std::path::PathBuf = method21(v1);
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
fn go_74(mut v0: i32, mut v1: u64, mut v2: Rc<str>) -> Rc<str> {
    loop {
        let mut v3: bool = v0 <= 0i32;
        if v3 {
            return v2.clone();
        } else {
            let mut v4: u64 = v1 & 15u64;
            let v5: i32 = (v4 as i32);
            let mut v6: i32 = v0.wrapping_sub(1i32);
            let mut v7: u64 = v1.wrapping_shr((4i32) as u32);
            let v8: Rc<str> = string_slice(&{ thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789abcdef"); } LIT.with(|lit| lit.clone()) }, v5 as i64, v5 as i64);
            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v2));
            (v0, v1, v2) = (v6, v7, v9.clone());
            continue;
        }
    }
}
fn fixture_dir_72() -> Rc<str> {
    let v0: Rc<str> = get_temp_path_73();
    let mut v1: std::collections::hash_map::DefaultHasher = std::hash::BuildHasher::build_hasher(&std::collections::hash_map::RandomState::new());
    let mut v3: bool = true; // 1;
    let mut v1 = v1;
    let v5: u128 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos();
    std::hash::Hasher::write_u128(&mut v1, v5);
    let mut v6: u64 = std::hash::Hasher::finish(&v1);
    std::hash::Hasher::write_u64(&mut v1, v6);
    let mut v7: u64 = std::hash::Hasher::finish(&v1);
    let v8: u64 = v6 & 4294967295u64;
    let v9: i32 = 8i32;
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = go_74(v9, v8, v10.clone());
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
    let mut v14: u64 = v6.wrapping_shr((32i32) as u32);
    let v15: u64 = v14 & 65535u64;
    let v16: i32 = 4i32;
    let v17: Rc<str> = go_74(v16, v15, v10.clone());
    let v18: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v17));
    let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("-4"); } LIT.with(|lit| lit.clone()) };
    let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
    let v21: u64 = v6.wrapping_shr((48i32) as u32);
    let v22: u64 = v21 & 4095u64;
    let v23: i32 = 3i32;
    let v24: Rc<str> = go_74(v23, v22, v10.clone());
    let v26: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v20, v24)), v12));
    let v27: u64 = v7 & 16383u64;
    let v28: u64 = v27 | 32768u64;
    let v29: i32 = 4i32;
    let v30: Rc<str> = go_74(v29, v28, v10.clone());
    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v26, v30)), v12));
    let v33: u64 = v7.wrapping_shr((16i32) as u32);
    let v34: u64 = v33 & 281474976710655u64;
    let v35: i32 = 12i32;
    let v36: Rc<str> = go_74(v35, v34, v10);
    let v37: Rc<str> = Rc::<str>::from(format!("{}{}", v32, v36));
    let v38: Rc<str> = Rc::<str>::from(v37.to_lowercase());
    let v39: Rc<str> = v38 ;
    let v40: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html-self-test-"); } LIT.with(|lit| lit.clone()) };
    let v41: Rc<str> = Rc::<str>::from(format!("{}{}", v40, v39));
    method20(v0, v41)
}
fn make_dir_76(mut v0: Rc<str>) -> () {
    std::fs::create_dir_all(&*v0).unwrap();
    ()
}
fn times_77(mut v0: Rc<str>, mut v1: i64) -> Rc<str> {
    let mut v2: bool = v1 <= 0i64;
    if v2 {
        let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
        v3
    } else {
        let mut v4: i64 = v1.wrapping_sub(1i64);
        let v5: Rc<str> = times_77(v0.clone(), v4);
        let mut v6: Rc<str> = Rc::<str>::from(format!("{}{}", v0, v5));
        v6
    }
}
fn lay_75(mut v0: Rc<str>, mut v1: i64) -> () {
    loop {
        let mut v2: bool = v1 >= 0i64;
        if v2 {
            make_dir_76(v0.clone());
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("file.txt"); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = method20(v0.clone(), v3.clone());
            let v5: Rc<str> = digit_65(v1);
            let mut v6: i64 = v1.wrapping_add(1i64);
            let mut v7: Rc<str> = times_77(v5.clone(), v6);
            put_55(v4.clone(), v7.clone());
            let v8: Rc<str> = digit_65(v1);
            let v9: Rc<str> = method20(v0.clone(), v8.clone());
            let v10: i64 = v1.wrapping_sub(1i64);
            (v0, v1) = (v9.clone(), v10);
            continue;
        }
        return ();
    }
}
fn self_test_71() -> US5 {
    let v0: Rc<str> = fixture_dir_72();
    let mut v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("_.root"); } LIT.with(|lit| lit.clone()) };
    let mut v2: Rc<str> = method20(v0.clone(), v1);
    let mut v3: i64 = 3i64;
    lay_75(v2, v3);
    let mut v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<UH2> = walk_9(v0, v4);
    let mut v6: bool = false;
    let mut v7: US8 = nodes_html_58(v6, v5);
    let v8: Rc<str> = markup_text_54(v7);
    let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div>"); } LIT.with(|lit| lit.clone()) };
    let v10: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div>"); } LIT.with(|lit| lit.clone()) };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div>"); } LIT.with(|lit| lit.clone()) };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div>"); } LIT.with(|lit| lit.clone()) };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root\">_.root</a><span> (10.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3\">3</a><span> (6.00 B)</span></summary><div><details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2\">2</a><span> (3.00 B)</span></summary><div>"); } LIT.with(|lit| lit.clone()) };
    let mut v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div>"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
    let v16: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<details open=\"true\"><summary>&#128194; <a href=\"_.root/3/2/1\">1</a><span> (1.00 B)</span></summary><div><div>&#128196; <a href=\"_.root/3/2/1/file.txt\">file.txt</a><span> (1.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>&#128196; <a href=\"_.root/3/2/file.txt\">file.txt</a><span> (2.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v13, Rc::<str>::from(format!("{}{}", v16, v17))));
    let v20: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>&#128196; <a href=\"_.root/3/file.txt\">file.txt</a><span> (3.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
    let v21: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<div>&#128196; <a href=\"_.root/file.txt\">file.txt</a><span> (4.00 B)</span></div></div></details>"); } LIT.with(|lit| lit.clone()) };
    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v19, Rc::<str>::from(format!("{}{}", v20, v21))));
    let v24: bool = &*v8 == &*v23;
    if v24 {
        US5::US5_Wrote(0i32)
    } else {
        US5::US5_Mismatch(v8)
    }
}
fn run_7(mut v0: US1) -> US5 {
    match &v0 {
        US1::US1_BadArgs(v6) => {
            let mut v6: Rc<str> = v6.clone();
            US5::US5_Refused(v6)
        }
        US1::US1_Render(v1, v2, v3) => {
            let mut v1: Rc<str> = v1.clone();
            let mut v2: Rc<str> = v2.clone();
            let mut v3: bool = *v3;
            render_tree_8(v1, v2, v3)
        }
        US1::US1_SelfTest => {
            self_test_71()
        }
    }
}
fn exit_of_78(mut v0: US5) -> i32 {
    match &v0 {
        US5::US5_Mismatch(v2) => {
            let mut v2: Rc<str> = v2.clone();
            let mut v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("html self-test mismatch: "); } LIT.with(|lit| lit.clone()) };
            let mut v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v2));
            eprintln!("{}", v4);
            1i32
        }
        US5::US5_Refused(v5) => {
            let v5: Rc<str> = v5.clone();
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
    let v0: i32 = 0i32;
    let mut v1: Rc<UH0> = arguments_from_0(v0);
    let mut v2: US1 = mode_of_3(v1);
    let mut v3: US5 = run_7(v2);
    exit_of_78(v3)
}
fn main() { #[cfg(target_arch = "wasm32")] { spiral_main(); return; }
    let main = std::thread::Builder::new().stack_size(1 << 30).spawn(spiral_main).unwrap();
    std::process::exit(match main.join() { Ok(code) => code, Err(_) => 1 });
}
