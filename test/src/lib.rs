pub use shine_test_macro::test;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_test::wasm_bindgen_test as impl_test_async;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_test::wasm_bindgen_test as impl_test;

#[cfg(not(target_arch = "wasm32"))]
pub use core::prelude::v1::test as impl_test;
#[cfg(not(target_arch = "wasm32"))]
pub use tokio::test as impl_test_async;

/// Test setup executed before each test.
pub fn setup_test() {
    #[cfg(not(any(target_arch = "wasm32", miri)))]
    {
        let _ = env_logger::builder().is_test(true).try_init();
        color_backtrace::install();
    }

    #[cfg(target_arch = "wasm32")]
    {
        // logger it should be initialized only once otherwise some warning it thrown
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(|| wasm_logger::init(::wasm_logger::Config::new(log::Level::Trace)));
    }
}
