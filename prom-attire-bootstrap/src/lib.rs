extern crate proc_macro;
extern crate syn;

extern crate prom_attire_impl;

#[proc_macro_derive(FromAttributesBootstrap)]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let config = prom_attire_impl::Config {
        scope: Some("prom_attire"),
    };
    let input = &input.to_string();
    prom_attire_impl::derive(input, config).unwrap().parse().unwrap()
}