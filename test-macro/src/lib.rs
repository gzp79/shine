use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{parse_macro_input, Attribute, AttributeArgs, Ident, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let mut test_decors = Vec::new();
    // wasm_bindgen_test for wasm targets
    test_decors.push(quote! { #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen_test::wasm_bindgen_test)] });
    if input.sig.asyncness.is_some() {
        // tokio::test for async test
        test_decors.push(quote! { #[cfg_attr(not(target_arch = "wasm32"), ::tokio::test(flavor = "multi_thread"))] });
    } else {
        // core::test for none-async test
        test_decors.push(quote! { #[cfg_attr(not(target_arch = "wasm32"), ::core::prelude::v1::test)] });
    };

    /*if input.sig.asyncness.is_some() {
        // tokio::test for async test
        test_decors.push(quote! { #[::shine_test::impl_test_async] });
    } else {
        // core::test for none-async test
        test_decors.push(quote! { #[::shine_test::impl_test] });
    };*/

    let test_attrs = match args.as_slice() {
        [] => input.attrs.clone(),
        //todo: serial, or serial = "key" -> input.attrs.clone().append("#[::serial_test::serial(key)]")
        _ => panic!("unsupported attributes supplied: {args:?}"),
    };

    expand_wrapper(&test_decors, &test_attrs, &input)
}

/// Expand the wasm bindgen configuration, By default all tests are running in (headless) browser.
fn expand_wasm_bindgen_test_configure(test_name: &Ident) -> Tokens {
    quote! {
      #[cfg(target_arch = "wasm32")]
      mod #test_name {
        ::wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
      }
    }
}

/// Emit code for a wrapper function around a test function.
fn expand_wrapper(test_decors: &[Tokens], test_attrs: &[Attribute], input: &ItemFn) -> TokenStream {
    let async_token = &input.sig.asyncness;
    let await_token = async_token.map(|_| quote! {.await});

    let body = &input.block;
    let test_name = &input.sig.ident;

    // Note that Rust does not allow us to have a test function with
    // #[should_panic] that has a non-unit return value.
    let ret = match &input.sig.output {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, ty) => quote! {-> #ty},
    };

    let wasm_bindgen_test_configure = expand_wasm_bindgen_test_configure(test_name);

    let result = quote! {
      #wasm_bindgen_test_configure

      #(#test_decors)*
      #(#test_attrs)*
      #async_token fn #test_name() #ret {
        #async_token fn test_impl() #ret {
          #body
        }

        ::shine_test::setup_test();

        test_impl()#await_token
      }
    };
    result.into()
}
