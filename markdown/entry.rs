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
enum US0 {
    US0_Markup(Rc<str>),
}
#[derive(Clone)]
enum US2 {
    US2_H1,
    US2_H2,
    US2_H3,
    US2_H4,
    US2_H5,
    US2_H6,
}
#[derive(Clone)]
enum UH2 {
    UH2_Text(Rc<str>),
    UH2_Code(Rc<str>),
    UH2_Strong(Rc<UH1>),
    UH2_Link(Rc<str>, Rc<str>),
    UH2_Image(Rc<str>, Rc<str>),
    UH2_AutoLink(Rc<str>),
    UH2_LinkedImage(Rc<str>, Rc<str>, Rc<str>),
}
#[derive(Clone)]
enum UH1 {
    UH1_Nil,
    UH1_Cons(Rc<UH2>, Rc<UH1>),
}
#[derive(Clone)]
enum UH3 {
    UH3_Nil,
    UH3_Cons(Rc<UH1>, Rc<UH3>),
}
#[derive(Clone)]
enum UH4 {
    UH4_Nil,
    UH4_Cons(Rc<str>, Rc<UH4>),
}
#[derive(Clone)]
enum US3 {
    US3_AlignNone,
    US3_AlignLeft,
    US3_AlignCenter,
    US3_AlignRight,
}
#[derive(Clone)]
enum UH5 {
    UH5_Nil,
    UH5_Cons(US3, Rc<UH5>),
}
#[derive(Clone)]
enum UH6 {
    UH6_Nil,
    UH6_Cons(Rc<UH3>, Rc<UH6>),
}
#[derive(Clone)]
enum US1 {
    US1_Heading(US2, Rc<UH1>),
    US1_Paragraph(Rc<UH1>),
    US1_Bullets(Rc<UH3>),
    US1_Numbered(i64, Rc<UH3>),
    US1_CodeBlock(Rc<UH4>),
    US1_Rule,
    US1_Table(Rc<UH5>, Rc<UH3>, Rc<UH6>),
}
#[derive(Clone)]
enum UH0 {
    UH0_Nil,
    UH0_Cons(US1, Rc<UH0>),
}
#[derive(Clone)]
enum US4 {
    US4_Idle,
    US4_InParagraph(Rc<UH1>),
    US4_InBullets(Rc<UH3>, Rc<UH1>),
    US4_InNumbered(i64, Rc<UH3>, Rc<UH1>),
    US4_InCode(Rc<UH4>),
    US4_InTable(Rc<UH5>, Rc<UH3>, Rc<UH6>),
}
#[derive(Clone)]
enum US5 {
    US5_Blank,
    US5_Fence,
    US5_HeadingLine(US2, Rc<str>),
    US5_RuleLine,
    US5_RowLine(Rc<str>),
    US5_BulletLine(Rc<str>),
    US5_NumberLine(i64, Rc<str>),
    US5_PlainLine(Rc<str>),
}
#[derive(Clone)]
enum US6 {
    US6_Some(US2, Rc<str>),
    US6_None,
}
#[derive(Clone)]
enum US7 {
    US7_Some(US2),
    US7_None,
}
#[derive(Clone)]
enum US8 {
    US8_Some(Rc<str>),
    US8_None,
}
#[derive(Clone)]
enum US9 {
    US9_Some(i64, Rc<str>),
    US9_None,
}
#[derive(Clone)]
enum US10 {
    US10_Some(Rc<UH5>),
    US10_None,
}
fn markup_text_1(mut v0: US0) -> Rc<str> {
    match &v0 {
        US0::US0_Markup(v1) => {
            let v1: Rc<str> = v1.clone();
            v1
        }
    }
}
fn trusted_4(mut v0: Rc<str>) -> US0 {
    US0::US0_Markup(v0)
}
fn align_attr_10(mut v0: US3) -> Rc<str> {
    match &v0 {
        US3::US3_AlignCenter => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: center\""); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US3::US3_AlignLeft => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: left\""); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US3::US3_AlignNone => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US3::US3_AlignRight => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" style=\"text-align: right\""); } LIT.with(|lit| lit.clone()) };
            v4
        }
    }
}
fn run_of_17(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<str> {
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
fn entity_18(mut v0: u8) -> Rc<str> {
    let v1: bool = v0 == b'&';
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
fn escape_from_16(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<str>) -> Rc<str> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = run_of_17(v0.clone(), v2, v1);
            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v6));
            return v7.clone();
        } else {
            let v8: u8 = v0.clone().as_bytes()[v1 as usize];
            let v9: Rc<str> = entity_18(v8);
            let v10: bool = &*v9 == "";
            if v10 {
                let v11: i32 = v1.wrapping_add(1i32);
                (v0, v1, v2, v3) = (v0.clone(), v11, v2, v3.clone());
                continue;
            } else {
                let v13: i32 = v1.wrapping_add(1i32);
                let v14: Rc<str> = run_of_17(v0.clone(), v2, v1);
                let v16: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v3, v14)), v9));
                (v0, v1, v2, v3) = (v0.clone(), v13, v13, v16.clone());
                continue;
            }
        }
    }
}
fn escape_15(mut v0: Rc<str>) -> US0 {
    let v1: i32 = 0i32;
    let v2: i32 = 0i32;
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = escape_from_16(v0, v1, v2, v3);
    US0::US0_Markup(v4)
}
fn quoted_14(mut v0: Rc<str>) -> US0 {
    let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\""); } LIT.with(|lit| lit.clone()) };
    let v2: US0 = trusted_4(v1.clone());
    let v3: US0 = escape_15(v0);
    let v9: US0 = match &v2 {
        US0::US0_Markup(v4) => {
            let v4: Rc<str> = v4.clone();
            match &v3 {
                US0::US0_Markup(v5) => {
                    let v5: Rc<str> = v5.clone();
                    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
                    US0::US0_Markup(v6)
                }
            }
        }
    };
    let v10: US0 = trusted_4(v1);
    match &v9 {
        US0::US0_Markup(v11) => {
            let v11: Rc<str> = v11.clone();
            match &v10 {
                US0::US0_Markup(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                    US0::US0_Markup(v13)
                }
            }
        }
    }
}
fn anchor_13(mut v0: Rc<str>, mut v1: US0) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<a href="); } LIT.with(|lit| lit.clone()) };
    let v3: US0 = trusted_4(v2);
    let v4: US0 = quoted_14(v0);
    let v10: US0 = match &v3 {
        US0::US0_Markup(v5) => {
            let v5: Rc<str> = v5.clone();
            match &v4 {
                US0::US0_Markup(v6) => {
                    let v6: Rc<str> = v6.clone();
                    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
                    US0::US0_Markup(v7)
                }
            }
        }
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v12: US0 = trusted_4(v11);
    let v18: US0 = match &v10 {
        US0::US0_Markup(v13) => {
            let v13: Rc<str> = v13.clone();
            match &v12 {
                US0::US0_Markup(v14) => {
                    let v14: Rc<str> = v14.clone();
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                    US0::US0_Markup(v15)
                }
            }
        }
    };
    let v24: US0 = match &v18 {
        US0::US0_Markup(v19) => {
            let v19: Rc<str> = v19.clone();
            match &v1 {
                US0::US0_Markup(v20) => {
                    let v20: Rc<str> = v20.clone();
                    let v21: Rc<str> = Rc::<str>::from(format!("{}{}", v19, v20));
                    US0::US0_Markup(v21)
                }
            }
        }
    };
    let v25: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</a>"); } LIT.with(|lit| lit.clone()) };
    let v26: US0 = trusted_4(v25);
    match &v24 {
        US0::US0_Markup(v27) => {
            let v27: Rc<str> = v27.clone();
            match &v26 {
                US0::US0_Markup(v28) => {
                    let v28: Rc<str> = v28.clone();
                    let v29: Rc<str> = Rc::<str>::from(format!("{}{}", v27, v28));
                    US0::US0_Markup(v29)
                }
            }
        }
    }
}
fn wrap_19(mut v0: Rc<str>, mut v1: US0) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<str> = Rc::<str>::from(format!("{}{}", v2, v0));
    let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v4));
    let v6: US0 = trusted_4(v5);
    let v12: US0 = match &v6 {
        US0::US0_Markup(v7) => {
            let v7: Rc<str> = v7.clone();
            match &v1 {
                US0::US0_Markup(v8) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                    US0::US0_Markup(v9)
                }
            }
        }
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v13, v0)), v4));
    let v16: US0 = trusted_4(v15);
    match &v12 {
        US0::US0_Markup(v17) => {
            let v17: Rc<str> = v17.clone();
            match &v16 {
                US0::US0_Markup(v18) => {
                    let v18: Rc<str> = v18.clone();
                    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", v17, v18));
                    US0::US0_Markup(v19)
                }
            }
        }
    }
}
fn image_20(mut v0: Rc<str>, mut v1: Rc<str>) -> US0 {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<img src="); } LIT.with(|lit| lit.clone()) };
    let v3: US0 = trusted_4(v2);
    let v4: US0 = quoted_14(v1);
    let v10: US0 = match &v3 {
        US0::US0_Markup(v5) => {
            let v5: Rc<str> = v5.clone();
            match &v4 {
                US0::US0_Markup(v6) => {
                    let v6: Rc<str> = v6.clone();
                    let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
                    US0::US0_Markup(v7)
                }
            }
        }
    };
    let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" alt="); } LIT.with(|lit| lit.clone()) };
    let v12: US0 = trusted_4(v11);
    let v18: US0 = match &v10 {
        US0::US0_Markup(v13) => {
            let v13: Rc<str> = v13.clone();
            match &v12 {
                US0::US0_Markup(v14) => {
                    let v14: Rc<str> = v14.clone();
                    let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                    US0::US0_Markup(v15)
                }
            }
        }
    };
    let v19: US0 = quoted_14(v0);
    let v25: US0 = match &v18 {
        US0::US0_Markup(v20) => {
            let v20: Rc<str> = v20.clone();
            match &v19 {
                US0::US0_Markup(v21) => {
                    let v21: Rc<str> = v21.clone();
                    let v22: Rc<str> = Rc::<str>::from(format!("{}{}", v20, v21));
                    US0::US0_Markup(v22)
                }
            }
        }
    };
    let v26: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v27: US0 = trusted_4(v26);
    match &v25 {
        US0::US0_Markup(v28) => {
            let v28: Rc<str> = v28.clone();
            match &v27 {
                US0::US0_Markup(v29) => {
                    let v29: Rc<str> = v29.clone();
                    let v30: Rc<str> = Rc::<str>::from(format!("{}{}", v28, v29));
                    US0::US0_Markup(v30)
                }
            }
        }
    }
}
fn inline_html_12(mut v0: Rc<UH2>) -> US0 {
    match &*v0 {
        UH2::UH2_AutoLink(v18) => {
            let v18: Rc<str> = v18.clone();
            let v19: US0 = escape_15(v18.clone());
            anchor_13(v18, v19)
        }
        UH2::UH2_Code(v3) => {
            let v3: Rc<str> = v3.clone();
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("code"); } LIT.with(|lit| lit.clone()) };
            let v5: US0 = escape_15(v3);
            wrap_19(v4, v5)
        }
        UH2::UH2_Image(v15, v16) => {
            let v15: Rc<str> = v15.clone();
            let v16: Rc<str> = v16.clone();
            image_20(v15, v16)
        }
        UH2::UH2_Link(v11, v12) => {
            let v11: Rc<str> = v11.clone();
            let v12: Rc<str> = v12.clone();
            let v13: US0 = escape_15(v11);
            anchor_13(v12, v13)
        }
        UH2::UH2_LinkedImage(v21, v22, v23) => {
            let v21: Rc<str> = v21.clone();
            let v22: Rc<str> = v22.clone();
            let v23: Rc<str> = v23.clone();
            let v24: US0 = image_20(v21, v22);
            anchor_13(v23, v24)
        }
        UH2::UH2_Strong(v7) => {
            let v7: Rc<UH1> = v7.clone();
            let v8: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("strong"); } LIT.with(|lit| lit.clone()) };
            let v9: US0 = inlines_html_11(v7);
            wrap_19(v8, v9)
        }
        UH2::UH2_Text(v1) => {
            let v1: Rc<str> = v1.clone();
            escape_15(v1)
        }
    }
}
fn inlines_html_11(mut v0: Rc<UH1>) -> US0 {
    match &*v0 {
        UH1::UH1_Cons(v1, v2) => {
            let v1: Rc<UH2> = v1.clone();
            let v2: Rc<UH1> = v2.clone();
            let v3: US0 = inline_html_12(v1);
            let v4: US0 = inlines_html_11(v2);
            match &v3 {
                US0::US0_Markup(v5) => {
                    let v5: Rc<str> = v5.clone();
                    match &v4 {
                        US0::US0_Markup(v6) => {
                            let v6: Rc<str> = v6.clone();
                            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
                            US0::US0_Markup(v7)
                        }
                    }
                }
            }
        }
        UH1::UH1_Nil => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v11)
        }
    }
}
fn cell_html_9(mut v0: Rc<str>, mut v1: US3, mut v2: Rc<UH1>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<"); } LIT.with(|lit| lit.clone()) };
    let v4: Rc<str> = Rc::<str>::from(format!("{}{}", v3, v0));
    let v5: Rc<str> = align_attr_10(v1);
    let v6: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v5));
    let v7: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(">"); } LIT.with(|lit| lit.clone()) };
    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
    let v9: US0 = trusted_4(v8);
    let v10: US0 = inlines_html_11(v2);
    let v16: US0 = match &v9 {
        US0::US0_Markup(v11) => {
            let v11: Rc<str> = v11.clone();
            match &v10 {
                US0::US0_Markup(v12) => {
                    let v12: Rc<str> = v12.clone();
                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                    US0::US0_Markup(v13)
                }
            }
        }
    };
    let v17: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</"); } LIT.with(|lit| lit.clone()) };
    let v19: Rc<str> = Rc::<str>::from(format!("{}{}", Rc::<str>::from(format!("{}{}", v17, v0)), v7));
    let v20: US0 = trusted_4(v19);
    match &v16 {
        US0::US0_Markup(v21) => {
            let v21: Rc<str> = v21.clone();
            match &v20 {
                US0::US0_Markup(v22) => {
                    let v22: Rc<str> = v22.clone();
                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                    US0::US0_Markup(v23)
                }
            }
        }
    }
}
fn cells_html_8(mut v0: Rc<str>, mut v1: Rc<UH5>, mut v2: Rc<UH3>) -> US0 {
    match &*v2 {
        UH3::UH3_Cons(v5, v6) => {
            let v5: Rc<UH1> = v5.clone();
            let v6: Rc<UH3> = v6.clone();
            match &*v1 {
                UH5::UH5_Cons(v7, v8) => {
                    let v7: US3 = v7.clone();
                    let v8: Rc<UH5> = v8.clone();
                    let v9: US0 = cell_html_9(v0.clone(), v7, v5.clone());
                    let v10: US0 = cells_html_8(v0.clone(), v8, v6.clone());
                    match &v9 {
                        US0::US0_Markup(v11) => {
                            let v11: Rc<str> = v11.clone();
                            match &v10 {
                                US0::US0_Markup(v12) => {
                                    let v12: Rc<str> = v12.clone();
                                    let v13: Rc<str> = Rc::<str>::from(format!("{}{}", v11, v12));
                                    US0::US0_Markup(v13)
                                }
                            }
                        }
                    }
                }
                UH5::UH5_Nil => {
                    let v17: US3 = US3::US3_AlignNone;
                    let v18: US0 = cell_html_9(v0.clone(), v17, v5);
                    let v19: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                    let v20: US0 = cells_html_8(v0, v19, v6);
                    match &v18 {
                        US0::US0_Markup(v21) => {
                            let v21: Rc<str> = v21.clone();
                            match &v20 {
                                US0::US0_Markup(v22) => {
                                    let v22: Rc<str> = v22.clone();
                                    let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                                    US0::US0_Markup(v23)
                                }
                            }
                        }
                    }
                }
            }
        }
        UH3::UH3_Nil => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v3)
        }
    }
}
fn row_html_7(mut v0: Rc<str>, mut v1: Rc<UH5>, mut v2: Rc<UH3>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<tr>"); } LIT.with(|lit| lit.clone()) };
    let v4: US0 = trusted_4(v3);
    let v5: US0 = cells_html_8(v0, v1, v2);
    let v11: US0 = match &v4 {
        US0::US0_Markup(v6) => {
            let v6: Rc<str> = v6.clone();
            match &v5 {
                US0::US0_Markup(v7) => {
                    let v7: Rc<str> = v7.clone();
                    let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                    US0::US0_Markup(v8)
                }
            }
        }
    };
    let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tr>\n"); } LIT.with(|lit| lit.clone()) };
    let v13: US0 = trusted_4(v12);
    match &v11 {
        US0::US0_Markup(v14) => {
            let v14: Rc<str> = v14.clone();
            match &v13 {
                US0::US0_Markup(v15) => {
                    let v15: Rc<str> = v15.clone();
                    let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                    US0::US0_Markup(v16)
                }
            }
        }
    }
}
fn rows_html_21(mut v0: Rc<UH5>, mut v1: Rc<UH6>) -> US0 {
    match &*v1 {
        UH6::UH6_Cons(v2, v3) => {
            let v2: Rc<UH3> = v2.clone();
            let v3: Rc<UH6> = v3.clone();
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("td"); } LIT.with(|lit| lit.clone()) };
            let v5: US0 = row_html_7(v4, v0.clone(), v2);
            let v6: US0 = rows_html_21(v0, v3);
            match &v5 {
                US0::US0_Markup(v7) => {
                    let v7: Rc<str> = v7.clone();
                    match &v6 {
                        US0::US0_Markup(v8) => {
                            let v8: Rc<str> = v8.clone();
                            let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                            US0::US0_Markup(v9)
                        }
                    }
                }
            }
        }
        UH6::UH6_Nil => {
            let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v13)
        }
    }
}
fn table_html_6(mut v0: Rc<UH5>, mut v1: Rc<UH3>, mut v2: Rc<UH6>) -> US0 {
    let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<table>\n<thead>\n"); } LIT.with(|lit| lit.clone()) };
    let v4: US0 = trusted_4(v3);
    let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("th"); } LIT.with(|lit| lit.clone()) };
    let v6: US0 = row_html_7(v5, v0.clone(), v1);
    let v12: US0 = match &v4 {
        US0::US0_Markup(v7) => {
            let v7: Rc<str> = v7.clone();
            match &v6 {
                US0::US0_Markup(v8) => {
                    let v8: Rc<str> = v8.clone();
                    let v9: Rc<str> = Rc::<str>::from(format!("{}{}", v7, v8));
                    US0::US0_Markup(v9)
                }
            }
        }
    };
    let v13: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</thead>\n<tbody>\n"); } LIT.with(|lit| lit.clone()) };
    let v14: US0 = trusted_4(v13);
    let v20: US0 = match &v12 {
        US0::US0_Markup(v15) => {
            let v15: Rc<str> = v15.clone();
            match &v14 {
                US0::US0_Markup(v16) => {
                    let v16: Rc<str> = v16.clone();
                    let v17: Rc<str> = Rc::<str>::from(format!("{}{}", v15, v16));
                    US0::US0_Markup(v17)
                }
            }
        }
    };
    let v21: US0 = rows_html_21(v0, v2);
    let v27: US0 = match &v20 {
        US0::US0_Markup(v22) => {
            let v22: Rc<str> = v22.clone();
            match &v21 {
                US0::US0_Markup(v23) => {
                    let v23: Rc<str> = v23.clone();
                    let v24: Rc<str> = Rc::<str>::from(format!("{}{}", v22, v23));
                    US0::US0_Markup(v24)
                }
            }
        }
    };
    let v28: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</tbody>\n</table>\n"); } LIT.with(|lit| lit.clone()) };
    let v29: US0 = trusted_4(v28);
    match &v27 {
        US0::US0_Markup(v30) => {
            let v30: Rc<str> = v30.clone();
            match &v29 {
                US0::US0_Markup(v31) => {
                    let v31: Rc<str> = v31.clone();
                    let v32: Rc<str> = Rc::<str>::from(format!("{}{}", v30, v31));
                    US0::US0_Markup(v32)
                }
            }
        }
    }
}
fn items_html_22(mut v0: Rc<UH3>) -> US0 {
    match &*v0 {
        UH3::UH3_Cons(v1, v2) => {
            let v1: Rc<UH1> = v1.clone();
            let v2: Rc<UH3> = v2.clone();
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<li>"); } LIT.with(|lit| lit.clone()) };
            let v4: US0 = trusted_4(v3);
            let v5: US0 = inlines_html_11(v1);
            let v11: US0 = match &v4 {
                US0::US0_Markup(v6) => {
                    let v6: Rc<str> = v6.clone();
                    match &v5 {
                        US0::US0_Markup(v7) => {
                            let v7: Rc<str> = v7.clone();
                            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                            US0::US0_Markup(v8)
                        }
                    }
                }
            };
            let v12: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</li>\n"); } LIT.with(|lit| lit.clone()) };
            let v13: US0 = trusted_4(v12);
            let v19: US0 = match &v11 {
                US0::US0_Markup(v14) => {
                    let v14: Rc<str> = v14.clone();
                    match &v13 {
                        US0::US0_Markup(v15) => {
                            let v15: Rc<str> = v15.clone();
                            let v16: Rc<str> = Rc::<str>::from(format!("{}{}", v14, v15));
                            US0::US0_Markup(v16)
                        }
                    }
                }
            };
            let v20: US0 = items_html_22(v2);
            match &v19 {
                US0::US0_Markup(v21) => {
                    let v21: Rc<str> = v21.clone();
                    match &v20 {
                        US0::US0_Markup(v22) => {
                            let v22: Rc<str> = v22.clone();
                            let v23: Rc<str> = Rc::<str>::from(format!("{}{}", v21, v22));
                            US0::US0_Markup(v23)
                        }
                    }
                }
            }
        }
        UH3::UH3_Nil => {
            let v27: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v27)
        }
    }
}
fn code_html_23(mut v0: Rc<UH4>) -> US0 {
    match &*v0 {
        UH4::UH4_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH4> = v2.clone();
            let v3: US0 = escape_15(v1);
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v5: US0 = trusted_4(v4);
            let v11: US0 = match &v3 {
                US0::US0_Markup(v6) => {
                    let v6: Rc<str> = v6.clone();
                    match &v5 {
                        US0::US0_Markup(v7) => {
                            let v7: Rc<str> = v7.clone();
                            let v8: Rc<str> = Rc::<str>::from(format!("{}{}", v6, v7));
                            US0::US0_Markup(v8)
                        }
                    }
                }
            };
            let v12: US0 = code_html_23(v2);
            match &v11 {
                US0::US0_Markup(v13) => {
                    let v13: Rc<str> = v13.clone();
                    match &v12 {
                        US0::US0_Markup(v14) => {
                            let v14: Rc<str> = v14.clone();
                            let v15: Rc<str> = Rc::<str>::from(format!("{}{}", v13, v14));
                            US0::US0_Markup(v15)
                        }
                    }
                }
            }
        }
        UH4::UH4_Nil => {
            let v19: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v19)
        }
    }
}
fn level_tag_24(mut v0: US2) -> Rc<str> {
    match &v0 {
        US2::US2_H1 => {
            let v1: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h1"); } LIT.with(|lit| lit.clone()) };
            v1
        }
        US2::US2_H2 => {
            let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h2"); } LIT.with(|lit| lit.clone()) };
            v2
        }
        US2::US2_H3 => {
            let v3: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h3"); } LIT.with(|lit| lit.clone()) };
            v3
        }
        US2::US2_H4 => {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h4"); } LIT.with(|lit| lit.clone()) };
            v4
        }
        US2::US2_H5 => {
            let v5: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h5"); } LIT.with(|lit| lit.clone()) };
            v5
        }
        US2::US2_H6 => {
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("h6"); } LIT.with(|lit| lit.clone()) };
            v6
        }
    }
}
fn digit_27(mut v0: i64) -> Rc<str> {
    let v1: bool = v0 == 0i64;
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("0"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: bool = v0 == 1i64;
        if v3 {
            let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("1"); } LIT.with(|lit| lit.clone()) };
            v4
        } else {
            let v5: bool = v0 == 2i64;
            if v5 {
                let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("2"); } LIT.with(|lit| lit.clone()) };
                v6
            } else {
                let v7: bool = v0 == 3i64;
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
                                let v14: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("6"); } LIT.with(|lit| lit.clone()) };
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
fn show_i64_26(mut v0: i64) -> Rc<str> {
    let v1: bool = v0 < 10i64;
    if v1 {
        digit_27(v0)
    } else {
        let v3: i64 = v0.wrapping_div(10i64);
        let v4: Rc<str> = show_i64_26(v3);
        let v5: i64 = v0.wrapping_rem(10i64);
        let v6: Rc<str> = digit_27(v5);
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v6));
        v7
    }
}
fn ordered_open_25(mut v0: i64) -> Rc<str> {
    let v1: bool = v0 == 1i64;
    if v1 {
        let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ol>\n"); } LIT.with(|lit| lit.clone()) };
        v2
    } else {
        let v3: Rc<str> = show_i64_26(v0);
        let v4: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ol start=\""); } LIT.with(|lit| lit.clone()) };
        let v5: Rc<str> = Rc::<str>::from(format!("{}{}", v4, v3));
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\">\n"); } LIT.with(|lit| lit.clone()) };
        let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
        v7
    }
}
fn block_html_5(mut v0: US1) -> US0 {
    match &v0 {
        US1::US1_Bullets(v32) => {
            let v32: Rc<UH3> = v32.clone();
            let v33: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<ul>\n"); } LIT.with(|lit| lit.clone()) };
            let v34: US0 = trusted_4(v33);
            let v35: US0 = items_html_22(v32);
            let v41: US0 = match &v34 {
                US0::US0_Markup(v36) => {
                    let v36: Rc<str> = v36.clone();
                    match &v35 {
                        US0::US0_Markup(v37) => {
                            let v37: Rc<str> = v37.clone();
                            let v38: Rc<str> = Rc::<str>::from(format!("{}{}", v36, v37));
                            US0::US0_Markup(v38)
                        }
                    }
                }
            };
            let v42: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ul>\n"); } LIT.with(|lit| lit.clone()) };
            let v43: US0 = trusted_4(v42);
            match &v41 {
                US0::US0_Markup(v44) => {
                    let v44: Rc<str> = v44.clone();
                    match &v43 {
                        US0::US0_Markup(v45) => {
                            let v45: Rc<str> = v45.clone();
                            let v46: Rc<str> = Rc::<str>::from(format!("{}{}", v44, v45));
                            US0::US0_Markup(v46)
                        }
                    }
                }
            }
        }
        US1::US1_CodeBlock(v69) => {
            let v69: Rc<UH4> = v69.clone();
            let v70: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<pre><code>"); } LIT.with(|lit| lit.clone()) };
            let v71: US0 = trusted_4(v70);
            let v72: US0 = code_html_23(v69);
            let v78: US0 = match &v71 {
                US0::US0_Markup(v73) => {
                    let v73: Rc<str> = v73.clone();
                    match &v72 {
                        US0::US0_Markup(v74) => {
                            let v74: Rc<str> = v74.clone();
                            let v75: Rc<str> = Rc::<str>::from(format!("{}{}", v73, v74));
                            US0::US0_Markup(v75)
                        }
                    }
                }
            };
            let v79: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</code></pre>\n"); } LIT.with(|lit| lit.clone()) };
            let v80: US0 = trusted_4(v79);
            match &v78 {
                US0::US0_Markup(v81) => {
                    let v81: Rc<str> = v81.clone();
                    match &v80 {
                        US0::US0_Markup(v82) => {
                            let v82: Rc<str> = v82.clone();
                            let v83: Rc<str> = Rc::<str>::from(format!("{}{}", v81, v82));
                            US0::US0_Markup(v83)
                        }
                    }
                }
            }
        }
        US1::US1_Heading(v1, v2) => {
            let v1: US2 = v1.clone();
            let v2: Rc<UH1> = v2.clone();
            let v3: Rc<str> = level_tag_24(v1);
            let v4: US0 = inlines_html_11(v2);
            let v5: US0 = wrap_19(v3, v4);
            let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("\n"); } LIT.with(|lit| lit.clone()) };
            let v7: US0 = trusted_4(v6);
            match &v5 {
                US0::US0_Markup(v8) => {
                    let v8: Rc<str> = v8.clone();
                    match &v7 {
                        US0::US0_Markup(v9) => {
                            let v9: Rc<str> = v9.clone();
                            let v10: Rc<str> = Rc::<str>::from(format!("{}{}", v8, v9));
                            US0::US0_Markup(v10)
                        }
                    }
                }
            }
        }
        US1::US1_Numbered(v50, v51) => {
            let v50: i64 = *v50;
            let v51: Rc<UH3> = v51.clone();
            let v52: Rc<str> = ordered_open_25(v50);
            let v53: US0 = trusted_4(v52);
            let v54: US0 = items_html_22(v51);
            let v60: US0 = match &v53 {
                US0::US0_Markup(v55) => {
                    let v55: Rc<str> = v55.clone();
                    match &v54 {
                        US0::US0_Markup(v56) => {
                            let v56: Rc<str> = v56.clone();
                            let v57: Rc<str> = Rc::<str>::from(format!("{}{}", v55, v56));
                            US0::US0_Markup(v57)
                        }
                    }
                }
            };
            let v61: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</ol>\n"); } LIT.with(|lit| lit.clone()) };
            let v62: US0 = trusted_4(v61);
            match &v60 {
                US0::US0_Markup(v63) => {
                    let v63: Rc<str> = v63.clone();
                    match &v62 {
                        US0::US0_Markup(v64) => {
                            let v64: Rc<str> = v64.clone();
                            let v65: Rc<str> = Rc::<str>::from(format!("{}{}", v63, v64));
                            US0::US0_Markup(v65)
                        }
                    }
                }
            }
        }
        US1::US1_Paragraph(v14) => {
            let v14: Rc<UH1> = v14.clone();
            let v15: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<p>"); } LIT.with(|lit| lit.clone()) };
            let v16: US0 = trusted_4(v15);
            let v17: US0 = inlines_html_11(v14);
            let v23: US0 = match &v16 {
                US0::US0_Markup(v18) => {
                    let v18: Rc<str> = v18.clone();
                    match &v17 {
                        US0::US0_Markup(v19) => {
                            let v19: Rc<str> = v19.clone();
                            let v20: Rc<str> = Rc::<str>::from(format!("{}{}", v18, v19));
                            US0::US0_Markup(v20)
                        }
                    }
                }
            };
            let v24: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("</p>\n"); } LIT.with(|lit| lit.clone()) };
            let v25: US0 = trusted_4(v24);
            match &v23 {
                US0::US0_Markup(v26) => {
                    let v26: Rc<str> = v26.clone();
                    match &v25 {
                        US0::US0_Markup(v27) => {
                            let v27: Rc<str> = v27.clone();
                            let v28: Rc<str> = Rc::<str>::from(format!("{}{}", v26, v27));
                            US0::US0_Markup(v28)
                        }
                    }
                }
            }
        }
        US1::US1_Rule => {
            let v87: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("<hr>\n"); } LIT.with(|lit| lit.clone()) };
            trusted_4(v87)
        }
        US1::US1_Table(v89, v90, v91) => {
            let v89: Rc<UH5> = v89.clone();
            let v90: Rc<UH3> = v90.clone();
            let v91: Rc<UH6> = v91.clone();
            table_html_6(v89, v90, v91)
        }
    }
}
fn blocks_html_3(mut v0: Rc<UH0>) -> US0 {
    match &*v0 {
        UH0::UH0_Cons(v1, v2) => {
            let v1: US1 = v1.clone();
            let v2: Rc<UH0> = v2.clone();
            let v3: US0 = block_html_5(v1);
            let v4: US0 = blocks_html_3(v2);
            match &v3 {
                US0::US0_Markup(v5) => {
                    let v5: Rc<str> = v5.clone();
                    match &v4 {
                        US0::US0_Markup(v6) => {
                            let v6: Rc<str> = v6.clone();
                            let v7: Rc<str> = Rc::<str>::from(format!("{}{}", v5, v6));
                            US0::US0_Markup(v7)
                        }
                    }
                }
            }
        }
        UH0::UH0_Nil => {
            let v11: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(""); } LIT.with(|lit| lit.clone()) };
            trusted_4(v11)
        }
    }
}
fn method32(mut v0: Rc<UH0>, mut v1: Rc<UH0>) -> Rc<UH0> {
    loop {
        match &*v0 {
            UH0::UH0_Cons(v2, v3) => {
                let v2: US1 = v2.clone();
                let v3: Rc<UH0> = v3.clone();
                let v4: Rc<UH0> = Rc::new(UH0::UH0_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH0::UH0_Nil => {
                return v1.clone();
            }
        }
    }
}
fn starts_with_34(mut v0: Rc<str>, mut v1: Rc<str>) -> bool {
    let v2: i32 = (v1.clone().len() as i32);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v2 <= v3;
    if v4 {
        let v5: i32 = 0i32;
        let v6: Rc<str> = run_of_17(v0, v5, v2);
        let v7: bool = &*v6 == &*v1;
        v7
    } else {
        false
    }
}
fn method36(mut v0: Rc<UH3>, mut v1: Rc<UH3>) -> Rc<UH3> {
    loop {
        match &*v0 {
            UH3::UH3_Cons(v2, v3) => {
                let v2: Rc<UH1> = v2.clone();
                let v3: Rc<UH3> = v3.clone();
                let v4: Rc<UH3> = Rc::new(UH3::UH3_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH3::UH3_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method37(mut v0: Rc<UH4>, mut v1: Rc<UH4>) -> Rc<UH4> {
    loop {
        match &*v0 {
            UH4::UH4_Cons(v2, v3) => {
                let v2: Rc<str> = v2.clone();
                let v3: Rc<UH4> = v3.clone();
                let v4: Rc<UH4> = Rc::new(UH4::UH4_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH4::UH4_Nil => {
                return v1.clone();
            }
        }
    }
}
fn method38(mut v0: Rc<UH6>, mut v1: Rc<UH6>) -> Rc<UH6> {
    loop {
        match &*v0 {
            UH6::UH6_Cons(v2, v3) => {
                let v2: Rc<UH3> = v2.clone();
                let v3: Rc<UH6> = v3.clone();
                let v4: Rc<UH6> = Rc::new(UH6::UH6_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH6::UH6_Nil => {
                return v1.clone();
            }
        }
    }
}
fn close_35(mut v0: US4, mut v1: Rc<UH0>) -> Rc<UH0> {
    match &v0 {
        US4::US4_Idle => {
            v1.clone()
        }
        US4::US4_InBullets(v5, v6) => {
            let v5: Rc<UH3> = v5.clone();
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH3> = Rc::new(UH3::UH3_Cons(v6, v7));
            let v9: Rc<UH3> = method36(v5, v8);
            let v10: US1 = US1::US1_Bullets(v9);
            Rc::new(UH0::UH0_Cons(v10, v1.clone()))
        }
        US4::US4_InCode(v20) => {
            let v20: Rc<UH4> = v20.clone();
            let v21: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
            let v22: Rc<UH4> = method37(v20, v21);
            let v23: US1 = US1::US1_CodeBlock(v22);
            Rc::new(UH0::UH0_Cons(v23, v1.clone()))
        }
        US4::US4_InNumbered(v12, v13, v14) => {
            let v12: i64 = *v12;
            let v13: Rc<UH3> = v13.clone();
            let v14: Rc<UH1> = v14.clone();
            let v15: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            let v16: Rc<UH3> = Rc::new(UH3::UH3_Cons(v14, v15));
            let v17: Rc<UH3> = method36(v13, v16);
            let v18: US1 = US1::US1_Numbered(v12, v17);
            Rc::new(UH0::UH0_Cons(v18, v1.clone()))
        }
        US4::US4_InParagraph(v2) => {
            let v2: Rc<UH1> = v2.clone();
            let v3: US1 = US1::US1_Paragraph(v2);
            Rc::new(UH0::UH0_Cons(v3, v1.clone()))
        }
        US4::US4_InTable(v25, v26, v27) => {
            let v25: Rc<UH5> = v25.clone();
            let v26: Rc<UH3> = v26.clone();
            let v27: Rc<UH6> = v27.clone();
            let v28: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
            let v29: Rc<UH6> = method38(v27, v28);
            let v30: US1 = US1::US1_Table(v25, v26, v29);
            Rc::new(UH0::UH0_Cons(v30, v1))
        }
    }
}
fn spaces_from_40(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn hashes_from_42(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn heading_of_41(mut v0: Rc<str>) -> US6 {
    let v1: i32 = 0i32;
    let v2: i32 = hashes_from_42(v0.clone(), v1);
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
        let v8: bool = v2 == 1i32;
        let v32: US7 = if v8 {
            let v9: US2 = US2::US2_H1;
            US7::US7_Some(v9)
        } else {
            let v11: bool = v2 == 2i32;
            if v11 {
                let v12: US2 = US2::US2_H2;
                US7::US7_Some(v12)
            } else {
                let v14: bool = v2 == 3i32;
                if v14 {
                    let v15: US2 = US2::US2_H3;
                    US7::US7_Some(v15)
                } else {
                    let v17: bool = v2 == 4i32;
                    if v17 {
                        let v18: US2 = US2::US2_H4;
                        US7::US7_Some(v18)
                    } else {
                        let v20: bool = v2 == 5i32;
                        if v20 {
                            let v21: US2 = US2::US2_H5;
                            US7::US7_Some(v21)
                        } else {
                            let v23: bool = v2 == 6i32;
                            if v23 {
                                let v24: US2 = US2::US2_H6;
                                US7::US7_Some(v24)
                            } else {
                                US7::US7_None
                            }
                        }
                    }
                }
            }
        };
        match &v32 {
            US7::US7_None => {
                US6::US6_None
            }
            US7::US7_Some(v33) => {
                let v33: US2 = v33.clone();
                let v34: i32 = v2.wrapping_add(1i32);
                let v35: Rc<str> = run_of_17(v0, v34, v3);
                US6::US6_Some(v33, v35)
            }
        }
    } else {
        US6::US6_None
    }
}
fn at_is_44(mut v0: Rc<str>, mut v1: i32, mut v2: u8) -> bool {
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
fn bullet_of_43(mut v0: Rc<str>, mut v1: i32) -> US8 {
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
        at_is_44(v0.clone(), v9, v10)
    } else {
        false
    };
    if v12 {
        let v13: i32 = v1.wrapping_add(2i32);
        let v14: Rc<str> = run_of_17(v0, v13, v2);
        US8::US8_Some(v14)
    } else {
        US8::US8_None
    }
}
fn digit_value_47(mut v0: u8) -> i64 {
    let v1: bool = v0 == b'0';
    if v1 {
        0i64
    } else {
        let v2: bool = v0 == b'1';
        if v2 {
            1i64
        } else {
            let v3: bool = v0 == b'2';
            if v3 {
                2i64
            } else {
                let v4: bool = v0 == b'3';
                if v4 {
                    3i64
                } else {
                    let v5: bool = v0 == b'4';
                    if v5 {
                        4i64
                    } else {
                        let v6: bool = v0 == b'5';
                        if v6 {
                            5i64
                        } else {
                            let v7: bool = v0 == b'6';
                            if v7 {
                                6i64
                            } else {
                                let v8: bool = v0 == b'7';
                                if v8 {
                                    7i64
                                } else {
                                    let v9: bool = v0 == b'8';
                                    if v9 {
                                        8i64
                                    } else {
                                        let v10: bool = v0 == b'9';
                                        if v10 {
                                            9i64
                                        } else {
                                            -1i64
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
}
fn digits_from_46(mut v0: Rc<str>, mut v1: i32, mut v2: i64, mut v3: i32) -> (i64, i32) {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 < v4;
        let v8: i64 = if v5 {
            let v6: u8 = v0.clone().as_bytes()[v1 as usize];
            digit_value_47(v6)
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
            (v0, v1, v2, v3) = (v0.clone(), v12, v14, v15);
            continue;
        } else {
            return (v2, v1);
        }
    }
}
fn number_of_45(mut v0: Rc<str>, mut v1: i32) -> US9 {
    let v2: i64 = 0i64;
    let v3: i32 = 0i32;
    let (mut v4, mut v5): (i64, i32) = digits_from_46(v0.clone(), v1, v2, v3);
    let v6: bool = v5 > v1;
    let v9: bool = if v6 {
        let v7: u8 = b'.';
        at_is_44(v0.clone(), v5, v7)
    } else {
        false
    };
    let v13: bool = if v9 {
        let v10: i32 = v5.wrapping_add(1i32);
        let v11: u8 = b' ';
        at_is_44(v0.clone(), v10, v11)
    } else {
        false
    };
    if v13 {
        let v14: i32 = v5.wrapping_add(2i32);
        let v15: i32 = (v0.clone().len() as i32);
        let v16: Rc<str> = run_of_17(v0, v14, v15);
        US9::US9_Some(v4, v16)
    } else {
        US9::US9_None
    }
}
fn classify_39(mut v0: Rc<str>) -> US5 {
    let v1: i32 = 0i32;
    let v2: i32 = spaces_from_40(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: bool = v2 == v3;
    if v4 {
        US5::US5_Blank
    } else {
        let v6: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
        let v7: bool = starts_with_34(v0.clone(), v6);
        if v7 {
            US5::US5_Fence
        } else {
            let v9: bool = &*v0 == "---";
            if v9 {
                US5::US5_RuleLine
            } else {
                let v11: u8 = v0.clone().as_bytes()[v2 as usize];
                let v12: bool = v11 == b'|';
                if v12 {
                    US5::US5_RowLine(v0.clone())
                } else {
                    let v14: US6 = heading_of_41(v0.clone());
                    match &v14 {
                        US6::US6_None => {
                            let v18: US8 = bullet_of_43(v0.clone(), v2);
                            match &v18 {
                                US8::US8_None => {
                                    let v21: US9 = number_of_45(v0.clone(), v2);
                                    match &v21 {
                                        US9::US9_None => {
                                            US5::US5_PlainLine(v0)
                                        }
                                        US9::US9_Some(v22, v23) => {
                                            let v22: i64 = *v22;
                                            let v23: Rc<str> = v23.clone();
                                            US5::US5_NumberLine(v22, v23)
                                        }
                                    }
                                }
                                US8::US8_Some(v19) => {
                                    let v19: Rc<str> = v19.clone();
                                    US5::US5_BulletLine(v19)
                                }
                            }
                        }
                        US6::US6_Some(v15, v16) => {
                            let v15: US2 = v15.clone();
                            let v16: Rc<str> = v16.clone();
                            US5::US5_HeadingLine(v15, v16)
                        }
                    }
                }
            }
        }
    }
}
fn cell_end_50(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
                (v0, v1) = (v0.clone(), v9);
                continue;
            } else {
                let v11: bool = v4 == b'|';
                if v11 {
                    return v1;
                } else {
                    let v12: i32 = v1.wrapping_add(1i32);
                    (v0, v1) = (v0.clone(), v12);
                    continue;
                }
            }
        }
    }
}
fn trail_from_52(mut v0: Rc<str>, mut v1: i32) -> i32 {
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
            (v0, v1) = (v0.clone(), v7);
            continue;
        } else {
            return v1;
        }
    }
}
fn trim_51(mut v0: Rc<str>) -> Rc<str> {
    let v1: i32 = 0i32;
    let v2: i32 = spaces_from_40(v0.clone(), v1);
    let v3: i32 = (v0.clone().len() as i32);
    let v4: i32 = trail_from_52(v0.clone(), v3);
    run_of_17(v0, v2, v4)
}
fn cells_from_49(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH4>) -> Rc<UH4> {
    loop {
        let v3: i32 = (v0.clone().len() as i32);
        let v4: bool = v1 >= v3;
        if v4 {
            let v5: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
            return method37(v2.clone(), v5.clone());
        } else {
            let v7: i32 = cell_end_50(v0.clone(), v1);
            let v8: Rc<str> = run_of_17(v0.clone(), v1, v7);
            let v9: Rc<str> = trim_51(v8.clone());
            let v10: bool = v7 >= v3;
            let v12: bool = if v10 {
                let v11: bool = &*v9 == "";
                v11
            } else {
                false
            };
            if v12 {
                let v13: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
                return method37(v2.clone(), v13.clone());
            } else {
                let v15: i32 = v7.wrapping_add(1i32);
                let v16: Rc<UH4> = Rc::new(UH4::UH4_Cons(v9.clone(), v2.clone()));
                (v0, v1, v2) = (v0.clone(), v15, v16.clone());
                continue;
            }
        }
    }
}
fn row_cells_48(mut v0: Rc<str>) -> Rc<UH4> {
    let v1: Rc<str> = trim_51(v0);
    let v2: i32 = 1i32;
    let v3: i32 = (v1.clone().len() as i32);
    let v4: Rc<str> = run_of_17(v1, v2, v3);
    let v5: i32 = 0i32;
    let v6: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    cells_from_49(v4, v5, v6)
}
fn rev_inlines_57(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    loop {
        match &*v0 {
            UH1::UH1_Cons(v2, v3) => {
                let v2: Rc<UH2> = v2.clone();
                let v3: Rc<UH1> = v3.clone();
                let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v2.clone(), v1.clone()));
                (v0, v1) = (v3.clone(), v4.clone());
                continue;
            }
            UH1::UH1_Nil => {
                return v1.clone();
            }
        }
    }
}
fn find_char_61(mut v0: Rc<str>, mut v1: u8, mut v2: i32) -> i32 {
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
                (v0, v1, v2) = (v0.clone(), v1, v7);
                continue;
            }
        }
    }
}
fn code_close_60(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: u8 = b'`';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
    if v3 {
        let v4: u8 = b'`';
        let v5: i32 = v1.wrapping_add(1i32);
        find_char_61(v0, v4, v5)
    } else {
        -1i32
    }
}
fn escapable_63(mut v0: u8) -> bool {
    let v1: bool = v0 == b'*';
    let v3: bool = if v1 {
        true
    } else {
        let v2: bool = v0 == b'_';
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
fn escaped_at_62(mut v0: Rc<str>, mut v1: i32) -> bool {
    let v2: u8 = b'\\';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
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
        escapable_63(v9)
    } else {
        false
    }
}
fn double_star_65(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = v1.wrapping_add(1i32);
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
                (v0, v1) = (v0.clone(), v2);
                continue;
            }
        }
    }
}
fn strong_close_64(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: u8 = b'*';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
    let v7: bool = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: u8 = b'*';
        at_is_44(v0.clone(), v4, v5)
    } else {
        false
    };
    if v7 {
        let v8: i32 = v1.wrapping_add(2i32);
        double_star_65(v0, v8)
    } else {
        -1i32
    }
}
fn auto_close_66(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: u8 = b'<';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
    let v7: i32 = if v3 {
        let v4: u8 = b'>';
        let v5: i32 = v1.wrapping_add(1i32);
        find_char_61(v0.clone(), v4, v5)
    } else {
        -1i32
    };
    let v8: i32 = v1.wrapping_add(5i32);
    let v9: bool = v7 > v8;
    let v13: bool = if v9 {
        let v10: i32 = v1.wrapping_add(1i32);
        let v11: Rc<str> = run_of_17(v0, v10, v8);
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
fn paren_close_68(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: bool = v1 < 0i32;
    if v2 {
        -1i32
    } else {
        let v3: i32 = v1.wrapping_add(1i32);
        let v4: u8 = b'(';
        let v5: bool = at_is_44(v0.clone(), v3, v4);
        if v5 {
            let v6: u8 = b')';
            let v7: i32 = v1.wrapping_add(2i32);
            find_char_61(v0, v6, v7)
        } else {
            -1i32
        }
    }
}
fn image_close_67(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: u8 = b'!';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
    let v7: bool = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        let v5: u8 = b'[';
        at_is_44(v0.clone(), v4, v5)
    } else {
        false
    };
    if v7 {
        let v8: u8 = b']';
        let v9: i32 = v1.wrapping_add(2i32);
        let v10: i32 = find_char_61(v0.clone(), v8, v9);
        paren_close_68(v0, v10)
    } else {
        -1i32
    }
}
fn image_at_69(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<UH2> {
    let v3: u8 = b']';
    let v4: i32 = v1.wrapping_add(2i32);
    let v5: i32 = find_char_61(v0.clone(), v3, v4);
    let v6: Rc<str> = run_of_17(v0.clone(), v4, v5);
    let v7: i32 = v5.wrapping_add(2i32);
    let v8: Rc<str> = run_of_17(v0, v7, v2);
    Rc::new(UH2::UH2_Image(v6, v8))
}
fn badge_close_70(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: u8 = b'[';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
    let v6: i32 = if v3 {
        let v4: i32 = v1.wrapping_add(1i32);
        image_close_67(v0.clone(), v4)
    } else {
        -1i32
    };
    let v7: bool = v6 >= 0i32;
    let v11: bool = if v7 {
        let v8: i32 = v6.wrapping_add(1i32);
        let v9: u8 = b']';
        at_is_44(v0.clone(), v8, v9)
    } else {
        false
    };
    if v11 {
        let v12: i32 = v6.wrapping_add(1i32);
        paren_close_68(v0, v12)
    } else {
        -1i32
    }
}
fn linked_image_at_71(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<UH2> {
    let v3: i32 = v1.wrapping_add(1i32);
    let v4: i32 = image_close_67(v0.clone(), v3);
    let v5: u8 = b']';
    let v6: i32 = v1.wrapping_add(3i32);
    let v7: i32 = find_char_61(v0.clone(), v5, v6);
    let v8: Rc<str> = run_of_17(v0.clone(), v6, v7);
    let v9: i32 = v7.wrapping_add(2i32);
    let v10: Rc<str> = run_of_17(v0.clone(), v9, v4);
    let v11: i32 = v4.wrapping_add(3i32);
    let v12: Rc<str> = run_of_17(v0, v11, v2);
    Rc::new(UH2::UH2_LinkedImage(v8, v10, v12))
}
fn link_close_72(mut v0: Rc<str>, mut v1: i32) -> i32 {
    let v2: u8 = b'[';
    let v3: bool = at_is_44(v0.clone(), v1, v2);
    if v3 {
        let v4: u8 = b']';
        let v5: i32 = v1.wrapping_add(1i32);
        let v6: i32 = find_char_61(v0.clone(), v4, v5);
        paren_close_68(v0, v6)
    } else {
        -1i32
    }
}
fn link_at_73(mut v0: Rc<str>, mut v1: i32, mut v2: i32) -> Rc<UH2> {
    let v3: u8 = b']';
    let v4: i32 = v1.wrapping_add(1i32);
    let v5: i32 = find_char_61(v0.clone(), v3, v4);
    let v6: Rc<str> = run_of_17(v0.clone(), v4, v5);
    let v7: i32 = v5.wrapping_add(2i32);
    let v8: Rc<str> = run_of_17(v0, v7, v2);
    Rc::new(UH2::UH2_Link(v6, v8))
}
fn marker_75(mut v0: u8) -> bool {
    let v1: bool = v0 == b'`';
    let v3: bool = if v1 {
        true
    } else {
        let v2: bool = v0 == b'[';
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
fn next_marker_74(mut v0: Rc<str>, mut v1: i32) -> i32 {
    loop {
        let v2: i32 = (v0.clone().len() as i32);
        let v3: bool = v1 >= v2;
        if v3 {
            return v1;
        } else {
            let v4: u8 = v0.clone().as_bytes()[v1 as usize];
            let v5: bool = marker_75(v4);
            if v5 {
                return v1;
            } else {
                let v6: i32 = v1.wrapping_add(1i32);
                (v0, v1) = (v0.clone(), v6);
                continue;
            }
        }
    }
}
enum Group56 {
    M56(Rc<str>, i32, Rc<UH1>),
    M58(Rc<str>, i32, i32, Rc<UH1>),
    M59(Rc<str>, i32, Rc<UH1>),
}
fn group_56(mut st: Group56) -> Rc<UH1> {
    loop {
        match st {
            Group56::M56(mut v0, mut v1, mut v2) => {
                let v3: i32 = (v0.clone().len() as i32);
                let v4: bool = v1 >= v3;
                if v4 {
                    let v5: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                    return rev_inlines_57(v2.clone(), v5.clone());
                } else {
                    let v7: i32 = code_close_60(v0.clone(), v1);
                    let v8: bool = v7 >= 0i32;
                    if v8 {
                        let v9: i32 = code_close_60(v0.clone(), v1);
                        let v10: i32 = v9.wrapping_add(1i32);
                        let v11: i32 = v1.wrapping_add(1i32);
                        let v12: i32 = code_close_60(v0.clone(), v1);
                        let v13: Rc<str> = run_of_17(v0.clone(), v11, v12);
                        let v14: Rc<UH2> = Rc::new(UH2::UH2_Code(v13.clone()));
                        let v15: Rc<UH1> = Rc::new(UH1::UH1_Cons(v14.clone(), v2.clone()));
                        st = Group56::M56(v0.clone(), v10, v15.clone());
                        continue;
                    } else {
                        let v17: bool = escaped_at_62(v0.clone(), v1);
                        if v17 {
                            let v18: i32 = v1.wrapping_add(2i32);
                            let v19: i32 = v1.wrapping_add(1i32);
                            let v20: Rc<str> = run_of_17(v0.clone(), v19, v18);
                            let v21: Rc<UH2> = Rc::new(UH2::UH2_Text(v20.clone()));
                            let v22: Rc<UH1> = Rc::new(UH1::UH1_Cons(v21.clone(), v2.clone()));
                            st = Group56::M56(v0.clone(), v18, v22.clone());
                            continue;
                        } else {
                            let v24: i32 = strong_close_64(v0.clone(), v1);
                            let v25: bool = v24 >= 0i32;
                            if v25 {
                                let v26: i32 = strong_close_64(v0.clone(), v1);
                                st = Group56::M58(v0.clone(), v1, v26, v2.clone());
                                continue;
                            } else {
                                let v28: i32 = auto_close_66(v0.clone(), v1);
                                let v29: bool = v28 >= 0i32;
                                if v29 {
                                    let v30: i32 = auto_close_66(v0.clone(), v1);
                                    let v31: i32 = v30.wrapping_add(1i32);
                                    let v32: i32 = v1.wrapping_add(1i32);
                                    let v33: i32 = auto_close_66(v0.clone(), v1);
                                    let v34: Rc<str> = run_of_17(v0.clone(), v32, v33);
                                    let v35: Rc<UH2> = Rc::new(UH2::UH2_AutoLink(v34.clone()));
                                    let v36: Rc<UH1> = Rc::new(UH1::UH1_Cons(v35.clone(), v2.clone()));
                                    st = Group56::M56(v0.clone(), v31, v36.clone());
                                    continue;
                                } else {
                                    let v38: i32 = image_close_67(v0.clone(), v1);
                                    let v39: bool = v38 >= 0i32;
                                    if v39 {
                                        let v40: i32 = image_close_67(v0.clone(), v1);
                                        let v41: i32 = v40.wrapping_add(1i32);
                                        let v42: i32 = image_close_67(v0.clone(), v1);
                                        let v43: Rc<UH2> = image_at_69(v0.clone(), v1, v42);
                                        let v44: Rc<UH1> = Rc::new(UH1::UH1_Cons(v43.clone(), v2.clone()));
                                        st = Group56::M56(v0.clone(), v41, v44.clone());
                                        continue;
                                    } else {
                                        let v46: i32 = badge_close_70(v0.clone(), v1);
                                        let v47: bool = v46 >= 0i32;
                                        if v47 {
                                            let v48: i32 = badge_close_70(v0.clone(), v1);
                                            let v49: i32 = v48.wrapping_add(1i32);
                                            let v50: i32 = badge_close_70(v0.clone(), v1);
                                            let v51: Rc<UH2> = linked_image_at_71(v0.clone(), v1, v50);
                                            let v52: Rc<UH1> = Rc::new(UH1::UH1_Cons(v51.clone(), v2.clone()));
                                            st = Group56::M56(v0.clone(), v49, v52.clone());
                                            continue;
                                        } else {
                                            let v54: i32 = link_close_72(v0.clone(), v1);
                                            let v55: bool = v54 >= 0i32;
                                            if v55 {
                                                let v56: i32 = link_close_72(v0.clone(), v1);
                                                let v57: i32 = v56.wrapping_add(1i32);
                                                let v58: i32 = link_close_72(v0.clone(), v1);
                                                let v59: Rc<UH2> = link_at_73(v0.clone(), v1, v58);
                                                let v60: Rc<UH1> = Rc::new(UH1::UH1_Cons(v59.clone(), v2.clone()));
                                                st = Group56::M56(v0.clone(), v57, v60.clone());
                                                continue;
                                            } else {
                                                st = Group56::M59(v0.clone(), v1, v2.clone());
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Group56::M58(mut v0, mut v1, mut v2, mut v3) => {
                let v4: i32 = v2.wrapping_add(2i32);
                let v5: i32 = v1.wrapping_add(2i32);
                let v6: Rc<str> = run_of_17(v0.clone(), v5, v2);
                let v7: i32 = 0i32;
                let v8: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
                let v9: Rc<UH1> = inlines_from_56(v6.clone(), v7, v8.clone());
                let v10: Rc<UH2> = Rc::new(UH2::UH2_Strong(v9.clone()));
                let v11: Rc<UH1> = Rc::new(UH1::UH1_Cons(v10.clone(), v3.clone()));
                st = Group56::M56(v0.clone(), v4, v11.clone());
                continue;
            }
            Group56::M59(mut v0, mut v1, mut v2) => {
                let v3: i32 = v1.wrapping_add(1i32);
                let v4: i32 = next_marker_74(v0.clone(), v3);
                let v5: Rc<str> = run_of_17(v0.clone(), v1, v4);
                let v6: Rc<UH2> = Rc::new(UH2::UH2_Text(v5.clone()));
                let v7: Rc<UH1> = Rc::new(UH1::UH1_Cons(v6.clone(), v2.clone()));
                st = Group56::M56(v0.clone(), v4, v7.clone());
                continue;
            }
        }
    }
}
fn plain_at_59(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    group_56(Group56::M59(v0, v1, v2))
}
fn strong_at_58(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH1>) -> Rc<UH1> {
    group_56(Group56::M58(v0, v1, v2, v3))
}
fn inlines_from_56(mut v0: Rc<str>, mut v1: i32, mut v2: Rc<UH1>) -> Rc<UH1> {
    group_56(Group56::M56(v0, v1, v2))
}
fn parse_inline_55(mut v0: Rc<str>) -> Rc<UH1> {
    let v1: i32 = 0i32;
    let v2: Rc<UH1> = { thread_local!{ static CASE: Rc<UH1> = Rc::new(UH1::UH1_Nil); } CASE.with(|case| case.clone()) };
    inlines_from_56(v0, v1, v2)
}
fn method54(mut v0: Rc<UH4>, mut v1: Rc<UH3>) -> Rc<UH3> {
    match &*v0 {
        UH4::UH4_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH4> = v3.clone();
            let v4: Rc<UH3> = method54(v3, v1.clone());
            let v5: Rc<UH1> = parse_inline_55(v2);
            Rc::new(UH3::UH3_Cons(v5, v4))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn inline_cells_53(mut v0: Rc<UH4>) -> Rc<UH3> {
    let v1: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
    method54(v0, v1)
}
fn bulleted_76(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<UH0>) -> (US4, Rc<UH0>) {
    match &v0 {
        US4::US4_InBullets(v3, v4) => {
            let v3: Rc<UH3> = v3.clone();
            let v4: Rc<UH1> = v4.clone();
            let v5: Rc<UH1> = parse_inline_55(v1.clone());
            let v6: Rc<UH3> = Rc::new(UH3::UH3_Cons(v4, v3));
            let v7: US4 = US4::US4_InBullets(v6, v5);
            (v7, v2.clone())
        }
        _ => {
            let v8: Rc<UH1> = parse_inline_55(v1);
            let v9: Rc<UH0> = close_35(v0, v2);
            let v10: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            let v11: US4 = US4::US4_InBullets(v10, v8);
            (v11, v9)
        }
    }
}
fn heading_block_77(mut v0: US2, mut v1: Rc<str>) -> US1 {
    let v2: Rc<UH1> = parse_inline_55(v1);
    US1::US1_Heading(v0, v2)
}
fn numbered_78(mut v0: US4, mut v1: i64, mut v2: Rc<str>, mut v3: Rc<UH0>) -> (US4, Rc<UH0>) {
    match &v0 {
        US4::US4_InNumbered(v4, v5, v6) => {
            let v4: i64 = *v4;
            let v5: Rc<UH3> = v5.clone();
            let v6: Rc<UH1> = v6.clone();
            let v7: Rc<UH1> = parse_inline_55(v2.clone());
            let v8: Rc<UH3> = Rc::new(UH3::UH3_Cons(v6, v5));
            let v9: US4 = US4::US4_InNumbered(v4, v8, v7);
            (v9, v3.clone())
        }
        _ => {
            let v10: Rc<UH1> = parse_inline_55(v2);
            let v11: Rc<UH0> = close_35(v0, v3);
            let v12: Rc<UH3> = { thread_local!{ static CASE: Rc<UH3> = Rc::new(UH3::UH3_Nil); } CASE.with(|case| case.clone()) };
            let v13: US4 = US4::US4_InNumbered(v1, v12, v10);
            (v13, v11)
        }
    }
}
fn method81(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    match &*v0 {
        UH1::UH1_Cons(v2, v3) => {
            let v2: Rc<UH2> = v2.clone();
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<UH1> = method81(v3, v1.clone());
            Rc::new(UH1::UH1_Cons(v2, v4))
        }
        UH1::UH1_Nil => {
            v1
        }
    }
}
fn joined_80(mut v0: Rc<UH1>, mut v1: Rc<UH1>) -> Rc<UH1> {
    let v2: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from(" "); } LIT.with(|lit| lit.clone()) };
    let v3: Rc<UH2> = Rc::new(UH2::UH2_Text(v2));
    let v4: Rc<UH1> = Rc::new(UH1::UH1_Cons(v3, v1));
    method81(v0, v4)
}
fn continued_79(mut v0: US4, mut v1: Rc<str>, mut v2: Rc<UH0>) -> (US4, Rc<UH0>) {
    match &v0 {
        US4::US4_InBullets(v7, v8) => {
            let v7: Rc<UH3> = v7.clone();
            let v8: Rc<UH1> = v8.clone();
            let v9: Rc<str> = trim_51(v1.clone());
            let v10: Rc<UH1> = parse_inline_55(v9);
            let v11: Rc<UH1> = joined_80(v8, v10);
            let v12: US4 = US4::US4_InBullets(v7, v11);
            (v12, v2.clone())
        }
        US4::US4_InNumbered(v13, v14, v15) => {
            let v13: i64 = *v13;
            let v14: Rc<UH3> = v14.clone();
            let v15: Rc<UH1> = v15.clone();
            let v16: Rc<str> = trim_51(v1.clone());
            let v17: Rc<UH1> = parse_inline_55(v16);
            let v18: Rc<UH1> = joined_80(v15, v17);
            let v19: US4 = US4::US4_InNumbered(v13, v14, v18);
            (v19, v2.clone())
        }
        US4::US4_InParagraph(v3) => {
            let v3: Rc<UH1> = v3.clone();
            let v4: Rc<UH1> = parse_inline_55(v1.clone());
            let v5: Rc<UH1> = joined_80(v3, v4);
            let v6: US4 = US4::US4_InParagraph(v5);
            (v6, v2.clone())
        }
        _ => {
            let v20: Rc<UH1> = parse_inline_55(v1);
            let v21: Rc<UH0> = close_35(v0, v2);
            let v22: US4 = US4::US4_InParagraph(v20);
            (v22, v21)
        }
    }
}
fn delimiter_chars_83(mut v0: Rc<str>, mut v1: i32, mut v2: bool) -> bool {
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
                (v0, v1, v2) = (v0.clone(), v7, v8);
                continue;
            } else {
                let v10: bool = v5 == b'|';
                let v12: bool = if v10 {
                    true
                } else {
                    let v11: bool = v5 == b':';
                    v11
                };
                let v14: bool = if v12 {
                    true
                } else {
                    let v13: bool = v5 == b' ';
                    v13
                };
                if v14 {
                    let v15: i32 = v1.wrapping_add(1i32);
                    (v0, v1, v2) = (v0.clone(), v15, v2);
                    continue;
                } else {
                    return false;
                }
            }
        }
    }
}
fn align_of_85(mut v0: Rc<str>) -> US3 {
    let v1: i32 = (v0.clone().len() as i32);
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
        US3::US3_AlignCenter
    } else {
        if v9 {
            US3::US3_AlignRight
        } else {
            if v5 {
                US3::US3_AlignLeft
            } else {
                US3::US3_AlignNone
            }
        }
    }
}
fn method84(mut v0: Rc<UH4>, mut v1: Rc<UH5>) -> Rc<UH5> {
    match &*v0 {
        UH4::UH4_Cons(v2, v3) => {
            let v2: Rc<str> = v2.clone();
            let v3: Rc<UH4> = v3.clone();
            let v4: Rc<UH5> = method84(v3, v1.clone());
            let v5: US3 = align_of_85(v2);
            Rc::new(UH5::UH5_Cons(v5, v4))
        }
        UH4::UH4_Nil => {
            v1
        }
    }
}
fn delimiter_aligns_82(mut v0: Rc<UH4>) -> US10 {
    match &*v0 {
        UH4::UH4_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH4> = v2.clone();
            let v3: US5 = classify_39(v1);
            match &v3 {
                US5::US5_RowLine(v4) => {
                    let v4: Rc<str> = v4.clone();
                    let v5: i32 = 0i32;
                    let v6: bool = false;
                    let v7: bool = delimiter_chars_83(v4.clone(), v5, v6);
                    if v7 {
                        let v8: Rc<UH4> = row_cells_48(v4);
                        let v9: Rc<UH5> = { thread_local!{ static CASE: Rc<UH5> = Rc::new(UH5::UH5_Nil); } CASE.with(|case| case.clone()) };
                        let v10: Rc<UH5> = method84(v8, v9);
                        US10::US10_Some(v10)
                    } else {
                        US10::US10_None
                    }
                }
                _ => {
                    US10::US10_None
                }
            }
        }
        UH4::UH4_Nil => {
            US10::US10_None
        }
    }
}
fn after_first_86(mut v0: Rc<UH4>) -> Rc<UH4> {
    match &*v0 {
        UH4::UH4_Cons(v1, v2) => {
            let v1: Rc<str> = v1.clone();
            let v2: Rc<UH4> = v2.clone();
            v2
        }
        UH4::UH4_Nil => {
            { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) }
        }
    }
}
enum Group29 {
    M29(Rc<UH4>, US4, Rc<UH0>),
    M30(Rc<str>, Rc<UH4>, US4, Rc<UH0>),
    M31(Rc<UH4>, US4, Rc<UH0>),
    M33(Rc<str>, Rc<str>, Rc<UH4>, US4, Rc<UH0>),
}
fn group_29(mut st: Group29) -> Rc<UH0> {
    loop {
        match st {
            Group29::M29(mut v0, mut v1, mut v2) => {
                match &*v0 {
                    UH4::UH4_Cons(v6, v7) => {
                        let v6: Rc<str> = v6.clone();
                        let v7: Rc<UH4> = v7.clone();
                        match &v1 {
                            US4::US4_InCode(v8) => {
                                let v8: Rc<UH4> = v8.clone();
                                let v9: Rc<str> = { thread_local!{ static LIT: Rc<str> = Rc::<str>::from("```"); } LIT.with(|lit| lit.clone()) };
                                let v10: bool = starts_with_34(v6.clone(), v9.clone());
                                if v10 {
                                    let v11: US4 = US4::US4_Idle;
                                    let v12: Rc<UH0> = close_35(v1.clone(), v2.clone());
                                    st = Group29::M29(v7.clone(), v11.clone(), v12.clone());
                                    continue;
                                } else {
                                    let v14: Rc<UH4> = Rc::new(UH4::UH4_Cons(v6.clone(), v8.clone()));
                                    let v15: US4 = US4::US4_InCode(v14.clone());
                                    st = Group29::M29(v7.clone(), v15.clone(), v2.clone());
                                    continue;
                                }
                            }
                            US4::US4_InTable(v18, v19, v20) => {
                                let v18: Rc<UH5> = v18.clone();
                                let v19: Rc<UH3> = v19.clone();
                                let v20: Rc<UH6> = v20.clone();
                                let v21: US5 = classify_39(v6.clone());
                                match &v21 {
                                    US5::US5_RowLine(v22) => {
                                        let v22: Rc<str> = v22.clone();
                                        let v23: Rc<UH4> = row_cells_48(v22.clone());
                                        let v24: Rc<UH3> = inline_cells_53(v23.clone());
                                        let v25: Rc<UH6> = Rc::new(UH6::UH6_Cons(v24.clone(), v20.clone()));
                                        let v26: US4 = US4::US4_InTable(v18.clone(), v19.clone(), v25.clone());
                                        st = Group29::M29(v7.clone(), v26.clone(), v2.clone());
                                        continue;
                                    }
                                    _ => {
                                        let v28: US4 = US4::US4_Idle;
                                        let v29: Rc<UH0> = close_35(v1.clone(), v2.clone());
                                        st = Group29::M29(v0.clone(), v28.clone(), v29.clone());
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                st = Group29::M30(v6.clone(), v7.clone(), v1.clone(), v2.clone());
                                continue;
                            }
                        }
                    }
                    UH4::UH4_Nil => {
                        let v3: Rc<UH0> = close_35(v1.clone(), v2.clone());
                        let v4: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
                        return method32(v3.clone(), v4.clone());
                    }
                }
            }
            Group29::M30(mut v0, mut v1, mut v2, mut v3) => {
                let v4: US5 = classify_39(v0.clone());
                match &v4 {
                    US5::US5_Blank => {
                        let v9: US4 = US4::US4_Idle;
                        let v10: Rc<UH0> = close_35(v2.clone(), v3.clone());
                        st = Group29::M29(v1.clone(), v9.clone(), v10.clone());
                        continue;
                    }
                    US5::US5_BulletLine(v26) => {
                        let v26: Rc<str> = v26.clone();
                        let (mut v27, mut v28): (US4, Rc<UH0>) = bulleted_76(v2.clone(), v26.clone(), v3.clone());
                        st = Group29::M31(v1.clone(), v27.clone(), v28.clone());
                        continue;
                    }
                    US5::US5_Fence => {
                        let v5: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
                        let v6: US4 = US4::US4_InCode(v5.clone());
                        let v7: Rc<UH0> = close_35(v2.clone(), v3.clone());
                        st = Group29::M29(v1.clone(), v6.clone(), v7.clone());
                        continue;
                    }
                    US5::US5_HeadingLine(v12, v13) => {
                        let v12: US2 = v12.clone();
                        let v13: Rc<str> = v13.clone();
                        let v14: US4 = US4::US4_Idle;
                        let v15: US1 = heading_block_77(v12.clone(), v13.clone());
                        let v16: Rc<UH0> = close_35(v2.clone(), v3.clone());
                        let v17: Rc<UH0> = Rc::new(UH0::UH0_Cons(v15.clone(), v16.clone()));
                        st = Group29::M29(v1.clone(), v14.clone(), v17.clone());
                        continue;
                    }
                    US5::US5_NumberLine(v30, v31) => {
                        let v30: i64 = *v30;
                        let v31: Rc<str> = v31.clone();
                        let (mut v32, mut v33): (US4, Rc<UH0>) = numbered_78(v2.clone(), v30, v31.clone(), v3.clone());
                        st = Group29::M31(v1.clone(), v32.clone(), v33.clone());
                        continue;
                    }
                    US5::US5_PlainLine(v35) => {
                        let v35: Rc<str> = v35.clone();
                        let (mut v36, mut v37): (US4, Rc<UH0>) = continued_79(v2.clone(), v0.clone(), v3.clone());
                        st = Group29::M31(v1.clone(), v36.clone(), v37.clone());
                        continue;
                    }
                    US5::US5_RowLine(v24) => {
                        let v24: Rc<str> = v24.clone();
                        st = Group29::M33(v0.clone(), v24.clone(), v1.clone(), v2.clone(), v3.clone());
                        continue;
                    }
                    US5::US5_RuleLine => {
                        let v19: US4 = US4::US4_Idle;
                        let v20: Rc<UH0> = close_35(v2.clone(), v3.clone());
                        let v21: US1 = US1::US1_Rule;
                        let v22: Rc<UH0> = Rc::new(UH0::UH0_Cons(v21.clone(), v20.clone()));
                        st = Group29::M29(v1.clone(), v19.clone(), v22.clone());
                        continue;
                    }
                }
            }
            Group29::M31(mut v0, mut v1, mut v2) => {
                st = Group29::M29(v0.clone(), v1.clone(), v2.clone());
                continue;
            }
            Group29::M33(mut v0, mut v1, mut v2, mut v3, mut v4) => {
                let v5: US10 = delimiter_aligns_82(v2.clone());
                match &v5 {
                    US10::US10_None => {
                        let (mut v14, mut v15): (US4, Rc<UH0>) = continued_79(v3.clone(), v0.clone(), v4.clone());
                        st = Group29::M31(v2.clone(), v14.clone(), v15.clone());
                        continue;
                    }
                    US10::US10_Some(v6) => {
                        let v6: Rc<UH5> = v6.clone();
                        let v7: Rc<UH4> = after_first_86(v2.clone());
                        let v8: Rc<UH4> = row_cells_48(v1.clone());
                        let v9: Rc<UH3> = inline_cells_53(v8.clone());
                        let v10: Rc<UH6> = { thread_local!{ static CASE: Rc<UH6> = Rc::new(UH6::UH6_Nil); } CASE.with(|case| case.clone()) };
                        let v11: US4 = US4::US4_InTable(v6.clone(), v9.clone(), v10.clone());
                        let v12: Rc<UH0> = close_35(v3.clone(), v4.clone());
                        st = Group29::M29(v7.clone(), v11.clone(), v12.clone());
                        continue;
                    }
                }
            }
        }
    }
}
fn row_step_33(mut v0: Rc<str>, mut v1: Rc<str>, mut v2: Rc<UH4>, mut v3: US4, mut v4: Rc<UH0>) -> Rc<UH0> {
    group_29(Group29::M33(v0, v1, v2, v3, v4))
}
fn resume_31(mut v0: Rc<UH4>, mut v1: US4, mut v2: Rc<UH0>) -> Rc<UH0> {
    group_29(Group29::M31(v0, v1, v2))
}
fn line_step_30(mut v0: Rc<str>, mut v1: Rc<UH4>, mut v2: US4, mut v3: Rc<UH0>) -> Rc<UH0> {
    group_29(Group29::M30(v0, v1, v2, v3))
}
fn blocks_from_29(mut v0: Rc<UH4>, mut v1: US4, mut v2: Rc<UH0>) -> Rc<UH0> {
    group_29(Group29::M29(v0, v1, v2))
}
fn lines_from_87(mut v0: Rc<str>, mut v1: i32, mut v2: i32, mut v3: Rc<UH4>) -> Rc<UH4> {
    loop {
        let v4: i32 = (v0.clone().len() as i32);
        let v5: bool = v1 >= v4;
        if v5 {
            let v6: Rc<str> = run_of_17(v0.clone(), v2, v1);
            let v7: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
            let v8: Rc<UH4> = Rc::new(UH4::UH4_Cons(v6.clone(), v7.clone()));
            return method37(v3.clone(), v8.clone());
        } else {
            let v10: u8 = v0.clone().as_bytes()[v1 as usize];
            let v11: bool = v10 == b'\n';
            if v11 {
                let v12: i32 = v1.wrapping_add(1i32);
                let v13: Rc<str> = run_of_17(v0.clone(), v2, v1);
                let v14: Rc<UH4> = Rc::new(UH4::UH4_Cons(v13.clone(), v3.clone()));
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
fn parse_markdown_28(mut v0: Rc<str>) -> Rc<UH0> {
    let v1: i32 = 0i32;
    let v2: i32 = 0i32;
    let v3: Rc<UH4> = { thread_local!{ static CASE: Rc<UH4> = Rc::new(UH4::UH4_Nil); } CASE.with(|case| case.clone()) };
    let v4: Rc<UH4> = lines_from_87(v0, v1, v2, v3);
    let v5: US4 = US4::US4_Idle;
    let v6: Rc<UH0> = { thread_local!{ static CASE: Rc<UH0> = Rc::new(UH0::UH0_Nil); } CASE.with(|case| case.clone()) };
    blocks_from_29(v4, v5, v6)
}
fn markdown_html_2(mut v0: Rc<str>) -> US0 {
    let v1: Rc<UH0> = parse_markdown_28(v0);
    blocks_html_3(v1)
}
fn md_0(mut v0: Rc<str>) -> Rc<str> {
    let v1: US0 = markdown_html_2(v0);
    markup_text_1(v1)
}
fn closure0() -> Rc<dyn Fn(Rc<str>) -> Rc<str>> {
    thread_local!{ static CLOSURE: Rc<dyn Fn(Rc<str>) -> Rc<str>> = Rc::new(move |mut v0: Rc<str>| -> Rc<str> {
        md_0(v0.clone())
    }); } CLOSURE.with(|closure| closure.clone())
}
pub fn html_markdown(v0: &str) -> Rc<str> {
    closure0()(Rc::<str>::from(v0))
}
