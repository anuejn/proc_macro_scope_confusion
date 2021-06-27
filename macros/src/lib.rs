#[proc_macro]
pub fn demo_proc_macro(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenTree::Ident(proc_macro::Ident::new("a", proc_macro::Span::call_site())).into()
}