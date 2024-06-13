use proc_macro::TokenStream;
mod json_scheme;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", input);
    "fn hello() {println!(\"hello world\");}".parse().unwrap()
}

#[proc_macro]
pub fn gen(input: TokenStream) -> TokenStream {
    eprintln!("{:#?}", input);
    TokenStream::default()
}