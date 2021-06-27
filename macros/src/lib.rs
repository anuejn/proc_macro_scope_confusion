#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

#[proc_macro]
pub fn demo_proc_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("{:?}", proc_macro::Span::call_site());
    proc_macro::TokenTree::Ident(proc_macro::Ident::new("a", proc_macro::Span::call_site())).into()
}
