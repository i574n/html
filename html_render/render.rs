#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unused_assignments, dead_code, non_snake_case, non_camel_case_types, unreachable_code, while_true)]
use std::cell::RefCell;
use std::rc::Rc;
use html_model::{level, inline, list_inline, list_list_inline, list_string, align, list_align, list_list_list_inline, block, list_block, markup};
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
enum UH7 {
    UH7_Nil,
    UH7_Cons(Rc<str>, Rc<str>, Rc<UH7>),
}
fn markup_text_3(mut v0: markup) -> Rc<str> {
    match v0 {
        markup::markup_Markup(v1) => {
            let v1: Rc<str> = v1;
            v1
        }
    }
}
fn joined_2(mut v0: markup, mut v1: markup) -> markup {
    let v2: Rc<str> = markup_text_3(v0);
    let v3: Rc<str> = markup_text_3(v1);
    let v4: Rc<str> = string_concat(&[&*v2, &*v3]);
    markup::markup_Markup(v4)
}
fn trusted_4(mut v0: Rc<str>) -> markup {
    markup::markup_Markup(v0)
}
fn closure1() -> Rc<dyn Fn(Rc<list_inline>) -> markup> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<list_inline>) -> markup> = Rc::new(move |mut v0: Rc<list_inline>| -> markup {
        inlines_html_8(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn attribute_14(mut v0: Rc<str>, mut v1: Rc<str>) -> Rc<str> {
    let v4: markup = html_model::html_model_escape(&v0);
    let v5: Rc<str> = markup_text_3(v4);
    let v7: Rc<str> = string_concat(&[" ", &*v1, "=\"", &*v5, "\""]);
    v7
}
fn attributes_13(mut v0: Rc<UH7>) -> Rc<str> {
    match Rc::unwrap_or_clone(v0) {
        UH7::UH7_Cons(v1, v2, v3) => {
            let v1: Rc<str> = v1;
            let v2: Rc<str> = v2;
            let v3: Rc<UH7> = v3;
            let v4: Rc<str> = attribute_14(v2, v1);
            let v5: Rc<str> = attributes_13(v3);
            let v6: Rc<str> = string_concat(&[&*v4, &*v5]);
            v6
        }
        UH7::UH7_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v7
        }
    }
}
fn element_12(mut v0: Rc<str>, mut v1: Rc<UH7>, mut v2: markup) -> markup {
    let v4: Rc<str> = attributes_13(v1);
    let v7: Rc<str> = markup_text_3(v2);
    let v11: Rc<str> = string_concat(&["<", &*v0, &*v4, ">", &*v7, "</", &*v0, ">"]);
    markup::markup_Markup(v11)
}
fn anchor_11(mut v0: Rc<str>, mut v1: markup) -> markup {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("a"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("href"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    let v5: Rc<UH7> = Rc::new(UH7::UH7_Cons(v3, v0, v4));
    element_12(v2, v5, v1)
}
fn linked_10(mut v0: Rc<str>, mut v1: Rc<str>) -> markup {
    let v2: markup = html_model::html_model_escape(&v0);
    anchor_11(v1, v2)
}
fn wrap_16(mut v0: Rc<str>, mut v1: markup) -> markup {
    let v2: Rc<UH7> = { thread_local!{ static CASE: Rc<UH7> = Rc::new(UH7::UH7_Nil); } CASE.with(|case| case.clone()) };
    element_12(v0, v2, v1)
}
fn code_of_15(mut v0: Rc<str>) -> markup {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("code"); } LIT.with(|lit| lit.clone()) };
    let v2: markup = html_model::html_model_escape(&v0);
    wrap_16(v1, v2)
}
fn quoted_19(mut v0: Rc<str>) -> markup {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: markup = trusted_4(v1.clone());
    let v3: markup = html_model::html_model_escape(&v0);
    let v4: markup = joined_2(v2, v3);
    let v5: markup = trusted_4(v1);
    joined_2(v4, v5)
}
fn image_18(mut v0: Rc<str>, mut v1: Rc<str>) -> markup {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src="); } LIT.with(|lit| lit.clone()) };
    let v3: markup = trusted_4(v2);
    let v4: markup = quoted_19(v1);
    let v5: markup = joined_2(v3, v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" alt="); } LIT.with(|lit| lit.clone()) };
    let v7: markup = trusted_4(v6);
    let v8: markup = joined_2(v5, v7);
    let v9: markup = quoted_19(v0);
    let v10: markup = joined_2(v8, v9);
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v12: markup = trusted_4(v11);
    joined_2(v10, v12)
}
fn pic_17(mut v0: Rc<str>, mut v1: Rc<str>) -> markup {
    image_18(v1, v0)
}
fn link_html_20(mut v0: Rc<str>, mut v1: Rc<str>) -> markup {
    linked_10(v1, v0)
}
fn badge_21(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<str>) -> markup {
    let v3: markup = image_18(v2, v1);
    anchor_11(v0, v3)
}
fn strong_22(mut v0: markup) -> markup {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("strong"); } LIT.with(|lit| lit.clone()) };
    wrap_16(v1, v0)
}
fn inline_by_9(mut v0: Rc<dyn Fn(Rc<list_inline>) -> markup>, mut v1: &Rc<inline>) -> markup {
    match &**v1 {
        inline::inline_AutoLink(v15) => {
            let v15: Rc<str> = v15.clone();
            linked_10(v15.clone(), v15)
        }
        inline::inline_Code(v4) => {
            let v4: Rc<str> = v4.clone();
            code_of_15(v4)
        }
        inline::inline_Image(v12, v13) => {
            let v12: Rc<str> = v12.clone();
            let v13: Rc<str> = v13.clone();
            pic_17(v13, v12)
        }
        inline::inline_Link(v9, v10) => {
            let v9: Rc<str> = v9.clone();
            let v10: Rc<str> = v10.clone();
            link_html_20(v10, v9)
        }
        inline::inline_LinkedImage(v17, v18, v19) => {
            let v17: Rc<str> = v17.clone();
            let v18: Rc<str> = v18.clone();
            let v19: Rc<str> = v19.clone();
            badge_21(v19, v18, v17)
        }
        inline::inline_Strong(v6) => {
            let v6: Rc<list_inline> = v6.clone();
            let v7: markup = v0(v6);
            strong_22(v7)
        }
        inline::inline_Text(v2) => {
            let v2: Rc<str> = v2.clone();
            let v3: markup = html_model::html_model_escape(&v2);
            v3
        }
    }
}
fn inlines_html_8(mut v0: Rc<list_inline>) -> markup {
    match Rc::unwrap_or_clone(v0) {
        list_inline::list_inline_Cons(v1, v2) => {
            let v1: Rc<inline> = v1;
            let v2: Rc<list_inline> = v2;
            let v3: Rc<dyn Fn(Rc<list_inline>) -> markup> = closure1();
            let v4: markup = inline_by_9(v3, &v1);
            let v5: markup = inlines_html_8(v2);
            joined_2(v4, v5)
        }
        list_inline::list_inline_Nil => {
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v7)
        }
    }
}
fn items_html_7(mut v0: Rc<list_list_inline>) -> markup {
    match Rc::unwrap_or_clone(v0) {
        list_list_inline::list_list_inline_Cons(v1, v2) => {
            let v1: Rc<list_inline> = v1;
            let v2: Rc<list_list_inline> = v2;
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) };
            let v4: markup = trusted_4(v3);
            let v5: markup = inlines_html_8(v1);
            let v6: markup = joined_2(v4, v5);
            let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) };
            let v8: markup = trusted_4(v7);
            let v9: markup = joined_2(v6, v8);
            let v10: markup = items_html_7(v2);
            joined_2(v9, v10)
        }
        list_list_inline::list_list_inline_Nil => {
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v12)
        }
    }
}
fn bullets_html_6(mut v0: Rc<list_list_inline>) -> markup {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ul>\n"); } LIT.with(|lit| lit.clone()) };
    let v2: markup = trusted_4(v1);
    let v3: markup = items_html_7(v0);
    let v4: markup = joined_2(v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
    let v6: markup = trusted_4(v5);
    joined_2(v4, v6)
}
fn code_open_24(mut v0: Rc<str>) -> markup {
    let v1: bool = &*v0 == "";
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
        trusted_4(v2)
    } else {
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code class=\"language-"); } LIT.with(|lit| lit.clone()) };
        let v5: markup = trusted_4(v4);
        let v6: markup = html_model::html_model_escape(&v0);
        let v7: markup = joined_2(v5, v6);
        let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\">"); } LIT.with(|lit| lit.clone()) };
        let v9: markup = trusted_4(v8);
        joined_2(v7, v9)
    }
}
fn code_html_25(mut v0: Rc<list_string>) -> markup {
    match Rc::unwrap_or_clone(v0) {
        list_string::list_string_Cons(v1, v2) => {
            let v1: Rc<str> = v1;
            let v2: Rc<list_string> = v2;
            let v3: markup = html_model::html_model_escape(&v1);
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v5: markup = trusted_4(v4);
            let v6: markup = joined_2(v3, v5);
            let v7: markup = code_html_25(v2);
            joined_2(v6, v7)
        }
        list_string::list_string_Nil => {
            let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v9)
        }
    }
}
fn pre_html_23(mut v0: Rc<str>, mut v1: Rc<list_string>) -> markup {
    let v2: markup = code_open_24(v0);
    let v3: markup = code_html_25(v1);
    let v4: markup = joined_2(v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
    let v6: markup = trusted_4(v5);
    joined_2(v4, v6)
}
fn heading_html_26(mut v0: Rc<list_inline>, mut v1: level) -> markup {
    let v2: Rc<str> = html_model::html_model_level_tag(v1);
    let v3: markup = inlines_html_8(v0);
    let v4: markup = wrap_16(v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
    let v6: markup = trusted_4(v5);
    joined_2(v4, v6)
}
fn run_of_31(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
fn digit_30(mut v0: i64) -> Rc<str> {
    let v1: i32 = (v0 as i32);
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0123456789"); } LIT.with(|lit| lit.clone()) };
    let v3: i32 = v1.wrapping_add(1i32);
    run_of_31(v2, v1, v3)
}
fn show_i64_29(mut v0: i64) -> Rc<str> {
    let v1: bool = v0 < 10i64;
    if v1 {
        digit_30(v0)
    } else {
        let v3: i64 = v0.wrapping_div(10i64);
        let v4: Rc<str> = show_i64_29(v3);
        let v5: i64 = v0.wrapping_rem(10i64);
        let v6: Rc<str> = digit_30(v5);
        let v7: Rc<str> = string_concat(&[&*v4, &*v6]);
        v7
    }
}
fn ordered_open_28(mut v0: i64) -> Rc<str> {
    let v1: bool = v0 == 1i64;
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ol>\n"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = show_i64_29(v0);
        let v5: Rc<str> = string_concat(&["<ol start=\"", &*v3, "\">\n"]);
        v5
    }
}
fn numbered_html_27(mut v0: Rc<list_list_inline>, mut v1: i64) -> markup {
    let v2: Rc<str> = ordered_open_28(v1);
    let v3: markup = trusted_4(v2);
    let v4: markup = items_html_7(v0);
    let v5: markup = joined_2(v3, v4);
    let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ol>\n"); } LIT.with(|lit| lit.clone()) };
    let v7: markup = trusted_4(v6);
    joined_2(v5, v7)
}
fn para_html_32(mut v0: Rc<list_inline>) -> markup {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) };
    let v2: markup = trusted_4(v1);
    let v3: markup = inlines_html_8(v0);
    let v4: markup = joined_2(v2, v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
    let v6: markup = trusted_4(v5);
    joined_2(v4, v6)
}
fn hr_33() -> markup {
    let mut v0: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<hr>\n"); } LIT.with(|lit| lit.clone()) };
    trusted_4(v0)
}
fn align_head_38(mut v0: &Rc<list_align>) -> (align, Rc<list_align>) {
    match &**v0 {
        list_align::list_align_Cons(v1, v2) => {
            let v1: align = v1.clone();
            let v2: Rc<list_align> = v2.clone();
            (v1, v2)
        }
        list_align::list_align_Nil => {
            let v3: align = align::align_AlignNone;
            let v4: Rc<list_align> = { thread_local!{ static CASE: Rc<list_align> = Rc::new(list_align::list_align_Nil); } CASE.with(|case| case.clone()) };
            (v3, v4)
        }
    }
}
fn align_attr_40(mut v0: align) -> Rc<str> {
    match v0 {
        align::align_AlignCenter => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: center\""); } LIT.with(|lit| lit.clone()) };
            v3
        }
        align::align_AlignLeft => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: left\""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        align::align_AlignNone => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        align::align_AlignRight => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: right\""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn cell_html_39(mut v0: Rc<str>, mut v1: align, mut v2: Rc<list_inline>) -> markup {
    let v4: Rc<str> = align_attr_40(v1);
    let v6: Rc<str> = string_concat(&["<", &*v0, &*v4, ">"]);
    let v7: markup = trusted_4(v6);
    let v8: markup = inlines_html_8(v2);
    let v9: markup = joined_2(v7, v8);
    let v11: Rc<str> = string_concat(&["</", &*v0, ">"]);
    let v12: markup = trusted_4(v11);
    joined_2(v9, v12)
}
fn cells_html_37(mut v0: Rc<str>, mut v1: Rc<list_align>, mut v2: Rc<list_list_inline>) -> markup {
    match Rc::unwrap_or_clone(v2) {
        list_list_inline::list_list_inline_Cons(v5, v6) => {
            let v5: Rc<list_inline> = v5;
            let v6: Rc<list_list_inline> = v6;
            let (mut v7, mut v8): (align, Rc<list_align>) = align_head_38(&v1);
            let v9: markup = cell_html_39(v0.clone(), v7, v5);
            let v10: markup = cells_html_37(v0, v8, v6);
            joined_2(v9, v10)
        }
        list_list_inline::list_list_inline_Nil => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v3)
        }
    }
}
fn row_html_36(mut v0: Rc<str>, mut v1: Rc<list_align>, mut v2: Rc<list_list_inline>) -> markup {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<tr>"); } LIT.with(|lit| lit.clone()) };
    let v4: markup = trusted_4(v3);
    let v5: markup = cells_html_37(v0, v1, v2);
    let v6: markup = joined_2(v4, v5);
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tr>\n"); } LIT.with(|lit| lit.clone()) };
    let v8: markup = trusted_4(v7);
    joined_2(v6, v8)
}
fn rows_html_41(mut v0: Rc<list_align>, mut v1: Rc<list_list_list_inline>) -> markup {
    match Rc::unwrap_or_clone(v1) {
        list_list_list_inline::list_list_list_inline_Cons(v2, v3) => {
            let v2: Rc<list_list_inline> = v2;
            let v3: Rc<list_list_list_inline> = v3;
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
            let v5: markup = row_html_36(v4, v0.clone(), v2);
            let v6: markup = rows_html_41(v0, v3);
            joined_2(v5, v6)
        }
        list_list_list_inline::list_list_list_inline_Nil => {
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v8)
        }
    }
}
fn table_html_35(mut v0: Rc<list_align>, mut v1: Rc<list_list_inline>, mut v2: Rc<list_list_list_inline>) -> markup {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n<thead>\n"); } LIT.with(|lit| lit.clone()) };
    let v4: markup = trusted_4(v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
    let v6: markup = row_html_36(v5, v0.clone(), v1);
    let v7: markup = joined_2(v4, v6);
    let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</thead>\n<tbody>\n"); } LIT.with(|lit| lit.clone()) };
    let v9: markup = trusted_4(v8);
    let v10: markup = joined_2(v7, v9);
    let v11: markup = rows_html_41(v0, v2);
    let v12: markup = joined_2(v10, v11);
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tbody>\n</table>\n"); } LIT.with(|lit| lit.clone()) };
    let v14: markup = trusted_4(v13);
    joined_2(v12, v14)
}
fn table_of_34(mut v0: Rc<list_list_list_inline>, mut v1: Rc<list_list_inline>, mut v2: Rc<list_align>) -> markup {
    table_html_35(v2, v1, v0)
}
fn block_html_5(mut v0: block) -> markup {
    match v0 {
        block::block_Bullets(v6) => {
            let v6: Rc<list_list_inline> = v6;
            bullets_html_6(v6)
        }
        block::block_CodeBlock(v11, v12) => {
            let v11: Rc<str> = v11;
            let v12: Rc<list_string> = v12;
            pre_html_23(v11, v12)
        }
        block::block_Heading(v1, v2) => {
            let v1: level = v1;
            let v2: Rc<list_inline> = v2;
            heading_html_26(v2, v1)
        }
        block::block_Numbered(v8, v9) => {
            let v8: i64 = v8;
            let v9: Rc<list_list_inline> = v9;
            numbered_html_27(v9, v8)
        }
        block::block_Paragraph(v4) => {
            let v4: Rc<list_inline> = v4;
            para_html_32(v4)
        }
        block::block_Rule => {
            hr_33()
        }
        block::block_Table(v15, v16, v17) => {
            let v15: Rc<list_align> = v15;
            let v16: Rc<list_list_inline> = v16;
            let v17: Rc<list_list_list_inline> = v17;
            table_of_34(v17, v16, v15)
        }
    }
}
fn blocks_html_1(mut v0: Rc<list_block>) -> markup {
    match Rc::unwrap_or_clone(v0) {
        list_block::list_block_Cons(v1, v2) => {
            let v1: block = v1;
            let v2: Rc<list_block> = v2;
            let v3: markup = block_html_5(v1);
            let v4: markup = blocks_html_1(v2);
            joined_2(v3, v4)
        }
        list_block::list_block_Nil => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v6)
        }
    }
}
fn markdown_html_0(mut v0: Rc<str>) -> markup {
    let v1: Rc<list_block> = html_parse::html_parse_markdown(&v0);
    blocks_html_1(v1)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> markup> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> markup> = Rc::new(move |mut v0: Rc<str>| -> markup {
        markdown_html_0(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
fn md_42(mut v0: Rc<str>) -> Rc<str> {
    let v1: markup = markdown_html_0(v0);
    markup_text_3(v1)
}
fn closure2() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        md_42(v0)
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_render_markdown(v0: &str) -> markup {
    closure0()(Rc::<str>::from(v0))
}
pub fn html_render_md(v0: &str) -> Rc<str> {
    closure2()(Rc::<str>::from(v0))
}
