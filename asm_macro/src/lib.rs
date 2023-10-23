use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parenthesized,
    parse_macro_input,
    punctuated::Punctuated,
    token::{self, Comma, Paren},
    BinOp, Expr, ExprPath, Ident, Lit, LitInt, LitStr, Path, Token, UnOp,
};

enum HackLabel {
    Ident(Path),
    Str(String),
    Int(syn::LitInt),
}

impl From<HackLabel> for proc_macro2::TokenStream {
    fn from(value: HackLabel) -> Self {
        match value {
            HackLabel::Str(s) => s.into_token_stream(), 
            HackLabel::Ident(n) => n.into_token_stream(),//quote!(std::borrow::Cow::Borrowed(stringify!(#n))),
            HackLabel::Int(i) => quote!(crate::asm::Instruction::from(#i)),
        }
    }
}

mod kw {
    syn::custom_keyword!(A);
    syn::custom_keyword!(M);
    syn::custom_keyword!(D);
    syn::custom_keyword!(AD);
    syn::custom_keyword!(MD);
    syn::custom_keyword!(AM);
    syn::custom_keyword!(AMD);
    syn::custom_keyword!(JNE);
    syn::custom_keyword!(JGT);
    syn::custom_keyword!(JLT);
    syn::custom_keyword!(JGE);
    syn::custom_keyword!(JLE);
    syn::custom_keyword!(JMP);
    syn::custom_keyword!(JEQ);
    syn::custom_keyword!(SP);
    syn::custom_keyword!(LCL);
    syn::custom_keyword!(ARG);
    syn::custom_keyword!(THIS);
    syn::custom_keyword!(THAT);
    syn::custom_keyword!(SCREEN);
    syn::custom_keyword!(KBD);
    syn::custom_keyword!(R0);
    syn::custom_keyword!(R1);
    syn::custom_keyword!(R2);
    syn::custom_keyword!(R3);
    syn::custom_keyword!(R4);
    syn::custom_keyword!(R5);
    syn::custom_keyword!(R6);
    syn::custom_keyword!(R7);
    syn::custom_keyword!(R8);
    syn::custom_keyword!(R9);
    syn::custom_keyword!(R10);
    syn::custom_keyword!(R11);
    syn::custom_keyword!(R12);
    syn::custom_keyword!(R13);
    syn::custom_keyword!(R14);
    syn::custom_keyword!(R15);
    syn::custom_keyword!(MAX);
}

fn peek_dest(input: syn::parse::ParseStream) -> bool {
    input.peek(kw::A)
        || input.peek(kw::M)
        || input.peek(kw::D)
        || input.peek(kw::AM)
        || input.peek(kw::AD)
        || input.peek(kw::MD)
        || input.peek(kw::AMD)
}

fn peek_register(input: syn::parse::ParseStream) -> bool {
    input.peek(kw::SP)
        || input.peek(kw::LCL)
        || input.peek(kw::ARG)
        || input.peek(kw::THIS)
        || input.peek(kw::THAT)
        || input.peek(kw::SCREEN)
        || input.peek(kw::KBD)
        || input.peek(kw::R0)
        || input.peek(kw::R1)
        || input.peek(kw::R2)
        || input.peek(kw::R3)
        || input.peek(kw::R4)
        || input.peek(kw::R5)
        || input.peek(kw::R6)
        || input.peek(kw::R7)
        || input.peek(kw::R8)
        || input.peek(kw::R9)
        || input.peek(kw::R10)
        || input.peek(kw::R11)
        || input.peek(kw::R12)
        || input.peek(kw::R13)
        || input.peek(kw::R14)
        || input.peek(kw::R15)
}

fn peek_comp(input: syn::parse::ParseStream) -> bool {
    input.peek(kw::A)
        || input.peek(kw::D)
        || input.peek(Token![-])
        || input.peek(Token![!])
        || input.peek(LitInt)
}

// /// if the next token is potentially a dest or comp
// fn label_done(input: syn::parse::ParseStream) -> bool {
//     input.is_empty()
//         || peek_dest(input)
//         || input.peek(Token![!])
//         || input.peek(Token![-])
//         || input.peek(LitInt)
//         || input.peek(LitStr)
//         || input.peek(token::Paren)
//         || input.peek(Token![@])
// }

// fn label(input: syn::parse::ParseStream) -> syn::Result<String> {
//     let mut label = String::new();
//     while !label_done(input) {
//         if input.peek(Ident) {
//             let ident = input.parse::<Ident>()?;
//             label = format!("{label}{ident}");
//         } else if input.peek(Token![$]) {
//             input.parse::<Token![$]>()?;
//             label = format!("{label}$");
//         } else if input.peek(Token![.]) {
//             input.parse::<Token![.]>()?;
//             label = format!("{label}.");
//         }

//         if input.peek(LitInt) && !input.peek2(Token![;]) {
//             let i = input.parse::<LitInt>()?;
//             label = format!("{label}{i}");
//             continue;
//         }
//     }
//     Ok(label)
// }

fn comp_end(input: syn::parse::ParseStream) -> bool {
    input.is_empty()
        || input.peek(Token![@])
        || peek_dest(input)
        || input.peek(Paren)
        || input.peek(LitStr)
        || input.peek(Token![;])
        || (input.peek(LitInt) && !input.peek(Token![-]))
        || input.peek(Ident) && !(input.peek(kw::A) || input.peek(kw::D))
}

// TODO: Make spanned compile errors
impl syn::parse::Parse for Asm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let dest;
        let comp;
        let jump;
        let lookahead = input.lookahead1();
        if input.peek(Token![@]) {
            let _start = input.parse::<Token![@]>()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(LitInt) {
                let addr = input.parse::<LitInt>()?;
                Ok(Asm::At(HackLabel::Int(addr)))
            } else if peek_register(input) || input.peek(kw::MAX) {
                Ok(Self::At(HackLabel::Ident(input.parse::<Path>()?)))
            } else if lookahead.peek(LitStr) {
                Ok(Self::At(HackLabel::Str(input.parse::<LitStr>()?.value())))
            } else if lookahead.peek(Ident) {
                return Ok(Self::Var(input.parse::<Ident>()?));
            } else {
                Err(lookahead.error())
            }
        } else if input.peek(token::Paren) {
            let content;
            let _label: token::Paren = parenthesized!(content in input);
            let look = content.lookahead1();
            if look.peek(LitStr) {
                Ok(Self::Label(HackLabel::Str(
                    content.parse::<LitStr>()?.value(),
                )))
            } else if look.peek(Ident) {
                let ident = content.parse::<Ident>()?;
                if content.is_empty() {
                    return Ok(Self::Var(ident));
                } else {
                    return Err(look.error())
                }
            } else {
                Err(look.error())
            }
        } else if input.peek(syn::LitStr) {
            let comment = input.parse::<syn::LitStr>()?;
            Ok(Self::Comment(comment))
        } else if peek_comp(input) || (peek_dest(input) && input.peek2(Token![=])) {
            dest = if peek_dest(input) && input.peek2(Token![=]) {
                let d = input.parse::<Ident>()?;
                input.parse::<Token![=]>()?;
                d.to_token_stream()
            } else {
                quote!(None)
            };

            let look = input.lookahead1();
            // parse comp
            // 0 or 1
            if look.peek(syn::LitInt) {
                comp = match input.parse::<LitInt>()?.base10_parse() {
                    Ok(1) => quote!(One),
                    Ok(0) => quote!(Zero),
                    Ok(-1) => quote!(NegOne),
                    _ => return Err(look.error()),
                }
            } else if look.peek(Token![-]) || look.peek(Token![!]) {
                // -1, -A, -M, -D, !A, !M, !D
                let op = match input.parse::<UnOp>()? {
                    UnOp::Not(_) => quote!(Not),
                    UnOp::Neg(_) => quote!(Neg),
                    _ => return Err(look.error()),
                };
                let expr = match input.parse::<Expr>()? {
                    Expr::Lit(l) => match l.lit {
                        Lit::Int(i) => match i.base10_parse() {
                            Ok(1) => quote!(One),
                            Ok(0) => quote!(Zero),
                            _ => return Err(look.error())
                        },
                        _ => return Err(look.error()),
                    },
                    Expr::Path(p) => p.to_token_stream(),
                    _ => return Err(look.error()),
                };
                comp = format_ident!("{op}{expr}").to_token_stream();
            } else {
                let first = input.parse::<Ident>()?;
                if comp_end(input) {
                    comp = first.to_token_stream();
                } else {
                    let op = match input.parse::<BinOp>()? {
                        BinOp::Add(_) => quote!(Plus),
                        BinOp::Sub(_) => quote!(Minus),
                        BinOp::BitAnd(_) => quote!(And),
                        BinOp::BitOr(_) => quote!(Or),
                        _ => return Err(look.error()),
                    };
                    let second = if input.peek(LitInt) {
                        match input.parse::<LitInt>()?.base10_parse() {
                            Ok(1) => quote!(One),
                            Ok(0) => quote!(Zero),
                            _ => return Err(look.error()),
                        }
                    } else {
                        input.parse::<ExprPath>()?.to_token_stream()
                    };
                    comp = format_ident!("{first}{op}{second}").to_token_stream();
                } 
            }

            jump = if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
                let j = input.parse::<Ident>()?;
                j.to_token_stream()
            } else {
                quote!(Never)
            };

            Ok(Self::C(CInst { dest, comp, jump }))
        } else if input.peek(Ident) {
            Ok(Self::Var(input.parse::<Ident>()?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct AsmMacroInput {
    asm: Punctuated<syn::Result<Asm>, Comma>,
}

impl From<AsmMacroInput> for proc_macro2::TokenStream {
    fn from(value: AsmMacroInput) -> Self {
        let mut out = proc_macro2::TokenStream::new();
        if !value.asm.is_empty() {
            if value.asm.iter().count() > 1 {
                for pair in value.asm.into_pairs() {
                    let (asm, c) = pair.into_tuple();
                    match asm {
                        Ok(a) => {
                            out.extend(Into::<proc_macro2::TokenStream>::into(a));
                            out.extend(c.into_token_stream());
                        },
                        Err(e) => return e.into_compile_error(),
                    }
                }
                out = quote!([#out]);
            } else {
                out = value.asm.into_pairs().next().unwrap().into_value().unwrap().into();
            }
        }
        out
    }
}

impl syn::parse::Parse for AsmMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut out = Punctuated::new();
        // while let Ok(asm) = input.parse::<Asm>() {
        //    out.extend(std::iter::once(asm));
        // }
        while !input.is_empty() {
            out.extend(std::iter::once(input.parse::<Asm>()))
        }
        Ok(Self { asm: out })
    }
}

struct CInst {
    dest: proc_macro2::TokenStream,
    comp: proc_macro2::TokenStream,
    jump: proc_macro2::TokenStream,
}

enum Asm {
    Var(Ident),
    At(HackLabel),
    Comment(syn::LitStr),
    Label(HackLabel),
    C(CInst),
}

impl From<Asm> for proc_macro2::TokenStream {
    fn from(value: Asm) -> Self {
        match value {
            Asm::At(lit) => match lit {
                HackLabel::Ident(n) => quote!(crate::asm::Asm::#n),
                HackLabel::Str(s) => {
                    if s.contains('{') {
                        quote!(crate::asm::Asm::At(std::borrow::Cow::Owned(format!(#s))))
                    } else {
                        quote!(crate::asm::Asm::At(std::borrow::Cow::Borrowed(#s)))
                    }
                }
                HackLabel::Int(i) => quote!(crate::asm::Asm::Asm(crate::asm::Instruction::from(#i))),
            },
            Asm::Var(v) => quote!(crate::asm::Asm::from(#v.clone())),
            Asm::Comment(s) => {
                if s.value().contains('{') {
                    quote!(crate::asm::Asm::Comment(std::borrow::Cow::Owned(format!(#s))))
                } else {
                    quote!(crate::asm::Asm::Comment(std::borrow::Cow::Borrowed(#s)))
                }
            },
            Asm::Label(s) => match s {
                HackLabel::Str(s) => {
                    if s.contains('{') {
                        quote!(crate::asm::Asm::Label(std::borrow::Cow::Owned(format!(#s))))
                    } else {
                        quote!(crate::asm::Asm::Label(std::borrow::Cow::Borrowed(#s)))
                    }
                }
                HackLabel::Ident(n) => {
                    quote!(crate::asm::Asm::Label(std::borrow::Cow::Owned(#n.clone())))
                }
                _ => unreachable!(),
            },
            Asm::C(CInst { dest, comp, jump }) => {
                quote!(
                    crate::asm::Asm::Asm(
                    crate::asm::Instruction::c(crate::asm::Dest::#dest, crate::asm::Comp::#comp, crate::asm::Jump::#jump)
                ))
            }
        }
    }
}

#[proc_macro]
/// Macro for constructing `Asm` through Hack assembly
///
/// Built in labels like `@SP` are supported without string literals, but custom labels take an interpolated format string
///
/// Comments must be written as string literals, and line breaks can be made with an empty string literal
pub fn asm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AsmMacroInput);
    let output: proc_macro2::TokenStream = input.into();
    output.into()
}
