#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{inline, list_inline};
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
    US0_Some(i32, Rc<inline>),
    US0_None,
}
fn rev_inlines_2(mut v0: Rc<list_inline>, mut v1: Rc<list_inline>) -> Rc<list_inline> {
    loop {
        match &*v0 {
            list_inline::list_inline_Cons(v2, v3) => {
                let mut v2: Rc<inline> = v2.clone();
                let v3: Rc<list_inline> = v3.clone();
                let v4: Rc<list_inline> = Rc::new(list_inline::list_inline_Cons(v2, v1));
                (v0, v1) = (v3, v4);
                continue;
            }
            list_inline::list_inline_Nil => {
                return v1;
            }
        }
    }
}
fn closure1() -> Rc<dyn Fn(Rc<str>) -> Rc<list_inline>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>> = Rc::new(move |mut v0: Rc<str>| -> Rc<list_inline> {
        let mut v1: i32 = 0i32;
        let mut v2: Rc<list_inline> = { thread_local!{ static CASE: Rc<list_inline> = Rc::new(list_inline::list_inline_Nil); } CASE.with(|case| case.clone()) };
        inlines_from_1(v0, v1, v2)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn marker_6(mut v0: u8) -> bool {
    let mut v1: bool = v0 == b'`';
    let v3: bool = if v1 {
        true
    } else {
        let mut v2: bool = v0 == b'[';
        v2
    };
    let v5: bool = if v3 {
        true
    } else {
        let v4: bool = v0 == b'!';
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = v0 == b'*';
        v6
    };
    let v9: bool = if v7 {
        true
    } else {
        let v8: bool = v0 == b'<';
        v8
    };
    if v9 {
        true
    } else {
        let v10: bool = v0 == b'\\';
        v10
    }
}
fn next_marker_5(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = marker_6(v4);
            if v5 {
                return v1;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                v1 = v6;
                continue;
            }
        }
    }
}
fn run_of_7(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
fn plain_piece_4(mut v0: Rc<str>, mut v1: i32) -> (i32, Rc<inline>) {
    let mut v2: i32 = v1.wrapping_add(1i32);
    let v3: i32 = next_marker_5(v0.clone(), v2);
    let v4: Rc<str> = run_of_7(v0, v1, v3);
    let v5: Rc<inline> = Rc::new(inline::inline_Text(v4));
    (v3, v5)
}
fn or_else_9(mut v0: Rc<dyn Fn() -> US0>, mut v1: US0) -> US0 {
    match &v1 {
        US0::US0_None => {
            v0()
        }
        US0::US0_Some(v2, v3) => {
            let mut v2: i32 = *v2;
            let v3: Rc<inline> = v3.clone();
            v1
        }
    }
}
fn find_char_12(mut v0: Rc<str>, mut v1: u8, mut v2: i32) -> i32 {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return -1i32;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v2 as usize];
            let v6: bool = v5 == v1;
            if v6 {
                return v2;
            } else {
                let v7: i32 = v2.wrapping_add(1i32);
                v2 = v7;
                continue;
            }
        }
    }
}
fn at_is_13(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> bool {
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
fn code_close_11(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: u8 = b'`';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    if v3 {
        let v4: u8 = b'`';
        let v5: i32 = v1.wrapping_add(1i32);
        find_char_12(v0, v4, v5)
    } else {
        -1i32
    }
}
fn code_try_10(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: i32 = code_close_11(v0.clone(), v1);
    let v3: bool = v2 >= 0i32;
    if v3 {
        let v4: i32 = v2.wrapping_add(1i32);
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: Rc<str> = run_of_7(v0, v5, v2);
        let v7: Rc<inline> = Rc::new(inline::inline_Code(v6));
        US0::US0_Some(v4, v7)
    } else {
        US0::US0_None
    }
}
fn escapable_16(mut v0: u8) -> bool {
    let mut v1: bool = v0 == b'*';
    let v3: bool = if v1 {
        true
    } else {
        let mut v2: bool = v0 == b'_';
        v2
    };
    let v5: bool = if v3 {
        true
    } else {
        let v4: bool = v0 == b'\\';
        v4
    };
    let v7: bool = if v5 {
        true
    } else {
        let v6: bool = v0 == b'`';
        v6
    };
    let v9: bool = if v7 {
        true
    } else {
        let v8: bool = v0 == b'[';
        v8
    };
    let v11: bool = if v9 {
        true
    } else {
        let v10: bool = v0 == b']';
        v10
    };
    let v13: bool = if v11 {
        true
    } else {
        let v12: bool = v0 == b'#';
        v12
    };
    let v15: bool = if v13 {
        true
    } else {
        let v14: bool = v0 == b'-';
        v14
    };
    let v17: bool = if v15 {
        true
    } else {
        let v16: bool = v0 == b'!';
        v16
    };
    let v19: bool = if v17 {
        true
    } else {
        let v18: bool = v0 == b'<';
        v18
    };
    if v19 {
        true
    } else {
        let v20: bool = v0 == b'|';
        v20
    }
}
fn escaped_at_15(mut v0: Rc<str>, mut v1: i32) -> bool {
    let mut v2: u8 = b'\\';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    let v7: bool = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: i32 = (v0.clone().len() as i32);
        let v6: bool = v4 < v5;
        v6
    } else {
        false
    };
    if v7 {
        let v8: i32 = v1.wrapping_add(1i32);
        let v9: u8 = v0.as_bytes()[v8 as usize];
        escapable_16(v9)
    } else {
        false
    }
}
fn escape_try_14(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: bool = escaped_at_15(v0.clone(), v1);
    if v2 {
        let v3: i32 = v1.wrapping_add(2i32);
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: Rc<str> = run_of_7(v0, v4, v3);
        let v6: Rc<inline> = Rc::new(inline::inline_Text(v5));
        US0::US0_Some(v3, v6)
    } else {
        US0::US0_None
    }
}
fn closure2(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn() -> US0> {
    Rc::new(move || -> US0 {
        escape_try_14(v0.clone(), v1)
    })
}
fn double_star_20(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let mut v2: i32 = v1.wrapping_add(1i32);
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v2 >= v3;
        if v4 {
            return -1i32;
        } else {
            let v5: u8 = v0.clone().as_bytes()[v1 as usize];
            let v6: bool = v5 == b'*';
            let v9: bool = if v6 {
                let v7: u8 = v0.clone().as_bytes()[v2 as usize];
                let v8: bool = v7 == b'*';
                v8
            } else {
                false
            };
            if v9 {
                return v1;
            } else {
                v1 = v2;
                continue;
            }
        }
    }
}
fn strong_close_19(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: u8 = b'*';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    let v7: bool = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: u8 = b'*';
        at_is_13(v0.clone(), v4, v5)
    } else {
        false
    };
    if v7 {
        let v8: i32 = v1.wrapping_add(2i32);
        double_star_20(v0, v8)
    } else {
        -1i32
    }
}
fn strong_try_18(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>>, mut v1: Rc<str>, mut v2: i32) -> US0 {
    let v3: i32 = strong_close_19(v1.clone(), v2);
    let v4: bool = v3 >= 0i32;
    if v4 {
        let v5: i32 = v3.wrapping_add(2i32);
        let v6: i32 = v2.wrapping_add(2i32);
        let v7: Rc<str> = run_of_7(v1, v6, v3);
        let v8: Rc<list_inline> = v0(v7);
        let v9: Rc<inline> = Rc::new(inline::inline_Strong(v8));
        US0::US0_Some(v5, v9)
    } else {
        US0::US0_None
    }
}
fn auto_close_22(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: u8 = b'<';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    let v7: i32 = if v3 {
        let v4: u8 = b'>';
        let v5: i32 = v1.wrapping_add(1i32);
        find_char_12(v0.clone(), v4, v5)
    } else {
        -1i32
    };
    let v8: i32 = v1.wrapping_add(5i32);
    let v9: bool = v7 > v8;
    let v13: bool = if v9 {
        let v10: i32 = v1.wrapping_add(1i32);
        let v11: Rc<str> = run_of_7(v0, v10, v8);
        let v12: bool = &*v11 == "http";
        v12
    } else {
        false
    };
    if v13 {
        v7
    } else {
        -1i32
    }
}
fn auto_try_21(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: i32 = auto_close_22(v0.clone(), v1);
    let v3: bool = v2 >= 0i32;
    if v3 {
        let v4: i32 = v2.wrapping_add(1i32);
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: Rc<str> = run_of_7(v0, v5, v2);
        let v7: Rc<inline> = Rc::new(inline::inline_AutoLink(v6));
        US0::US0_Some(v4, v7)
    } else {
        US0::US0_None
    }
}
fn closure4(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn() -> US0> {
    Rc::new(move || -> US0 {
        auto_try_21(v0.clone(), v1)
    })
}
fn strong_or_auto_17(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>>, mut v1: Rc<str>, mut v2: i32) -> US0 {
    let v3: US0 = strong_try_18(v0, v1.clone(), v2);
    let v4: Rc<dyn Fn() -> US0> = closure4(v1, v2);
    or_else_9(v4, v3)
}
fn closure3(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>>, mut v1: Rc<str>, mut v2: i32) -> Rc<dyn Fn() -> US0> {
    Rc::new(move || -> US0 {
        strong_or_auto_17(v0.clone(), v1.clone(), v2)
    })
}
fn marked_8(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>>, mut v1: Rc<str>, mut v2: i32) -> US0 {
    let v3: US0 = code_try_10(v1.clone(), v2);
    let v4: Rc<dyn Fn() -> US0> = closure2(v1.clone(), v2);
    let v5: US0 = or_else_9(v4, v3);
    let v6: Rc<dyn Fn() -> US0> = closure3(v0, v1, v2);
    or_else_9(v6, v5)
}
fn paren_close_26(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: bool = v1 < 0i32;
    if v2 {
        -1i32
    } else {
        let v3: i32 = v1.wrapping_add(1i32);
        let v4: u8 = b'(';
        let v5: bool = at_is_13(v0.clone(), v3, v4);
        if v5 {
            let v6: u8 = b')';
            let v7: i32 = v1.wrapping_add(2i32);
            find_char_12(v0, v6, v7)
        } else {
            -1i32
        }
    }
}
fn image_close_25(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: u8 = b'!';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    let v7: bool = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: u8 = b'[';
        at_is_13(v0.clone(), v4, v5)
    } else {
        false
    };
    if v7 {
        let v8: u8 = b']';
        let v9: i32 = v1.wrapping_add(2i32);
        let v10: i32 = find_char_12(v0.clone(), v8, v9);
        paren_close_26(v0, v10)
    } else {
        -1i32
    }
}
fn image_at_27(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<inline> {
    let v3: u8 = b']';
    let v4: i32 = v1.wrapping_add(2i32);
    let v5: i32 = find_char_12(v0.clone(), v3, v4);
    let v6: Rc<str> = run_of_7(v0.clone(), v4, v5);
    let v7: i32 = v5.wrapping_add(2i32);
    let v8: Rc<str> = run_of_7(v0, v7, v2);
    Rc::new(inline::inline_Image(v6, v8))
}
fn image_try_24(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: i32 = image_close_25(v0.clone(), v1);
    let v3: bool = v2 >= 0i32;
    if v3 {
        let v4: i32 = v2.wrapping_add(1i32);
        let v5: Rc<inline> = image_at_27(v0, v1, v2);
        US0::US0_Some(v4, v5)
    } else {
        US0::US0_None
    }
}
fn badge_close_29(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: u8 = b'[';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    let v6: i32 = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        image_close_25(v0.clone(), v4)
    } else {
        -1i32
    };
    let v7: bool = v6 >= 0i32;
    let v11: bool = if v7 {
        let v8: i32 = v6.wrapping_add(1i32);
        let v9: u8 = b']';
        at_is_13(v0.clone(), v8, v9)
    } else {
        false
    };
    if v11 {
        let v12: i32 = v6.wrapping_add(1i32);
        paren_close_26(v0, v12)
    } else {
        -1i32
    }
}
fn linked_image_at_30(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<inline> {
    let v3: i32 = v1.wrapping_add(1i32);
    let v4: i32 = image_close_25(v0.clone(), v3);
    let v5: u8 = b']';
    let v6: i32 = v1.wrapping_add(3i32);
    let v7: i32 = find_char_12(v0.clone(), v5, v6);
    let v8: Rc<str> = run_of_7(v0.clone(), v6, v7);
    let v9: i32 = v7.wrapping_add(2i32);
    let v10: Rc<str> = run_of_7(v0.clone(), v9, v4);
    let v11: i32 = v4.wrapping_add(3i32);
    let v12: Rc<str> = run_of_7(v0, v11, v2);
    Rc::new(inline::inline_LinkedImage(v8, v10, v12))
}
fn badge_try_28(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: i32 = badge_close_29(v0.clone(), v1);
    let v3: bool = v2 >= 0i32;
    if v3 {
        let v4: i32 = v2.wrapping_add(1i32);
        let v5: Rc<inline> = linked_image_at_30(v0, v1, v2);
        US0::US0_Some(v4, v5)
    } else {
        US0::US0_None
    }
}
fn closure6(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn() -> US0> {
    Rc::new(move || -> US0 {
        badge_try_28(v0.clone(), v1)
    })
}
fn link_close_32(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let mut v2: u8 = b'[';
    let v3: bool = at_is_13(v0.clone(), v1, v2);
    if v3 {
        let v4: u8 = b']';
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: i32 = find_char_12(v0.clone(), v4, v5);
        paren_close_26(v0, v6)
    } else {
        -1i32
    }
}
fn link_at_33(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<inline> {
    let v3: u8 = b']';
    let v4: i32 = v1.wrapping_add(1i32);
    let v5: i32 = find_char_12(v0.clone(), v3, v4);
    let v6: Rc<str> = run_of_7(v0.clone(), v4, v5);
    let v7: i32 = v5.wrapping_add(2i32);
    let v8: Rc<str> = run_of_7(v0, v7, v2);
    Rc::new(inline::inline_Link(v6, v8))
}
fn link_try_31(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: i32 = link_close_32(v0.clone(), v1);
    let v3: bool = v2 >= 0i32;
    if v3 {
        let v4: i32 = v2.wrapping_add(1i32);
        let v5: Rc<inline> = link_at_33(v0, v1, v2);
        US0::US0_Some(v4, v5)
    } else {
        US0::US0_None
    }
}
fn closure7(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn() -> US0> {
    Rc::new(move || -> US0 {
        link_try_31(v0.clone(), v1)
    })
}
fn linked_piece_23(mut v0: Rc<str>, mut v1: i32) -> US0 {
    let mut v2: US0 = image_try_24(v0.clone(), v1);
    let v3: Rc<dyn Fn() -> US0> = closure6(v0.clone(), v1);
    let v4: US0 = or_else_9(v3, v2);
    let v5: Rc<dyn Fn() -> US0> = closure7(v0, v1);
    or_else_9(v5, v4)
}
fn closure5(mut v0: Rc<str>, mut v1: i32) -> Rc<dyn Fn() -> US0> {
    Rc::new(move || -> US0 {
        linked_piece_23(v0.clone(), v1)
    })
}
fn piece_3(mut v0: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>>, mut v1: Rc<str>, mut v2: i32) -> (i32, Rc<inline>) {
    let v3: US0 = marked_8(v0, v1.clone(), v2);
    let v4: Rc<dyn Fn() -> US0> = closure5(v1.clone(), v2);
    let v5: US0 = or_else_9(v4, v3);
    match &v5 {
        US0::US0_None => {
            plain_piece_4(v1, v2)
        }
        US0::US0_Some(v6, v7) => {
            let v6: i32 = *v6;
            let v7: Rc<inline> = v7.clone();
            (v6, v7)
        }
    }
}
fn inlines_from_1(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<list_inline>) -> Rc<list_inline> {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            let v5: Rc<list_inline> = { thread_local!{ static CASE: Rc<list_inline> = Rc::new(list_inline::list_inline_Nil); } CASE.with(|case| case.clone()) };
            return rev_inlines_2(v2, v5);
        } else {
            let v7: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>> = closure1();
            let (mut v8, mut v9): (i32, Rc<inline>) = piece_3(v7, v0.clone(), v1);
            let v10: Rc<list_inline> = Rc::new(list_inline::list_inline_Cons(v9, v2));
            (v1, v2) = (v8, v10);
            continue;
        }
    }
}
fn parse_inline_0(mut v0: Rc<str>) -> Rc<list_inline> {
    let mut v1: i32 = 0i32;
    let mut v2: Rc<list_inline> = { thread_local!{ static CASE: Rc<list_inline> = Rc::new(list_inline::list_inline_Nil); } CASE.with(|case| case.clone()) };
    inlines_from_1(v0, v1, v2)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<list_inline>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<list_inline>> = Rc::new(move |mut v0: Rc<str>| -> Rc<list_inline> {
        parse_inline_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_inline_parse(v0: &str) -> Rc<list_inline> {
    closure0()(Rc::<str>::from(v0))
}
