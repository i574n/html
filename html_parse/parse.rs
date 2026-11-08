#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{list_string, inline, list_inline, list_list_inline, align, list_align, list_list_list_inline, level, block, list_block, option_level, option_list_align};
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
enum US3 {
    US3_Idle,
    US3_InParagraph(Rc<str>),
    US3_InBullets(Rc<list_list_inline>, Rc<str>),
    US3_InNumbered(i64, Rc<list_list_inline>, Rc<str>),
    US3_InCode(Rc<str>, Rc<list_string>),
    US3_InTable(Rc<list_align>, Rc<list_list_inline>, Rc<list_list_list_inline>),
}
#[derive(Clone)]
enum US4 {
    US4_Blank,
    US4_Fence(Rc<str>),
    US4_HeadingLine(level, Rc<str>),
    US4_RuleLine,
    US4_RowLine(Rc<str>),
    US4_BulletLine(Rc<str>),
    US4_NumberLine(i64, Rc<str>),
    US4_PlainLine(Rc<str>),
}
#[derive(Clone)]
enum US5 {
    US5_Some(Rc<str>),
    US5_None,
}
#[derive(Clone)]
enum US6 {
    US6_Some(i64, Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Some(level, Rc<str>),
    US7_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<list_string>, US3, Rc<list_block>),
    US10_None,
}
fn method2(mut v0: Rc<list_block>, mut v1: Rc<list_block>) -> Rc<list_block> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_block::list_block_Cons(v2, v3) => {
                let v2: block = v2;
                let v3: Rc<list_block> = v3;
                let v4: Rc<list_block> = Rc::new(list_block::list_block_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            list_block::list_block_Nil => {
                return v1;
            }
        }
    }
}
fn run_of_6(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
fn starts_with_5(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: i32 = (v1.clone().len() as i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v2 <= v3;
    if v4 {
        let v5: i32 = 0i32;
        let v6: Rc<str> = run_of_6(v0, v5, v2);
        let v7: bool = &*v6 == &*v1;
        v7
    } else {
        false
    }
}
fn method10(mut v0: Rc<list_string>, mut v1: Rc<list_string>) -> Rc<list_string> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_string::list_string_Cons(v2, v3) => {
                let v2: Rc<str> = v2;
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
fn pre_of_9(mut v0: Rc<list_string>, mut v1: Rc<str>) -> block {
    let v2: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
    let v3: Rc<list_string> = method10(v0, v2);
    block::block_CodeBlock(v1, v3)
}
fn method12(mut v0: Rc<list_list_list_inline>, mut v1: Rc<list_list_list_inline>) -> Rc<list_list_list_inline> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_list_list_inline::list_list_list_inline_Cons(v2, v3) => {
                let v2: Rc<list_list_inline> = v2;
                let v3: Rc<list_list_list_inline> = v3;
                let v4: Rc<list_list_list_inline> = Rc::new(list_list_list_inline::list_list_list_inline_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            list_list_list_inline::list_list_list_inline_Nil => {
                return v1;
            }
        }
    }
}
fn rows_of_11(mut v0: Rc<list_list_list_inline>, mut v1: Rc<list_list_inline>, mut v2: Rc<list_align>) -> block {
    let v3: Rc<list_list_list_inline> = { thread_local!{ static CASE: Rc<list_list_list_inline> = Rc::new(list_list_list_inline::list_list_list_inline_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<list_list_list_inline> = method12(v0, v3);
    block::block_Table(v2, v1, v4)
}
fn close_block_8(mut v0: US3, mut v1: Rc<list_block>) -> Rc<list_block> {
    match v0 {
        US3::US3_InCode(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<list_string> = v3;
            let v4: block = pre_of_9(v3, v2);
            Rc::new(list_block::list_block_Cons(v4, v1))
        }
        US3::US3_InTable(v6, v7, v8) => {
            let v6: Rc<list_align> = v6;
            let v7: Rc<list_list_inline> = v7;
            let v8: Rc<list_list_list_inline> = v8;
            let v9: block = rows_of_11(v8, v7, v6);
            Rc::new(list_block::list_block_Cons(v9, v1))
        }
        _ => {
            v1
        }
    }
}
fn method15(mut v0: Rc<list_list_inline>, mut v1: Rc<list_list_inline>) -> Rc<list_list_inline> {
    loop {
        match Rc::unwrap_or_clone(v0) {
            list_list_inline::list_list_inline_Cons(v2, v3) => {
                let v2: Rc<list_inline> = v2;
                let v3: Rc<list_list_inline> = v3;
                let v4: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            list_list_inline::list_list_inline_Nil => {
                return v1;
            }
        }
    }
}
fn finished_14(mut v0: Rc<list_list_inline>, mut v1: Rc<str>) -> Rc<list_list_inline> {
    let v2: Rc<list_inline> = html_inline::html_inline_parse(&v1);
    let v3: Rc<list_list_inline> = { thread_local!{ static CASE: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Cons(v2, v3));
    method15(v0, v4)
}
fn bullets_of_13(mut v0: Rc<str>, mut v1: Rc<list_list_inline>) -> block {
    let v2: Rc<list_list_inline> = finished_14(v1, v0);
    block::block_Bullets(v2)
}
fn numbers_of_16(mut v0: Rc<str>, mut v1: Rc<list_list_inline>, mut v2: i64) -> block {
    let v3: Rc<list_list_inline> = finished_14(v1, v0);
    block::block_Numbered(v2, v3)
}
fn close_7(mut v0: US3, mut v1: Rc<list_block>) -> Rc<list_block> {
    match &v0 {
        US3::US3_InBullets(v6, v7) => {
            let v6: Rc<list_list_inline> = v6.clone();
            let v7: Rc<str> = v7.clone();
            let v8: block = bullets_of_13(v7, v6);
            Rc::new(list_block::list_block_Cons(v8, v1))
        }
        US3::US3_InNumbered(v10, v11, v12) => {
            let v10: i64 = *v10;
            let v11: Rc<list_list_inline> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: block = numbers_of_16(v12, v11, v10);
            Rc::new(list_block::list_block_Cons(v13, v1))
        }
        US3::US3_InParagraph(v2) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<list_inline> = html_inline::html_inline_parse(&v2);
            let v4: block = block::block_Paragraph(v3);
            Rc::new(list_block::list_block_Cons(v4, v1))
        }
        _ => {
            close_block_8(v0, v1)
        }
    }
}
fn code_step_4(mut v0: Rc<str>, mut v1: Rc<list_string>, mut v2: Rc<list_string>, mut v3: Rc<str>, mut v4: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
    let v6: bool = starts_with_5(v0.clone(), v5);
    if v6 {
        let v7: US3 = US3::US3_InCode(v3, v2);
        let v8: Rc<list_block> = close_7(v7, v4);
        let v9: US3 = US3::US3_Idle;
        (v1, v9, v8)
    } else {
        let v10: Rc<list_string> = Rc::new(list_string::list_string_Cons(v0, v2));
        let v11: US3 = US3::US3_InCode(v3, v10);
        (v1, v11, v4)
    }
}
fn at_is_22(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> bool {
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v1 < v3;
    if v4 {
        let v5: u8 = v0.as_bytes()[v1 as usize];
        let v6: bool = v5 == v2;
        v6
    } else {
        false
    }
}
fn bullet_of_21(mut v0: Rc<str>, mut v1: i32) -> US5 {
    let v2: i32 = (v0.clone().len() as i32);
    let v3: bool = v1 < v2;
    let v5: u8 = if v3 {
        let v4: u8 = v0.clone().as_bytes()[v1 as usize];
        v4
    } else {
        b' '
    };
    let v6: bool = v5 == b'-';
    let v8: bool = if v6 {
        true
    } else {
        let v7: bool = v5 == b'*';
        v7
    };
    let v12: bool = if v8 {
        let v9: i32 = v1.wrapping_add(1i32);
        let v10: u8 = b' ';
        at_is_22(v0.clone(), v9, v10)
    } else {
        false
    };
    if v12 {
        let v13: i32 = v1.wrapping_add(2i32);
        let v14: Rc<str> = run_of_6(v0, v13, v2);
        US5::US5_Some(v14)
    } else {
        US5::US5_None
    }
}
fn digit_value_25(mut v0: u8) -> i64 {
    let mut v1: u32 = u32::from(v0);
    let v2: i64 = (v1 as i64);
    let v3: bool = v2 >= 48i64;
    let v5: bool = if v3 {
        let v4: bool = v2 <= 57i64;
        v4
    } else {
        false
    };
    if v5 {
        let v6: i64 = v2.wrapping_sub(48i64);
        v6
    } else {
        -1i64
    }
}
fn digits_from_24(mut v0: Rc<str>, mut v1: i32, mut v2: i64, mut v3: i32) -> (i64, i32) {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 < v4;
        let v8: i64 = if v5 {
            let v6: u8 = v0.clone().as_bytes()[v1 as usize];
            digit_value_25(v6)
        } else {
            -1i64
        };
        let v9: bool = v8 >= 0i64;
        let v11: bool = if v9 {
            let v10: bool = v3 < 9i32;
            v10
        } else {
            false
        };
        if v11 {
            let v12: i32 = v1.wrapping_add(1i32);
            let v13: i64 = v2.wrapping_mul(10i64);
            let v14: i64 = v13.wrapping_add(v8);
            let v15: i32 = v3.wrapping_add(1i32);
            (v1, v2, v3) = (v12, v14, v15);
            continue;
        } else {
            return (v2, v1);
        }
    }
}
fn number_of_23(mut v0: Rc<str>, mut v1: i32) -> US6 {
    let v2: i64 = 0i64;
    let v3: i32 = 0i32;
    let (mut v4, mut v5): (i64, i32) = digits_from_24(v0.clone(), v1, v2, v3);
    let v6: bool = v5 > v1;
    let v9: bool = if v6 {
        let v7: u8 = b'.';
        at_is_22(v0.clone(), v5, v7)
    } else {
        false
    };
    let v13: bool = if v9 {
        let v10: i32 = v5.wrapping_add(1i32);
        let v11: u8 = b' ';
        at_is_22(v0.clone(), v10, v11)
    } else {
        false
    };
    if v13 {
        let v14: i32 = v5.wrapping_add(2i32);
        let v15: i32 = (v0.clone().len() as i32);
        let v16: Rc<str> = run_of_6(v0, v14, v15);
        US6::US6_Some(v4, v16)
    } else {
        US6::US6_None
    }
}
fn listed_20(mut v0: Rc<str>, mut v1: i32) -> US4 {
    let v2: US5 = bullet_of_21(v0.clone(), v1);
    match v2 {
        US5::US5_None => {
            let v5: US6 = number_of_23(v0.clone(), v1);
            match v5 {
                US6::US6_None => {
                    US4::US4_PlainLine(v0)
                }
                US6::US6_Some(v6, v7) => {
                    let v6: i64 = v6;
                    let v7: Rc<str> = v7;
                    US4::US4_NumberLine(v6, v7)
                }
            }
        }
        US5::US5_Some(v3) => {
            let v3: Rc<str> = v3;
            US4::US4_BulletLine(v3)
        }
    }
}
fn hashes_from_27(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 < v2;
        let v6: bool = if v3 {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = v4 == b'#';
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
fn heading_of_26(mut v0: Rc<str>) -> US7 {
    let mut v1: i32 = 0i32;
    let v2: i32 = hashes_from_27(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v2 < v3;
    let v7: bool = if v4 {
        let v5: u8 = v0.clone().as_bytes()[v2 as usize];
        let v6: bool = v5 == b' ';
        v6
    } else {
        false
    };
    if v7 {
        let v8: option_level = html_model::html_model_level_of(v2);
        match v8 {
            option_level::option_level_None => {
                US7::US7_None
            }
            option_level::option_level_Some(v9) => {
                let v9: level = v9;
                let v10: i32 = v2.wrapping_add(1i32);
                let v11: Rc<str> = run_of_6(v0, v10, v3);
                US7::US7_Some(v9, v11)
            }
        }
    } else {
        US7::US7_None
    }
}
fn shaped_19(mut v0: Rc<str>, mut v1: i32) -> US4 {
    let v2: u8 = v0.clone().as_bytes()[v1 as usize];
    let v3: bool = v2 == b'|';
    if v3 {
        US4::US4_RowLine(v0)
    } else {
        let v5: US7 = heading_of_26(v0.clone());
        match v5 {
            US7::US7_None => {
                listed_20(v0, v1)
            }
            US7::US7_Some(v6, v7) => {
                let v6: level = v6;
                let v7: Rc<str> = v7;
                US4::US4_HeadingLine(v6, v7)
            }
        }
    }
}
fn spaces_from_28(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
fn classify_18(mut v0: Rc<str>) -> US4 {
    let mut v1: i32 = 0i32;
    let v2: i32 = spaces_from_28(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v2 == v3;
    if v4 {
        US4::US4_Blank
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = starts_with_5(v0.clone(), v6);
        if v7 {
            let v8: i32 = 3i32;
            let v9: Rc<str> = run_of_6(v0, v8, v3);
            let v10: Rc<str> = html_model::html_model_trim(&v9);
            US4::US4_Fence(v10)
        } else {
            let v12: bool = &*v0 == "---";
            if v12 {
                US4::US4_RuleLine
            } else {
                shaped_19(v0, v2)
            }
        }
    }
}
fn method30(mut v0: Rc<list_string>, mut v1: Rc<list_list_inline>) -> Rc<list_list_inline> {
    match Rc::unwrap_or_clone(v0) {
        list_string::list_string_Cons(v2, v3) => {
            let v2: Rc<str> = v2;
            let v3: Rc<list_string> = v3;
            let v4: Rc<list_list_inline> = method30(v3, v1);
            let v5: Rc<list_inline> = html_inline::html_inline_parse(&v2);
            Rc::new(list_list_inline::list_list_inline_Cons(v5, v4))
        }
        list_string::list_string_Nil => {
            v1
        }
    }
}
fn inline_cells_29(mut v0: Rc<list_string>) -> Rc<list_list_inline> {
    let mut v1: Rc<list_list_inline> = { thread_local!{ static CASE: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Nil); } CASE.with(|case| case.clone()) };
    method30(v0, v1)
}
fn table_step_17(mut v0: Rc<str>, mut v1: Rc<list_string>, mut v2: Rc<list_string>, mut v3: US3, mut v4: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    let v5: US4 = classify_18(v0);
    match &v3 {
        US3::US3_InTable(v6, v7, v8) => {
            let v6: Rc<list_align> = v6.clone();
            let v7: Rc<list_list_inline> = v7.clone();
            let v8: Rc<list_list_list_inline> = v8.clone();
            match v5 {
                US4::US4_RowLine(v9) => {
                    let v9: Rc<str> = v9;
                    let v10: Rc<list_string> = html_table::html_table_row_cells(&v9);
                    let v11: Rc<list_list_inline> = inline_cells_29(v10);
                    let v12: Rc<list_list_list_inline> = Rc::new(list_list_list_inline::list_list_list_inline_Cons(v11, v8));
                    let v13: US3 = US3::US3_InTable(v6, v7, v12);
                    (v2, v13, v4)
                }
                _ => {
                    let v14: Rc<list_block> = close_7(v3, v4);
                    let v15: US3 = US3::US3_Idle;
                    (v1, v15, v14)
                }
            }
        }
        _ => {
            let v19: Rc<list_block> = close_7(v3, v4);
            let v20: US3 = US3::US3_Idle;
            (v1, v20, v19)
        }
    }
}
fn resume_33(mut v0: Rc<list_string>, mut v1: US3, mut v2: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    (v0, v1, v2)
}
fn delimiter_aligns_34(mut v0: &Rc<list_string>) -> option_list_align {
    match &**v0 {
        list_string::list_string_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<list_string> = v2.clone();
            let v3: US4 = classify_18(v1);
            match v3 {
                US4::US4_RowLine(v4) => {
                    let v4: Rc<str> = v4;
                    let v5: option_list_align = html_table::html_table_aligns_of(&v4);
                    v5
                }
                _ => {
                    option_list_align::option_list_align_None
                }
            }
        }
        list_string::list_string_Nil => {
            option_list_align::option_list_align_None
        }
    }
}
fn continues_36(mut v0: US3) -> bool {
    match v0 {
        US3::US3_InBullets(v2, v3) => {
            let v2: Rc<list_list_inline> = v2;
            let v3: Rc<str> = v3;
            true
        }
        US3::US3_InNumbered(v4, v5, v6) => {
            let v4: i64 = v4;
            let v5: Rc<list_list_inline> = v5;
            let v6: Rc<str> = v6;
            true
        }
        US3::US3_InParagraph(v1) => {
            let mut v1: Rc<str> = v1;
            true
        }
        _ => {
            false
        }
    }
}
fn joined_trim_39(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v3: Rc<str> = html_model::html_model_trim(&v1);
    let v4: Rc<str> = string_concat(&[&*v0, " ", &*v3]);
    v4
}
fn grow_item_38(mut v0: Rc<str>, mut v1: Rc<list_list_inline>, mut v2: Rc<str>) -> (Rc<list_list_inline>, Rc<str>) {
    let v3: Rc<str> = joined_trim_39(v0, v2);
    (v1, v3)
}
fn grow_numbered_40(mut v0: Rc<str>, mut v1: Rc<list_list_inline>, mut v2: i64, mut v3: Rc<str>) -> (i64, Rc<list_list_inline>, Rc<str>) {
    let v4: Rc<str> = joined_trim_39(v0, v3);
    (v2, v1, v4)
}
fn grow_37(mut v0: US3, mut v1: Rc<str>) -> US3 {
    match v0 {
        US3::US3_InBullets(v6, v7) => {
            let v6: Rc<list_list_inline> = v6;
            let v7: Rc<str> = v7;
            let (mut v8, mut v9): (Rc<list_list_inline>, Rc<str>) = grow_item_38(v7, v6, v1);
            US3::US3_InBullets(v8, v9)
        }
        US3::US3_InNumbered(v11, v12, v13) => {
            let v11: i64 = v11;
            let v12: Rc<list_list_inline> = v12;
            let v13: Rc<str> = v13;
            let (mut v14, mut v15, mut v16): (i64, Rc<list_list_inline>, Rc<str>) = grow_numbered_40(v13, v12, v11, v1);
            US3::US3_InNumbered(v14, v15, v16)
        }
        US3::US3_InParagraph(v2) => {
            let v2: Rc<str> = v2;
            let v4: Rc<str> = string_concat(&[&*v2, " ", &*v1]);
            US3::US3_InParagraph(v4)
        }
        _ => {
            US3::US3_InParagraph(v1)
        }
    }
}
fn continued_35(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<list_block>) -> (US3, Rc<list_block>) {
    let v3: bool = continues_36(v0.clone());
    if v3 {
        let v4: US3 = grow_37(v0, v1);
        (v4, v2)
    } else {
        let v5: US3 = grow_37(v0.clone(), v1);
        let v6: Rc<list_block> = close_7(v0, v2);
        (v5, v6)
    }
}
fn after_first_41(mut v0: &Rc<list_string>) -> Rc<list_string> {
    match &**v0 {
        list_string::list_string_Cons(v1, v2) => {
            let mut v1: Rc<str> = v1.clone();
            let v2: Rc<list_string> = v2.clone();
            v2
        }
        list_string::list_string_Nil => {
            { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
fn row_step_32(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<list_string>, mut v3: US3, mut v4: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    let v5: option_list_align = delimiter_aligns_34(&v2);
    match v5 {
        option_list_align::option_list_align_None => {
            let (mut v13, mut v14): (US3, Rc<list_block>) = continued_35(v3, v0, v4);
            resume_33(v2, v13, v14)
        }
        option_list_align::option_list_align_Some(v6) => {
            let v6: Rc<list_align> = v6;
            let v7: Rc<list_string> = after_first_41(&v2);
            let v8: Rc<list_string> = html_table::html_table_row_cells(&v1);
            let v9: Rc<list_list_inline> = inline_cells_29(v8);
            let v10: Rc<list_block> = close_7(v3, v4);
            let v11: Rc<list_list_list_inline> = { thread_local!{ static CASE: Rc<list_list_list_inline> = Rc::new(list_list_list_inline::list_list_list_inline_Nil); } CASE.with(|case| case.clone()) };
            let v12: US3 = US3::US3_InTable(v6, v9, v11);
            (v7, v12, v10)
        }
    }
}
fn bulleted_44(mut v0: US3, mut v1: Rc<str>, mut v2: Rc<list_block>) -> (US3, Rc<list_block>) {
    match &v0 {
        US3::US3_InBullets(v3, v4) => {
            let v3: Rc<list_list_inline> = v3.clone();
            let v4: Rc<str> = v4.clone();
            let v5: Rc<list_inline> = html_inline::html_inline_parse(&v4);
            let v6: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Cons(v5, v3));
            let v7: US3 = US3::US3_InBullets(v6, v1);
            (v7, v2)
        }
        _ => {
            let v8: Rc<list_block> = close_7(v0, v2);
            let v9: Rc<list_list_inline> = { thread_local!{ static CASE: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Nil); } CASE.with(|case| case.clone()) };
            let v10: US3 = US3::US3_InBullets(v9, v1);
            (v10, v8)
        }
    }
}
fn numbered_45(mut v0: US3, mut v1: i64, mut v2: Rc<str>, mut v3: Rc<list_block>) -> (US3, Rc<list_block>) {
    match &v0 {
        US3::US3_InNumbered(v4, v5, v6) => {
            let v4: i64 = *v4;
            let v5: Rc<list_list_inline> = v5.clone();
            let v6: Rc<str> = v6.clone();
            let v7: Rc<list_inline> = html_inline::html_inline_parse(&v6);
            let v8: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Cons(v7, v5));
            let v9: US3 = US3::US3_InNumbered(v4, v8, v2);
            (v9, v3)
        }
        _ => {
            let v10: Rc<list_block> = close_7(v0, v3);
            let v11: Rc<list_list_inline> = { thread_local!{ static CASE: Rc<list_list_inline> = Rc::new(list_list_inline::list_list_inline_Nil); } CASE.with(|case| case.clone()) };
            let v12: US3 = US3::US3_InNumbered(v1, v11, v2);
            (v12, v10)
        }
    }
}
fn item_step_43(mut v0: Rc<str>, mut v1: US4, mut v2: Rc<list_string>, mut v3: US3, mut v4: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    match v1 {
        US4::US4_BulletLine(v5) => {
            let v5: Rc<str> = v5;
            let (mut v6, mut v7): (US3, Rc<list_block>) = bulleted_44(v3, v5, v4);
            resume_33(v2, v6, v7)
        }
        US4::US4_NumberLine(v11, v12) => {
            let v11: i64 = v11;
            let v12: Rc<str> = v12;
            let (mut v13, mut v14): (US3, Rc<list_block>) = numbered_45(v3, v11, v12, v4);
            resume_33(v2, v13, v14)
        }
        _ => {
            let (mut v18, mut v19): (US3, Rc<list_block>) = continued_35(v3, v0, v4);
            resume_33(v2, v18, v19)
        }
    }
}
fn heading_block_47(mut v0: level, mut v1: Rc<str>) -> block {
    let v2: Rc<list_inline> = html_inline::html_inline_parse(&v1);
    block::block_Heading(v0, v2)
}
fn ended_46(mut v0: US4, mut v1: Rc<list_string>, mut v2: Rc<list_block>) -> US10 {
    match v0 {
        US4::US4_Blank => {
            let v7: US3 = US3::US3_Idle;
            US10::US10_Some(v1, v7, v2)
        }
        US4::US4_Fence(v3) => {
            let v3: Rc<str> = v3;
            let v4: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
            let v5: US3 = US3::US3_InCode(v3, v4);
            US10::US10_Some(v1, v5, v2)
        }
        US4::US4_HeadingLine(v9, v10) => {
            let v9: level = v9;
            let v10: Rc<str> = v10;
            let v11: block = heading_block_47(v9, v10);
            let v12: US3 = US3::US3_Idle;
            let v13: Rc<list_block> = Rc::new(list_block::list_block_Cons(v11, v2));
            US10::US10_Some(v1, v12, v13)
        }
        US4::US4_RuleLine => {
            let v15: US3 = US3::US3_Idle;
            let v16: block = block::block_Rule;
            let v17: Rc<list_block> = Rc::new(list_block::list_block_Cons(v16, v2));
            US10::US10_Some(v1, v15, v17)
        }
        _ => {
            US10::US10_None
        }
    }
}
fn plain_step_42(mut v0: Rc<str>, mut v1: US4, mut v2: Rc<list_string>, mut v3: US3, mut v4: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    let v5: Rc<list_block> = close_7(v3.clone(), v4.clone());
    let v6: US10 = ended_46(v1.clone(), v2.clone(), v5);
    match v6 {
        US10::US10_None => {
            item_step_43(v0, v1, v2, v3, v4)
        }
        US10::US10_Some(v7, v8, v9) => {
            let v7: Rc<list_string> = v7;
            let v8: US3 = v8;
            let v9: Rc<list_block> = v9;
            (v7, v8, v9)
        }
    }
}
fn line_step_31(mut v0: Rc<str>, mut v1: Rc<list_string>, mut v2: US3, mut v3: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    let v4: US4 = classify_18(v0.clone());
    match &v4 {
        US4::US4_RowLine(v5) => {
            let v5: Rc<str> = v5.clone();
            row_step_32(v0, v5, v1, v2, v3)
        }
        _ => {
            plain_step_42(v0, v4, v1, v2, v3)
        }
    }
}
fn step_3(mut v0: Rc<list_string>, mut v1: Rc<str>, mut v2: Rc<list_string>, mut v3: US3, mut v4: Rc<list_block>) -> (Rc<list_string>, US3, Rc<list_block>) {
    match &v3 {
        US3::US3_InCode(v5, v6) => {
            let v5: Rc<str> = v5.clone();
            let v6: Rc<list_string> = v6.clone();
            code_step_4(v1, v2, v6, v5, v4)
        }
        US3::US3_InTable(v10, v11, v12) => {
            let v10: Rc<list_align> = v10.clone();
            let v11: Rc<list_list_inline> = v11.clone();
            let v12: Rc<list_list_list_inline> = v12.clone();
            table_step_17(v1, v0, v2, v3, v4)
        }
        _ => {
            line_step_31(v1, v2, v3, v4)
        }
    }
}
fn blocks_from_1(mut v0: Rc<list_string>, mut v1: US3, mut v2: Rc<list_block>) -> Rc<list_block> {
    loop {
        match &*v0 {
            list_string::list_string_Cons(v6, v7) => {
                let v6: Rc<str> = v6.clone();
                let v7: Rc<list_string> = v7.clone();
                let (mut v8, mut v9, mut v10): (Rc<list_string>, US3, Rc<list_block>) = step_3(v0, v6, v7, v1, v2);
                (v0, v1, v2) = (v8, v9, v10);
                continue;
            }
            list_string::list_string_Nil => {
                let v3: Rc<list_block> = close_7(v1, v2);
                let v4: Rc<list_block> = { thread_local!{ static CASE: Rc<list_block> = Rc::new(list_block::list_block_Nil); } CASE.with(|case| case.clone()) };
                return method2(v3, v4);
            }
        }
    }
}
fn lines_from_48(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<list_string>) -> Rc<list_string> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = run_of_6(v0, v2, v1);
            let v7: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<list_string> = Rc::new(list_string::list_string_Cons(v6, v7));
            return method10(v3, v8);
        } else {
            let v10: u8 = v0.clone().as_bytes()[v1 as usize];
            let v11: bool = v10 == b'\n';
            if v11 {
                let v12: i32 = v1.wrapping_add(1i32);
                let v13: Rc<str> = run_of_6(v0.clone(), v2, v1);
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
fn parse_markdown_0(mut v0: Rc<str>) -> Rc<list_block> {
    let mut v1: i32 = 0i32;
    let v2: i32 = 0i32;
    let v3: Rc<list_string> = { thread_local!{ static CASE: Rc<list_string> = Rc::new(list_string::list_string_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<list_string> = lines_from_48(v0, v1, v2, v3);
    let v5: US3 = US3::US3_Idle;
    let v6: Rc<list_block> = { thread_local!{ static CASE: Rc<list_block> = Rc::new(list_block::list_block_Nil); } CASE.with(|case| case.clone()) };
    blocks_from_1(v4, v5, v6)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<list_block>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<list_block>> = Rc::new(move |mut v0: Rc<str>| -> Rc<list_block> {
        parse_markdown_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_parse_markdown(v0: &str) -> Rc<list_block> {
    closure0()(Rc::<str>::from(v0))
}
