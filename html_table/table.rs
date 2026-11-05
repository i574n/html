#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{list_string, align, list_align, option_list_align};
fn string_slice(value: &str, from: i64, to: i64) -> Rc<str> {
    let bytes = value.as_bytes();
    let length = bytes.len() as i64;
    if from < 0 || from > length || to < from - 1 || to >= length { std::process::abort(); }
    if to < from { return { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) }; }
    if (bytes[from as usize] & 0xC0) == 0x80 || (to + 1 < length && (bytes[(to + 1) as usize] & 0xC0) == 0x80) { std::process::exit(3); }
    let slice = &bytes[from as usize..(to + 1) as usize];
    match std::str::from_utf8(slice) { Ok(text) => Rc::<str>::from(text), Err(error) => Rc::<str>::from(std::str::from_utf8(&slice[..error.valid_up_to()]).unwrap_or("")) }
}
fn method2(mut v0: Rc<list_string>, mut v1: Rc<list_string>) -> Rc<list_string> {
    loop {
        match &*v0 {
            list_string::list_string_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<list_string> = v3.clone();
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
fn cell_end_3(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'\\';
            let v8: bool = if v5 {
                let v6: i32 = v1.wrapping_add(1i32);
                let v7: bool = v6 < v2;
                v7
            } else {
                false
            };
            if v8 {
                let v9: i32 = v1.wrapping_add(2i32);
                v1 = v9;
                continue;
            } else {
                let v11: bool = v4 == b'|';
                if v11 {
                    return v1;
                } else {
                    let v12: i32 = v1.wrapping_add(1i32);
                    v1 = v12;
                    continue;
                }
            }
        }
    }
}
fn run_of_4(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
fn cells_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<list_string>) -> Rc<list_string> {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            let v5: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
            return method2(v2, v5);
        } else {
            let v7: i32 = cell_end_3(v0.clone(), v1);
            let v8: Rc<str> = run_of_4(v0.clone(), v1, v7);
            let v9: Rc<str> = html_model::html_model_trim(&v8);
            let v10: bool = v7 >= v3;
            let v12: bool = if v10 {
                let v11: bool = &*v9 == "";
                v11
            } else {
                false
            };
            if v12 {
                let v13: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
                return method2(v2, v13);
            } else {
                let v15: i32 = v7.wrapping_add(1i32);
                let v16: Rc<list_string> = Rc::new(list_string::list_string_Cons(v9, v2));
                (v1, v2) = (v15, v16);
                continue;
            }
        }
    }
}
fn row_cells_0(mut v0: Rc<str>) -> Rc<list_string> {
    let mut v1: Rc<str> = html_model::html_model_trim(&v0);
    let v2: i32 = 1i32;
    let v3: i32 = (v1.clone().len() as i32);
    let v4: Rc<str> = run_of_4(v1, v2, v3);
    let v5: i32 = 0i32;
    let v6: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
    cells_from_1(v4, v5, v6)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<list_string>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<list_string>> = Rc::new(move |mut v0: Rc<str>| -> Rc<list_string> {
        row_cells_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn table_char_7(mut v0: u8) -> bool {
    let mut v1: bool = v0 == b'|';
    let v3: bool = if v1 {
        true
    } else {
        let v2: bool = v0 == b':';
        v2
    };
    if v3 {
        true
    } else {
        let v4: bool = v0 == b' ';
        v4
    }
}
fn delimiter_chars_6(mut v0: Rc<str>, mut v1: i32, mut v2: bool) -> bool {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            return v2;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'-';
            if v6 {
                let v7: i32 = v1.wrapping_add(1i32);
                let v8: bool = true;
                (v1, v2) = (v7, v8);
                continue;
            } else {
                let v10: bool = table_char_7(v5);
                if v10 {
                    let v11: i32 = v1.wrapping_add(1i32);
                    v1 = v11;
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn align_of_9(mut v0: Rc<str>) -> align {
    let mut v1: i32 = (v0.clone().len() as i32);
    let v2: bool = v1 > 0i32;
    let v5: bool = if v2 {
        let v3: u8 = v0.clone().as_bytes()[0i32 as usize];
        let v4: bool = v3 == b':';
        v4
    } else {
        false
    };
    let v9: bool = if v2 {
        let v6: i32 = v1.wrapping_sub(1i32);
        let v7: u8 = v0.as_bytes()[v6 as usize];
        let v8: bool = v7 == b':';
        v8
    } else {
        false
    };
    let v10: bool = v5 && v9;
    if v10 {
        align::align_AlignCenter
    } else {
        if v9 {
            align::align_AlignRight
        } else {
            if v5 {
                align::align_AlignLeft
            } else {
                align::align_AlignNone
            }
        }
    }
}
fn method8(mut v0: Rc<list_string>, mut v1: Rc<list_align>) -> Rc<list_align> {
    match &*v0 {
        list_string::list_string_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<list_string> = v3.clone();
            let v4: Rc<list_align> = method8(v3, v1);
            let v5: align = align_of_9(v2);
            Rc::new(list_align::list_align_Cons(v5, v4))
        }
        list_string::list_string_Nil => {
            v1
        }
    }
}
fn aligns_of_5(mut v0: Rc<str>) -> option_list_align {
    let mut v1: i32 = 0i32;
    let v2: bool = false;
    let v3: bool = delimiter_chars_6(v0.clone(), v1, v2);
    if v3 {
        let v4: Rc<list_string> = row_cells_0(v0);
        let v5: Rc<list_align> = { thread_local!{ static CASE: Rc<list_align> = Rc::new(list_align::list_align_Nil); } CASE.with(|case| case.clone()) };
        let v6: Rc<list_align> = method8(v4, v5);
        option_list_align::option_list_align_Some(v6)
    } else {
        option_list_align::option_list_align_None
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> option_list_align> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> option_list_align> = Rc::new(move |mut v0: Rc<str>| -> option_list_align {
        aligns_of_5(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_table_row_cells(v0: &str) -> Rc<list_string> {
    closure0()(Rc::<str>::from(v0))
}
pub fn html_table_aligns_of(v0: &str) -> option_list_align {
    closure1()(Rc::<str>::from(v0))
}
