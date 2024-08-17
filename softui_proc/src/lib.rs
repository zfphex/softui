#![feature(proc_macro_quote)]
use proc_macro::*;

extern crate proc_macro;

//We will get an input of layout!(text(), rect()) which is really just layout!(Text, Rect)

#[proc_macro]
pub fn layout(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let mut stream = vec!["{".to_string()];
    dbg!(&input);
    let mut stream = Vec::new();
    for item in input.into_iter() {
        match item {
            TokenTree::Ident(ident) => {
                stream.push(format!("{}.area();", ident));
            }
            _ => (),
        }
    }
    // stream.push("}".to_string());
    let stream = stream.join("");
    dbg!(&stream);

    stream.parse::<TokenStream>().unwrap()
}
