//! **fetchtest**  

//needed for dodrio! macro (typed-html)
#![recursion_limit = "512"]
//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
//endregion

//region: extern and use statements
extern crate console_error_panic_hook;
extern crate log;
extern crate serde;
//#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
extern crate strum_macros;
extern crate web_sys;
#[macro_use]
extern crate unwrap;
extern crate conv;
extern crate wasm_bindgen_futures;

use dodrio::builder::text;
use wasm_bindgen::prelude::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
use serde::{Deserialize, Serialize};
use web_sys::{console, Request, RequestInit,  Response};
use wasm_bindgen::JsCast;
//endregion

///simple console write with a string
fn log1(x: &str) {
    console::log_1(&JsValue::from_str(x));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootRenderingComponent {
    pub response: String,
}

#[wasm_bindgen(start)]
pub fn run() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(
        document.get_element_by_id("div_for_virtual_dom"),
        "No #div_for_virtual_dom"
    );

    // Construct a new rendering component.
    let rrc = RootRenderingComponent::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Run the component forever.
    vdom.forget();
}

impl RootRenderingComponent {
    fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            response: "".to_string(),
        }
    }
}

// The `Render` implementation. It is called for every Dodrio animation frame to render the vdom.
impl Render for RootRenderingComponent {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        //create the virtual dom
        dodrio!(bump,
            <div>
                <h1>
                    {vec![text(
                        bumpalo::format!(in bump, "fetchtest - rust wasm-bindgen{}",
                        "")
                        .into_bump_str()
                    )]}
                </h1>
                <div>
                    {vec![text(
                        bumpalo::format!(in bump, "{}",
                        self.response)
                        .into_bump_str()
                    )]}
                </div>
                <button style= "margin:auto;display:block;" onclick={move |root, vdom, _event| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    fetch_rust();
                    vdom.schedule_render();
                }}>"fetch rust"</button>
                <div id="for_fetch_rust">
                </div>
            </div>
        )
    }
}

///fetch in Rust
#[wasm_bindgen]
pub fn fetch_rust() {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    //Firefox understand NoCors and works.
    //Chrome is very limiting with his CORB and does not understand NoCors.
    //opts.mode(RequestMode::NoCors);

    let request = unwrap!(Request::new_with_str_and_init(
        "https://jsonplaceholder.typicode.com/todos/1",
        &opts
    ));

    let window = unwrap!(
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))
    );
    //1. wasm_bindgen knows only method fetch_with_request, and that returns a promise
    //https://dev.to/werner/practical-rust-web-development-front-end-538d

    let clos_success_response = Closure::once(move |js_value: JsValue| {
        log1("clos_success_response");
        assert!(js_value.is_instance_of::<Response>());
        let resp: Response = unwrap!(js_value.dyn_into());

        //again a new promise and new Closures...
        let clos_success_text = Closure::once(move |js_value: JsValue| {
            log1("clos_success_text");
            assert!(js_value.is_string());
            let txt: String = unwrap!(js_value.as_string());
            log1("after as_string clos_success_text");
            let window = unwrap!(web_sys::window());
            let document = unwrap!(window.document());
            let div_for_fetch_rust = unwrap!(
                document.get_element_by_id("for_fetch_rust"),
                "No #for_fetch_rust"
            );
            div_for_fetch_rust.set_inner_html(&txt);
            log1(&txt);
        });
        let clos_error_text = Closure::once(move |js_value: JsValue| {
            log1("clos_error_text");
            console::log_1(&js_value);
        });

        //again a new promise...
        #[rustfmt::skip]
        unwrap!(resp.text())
            .then(&clos_success_text)
            .catch(&clos_error_text)
        ;

        //wilingly memory leaking
        clos_success_text.forget();
        clos_error_text.forget();
        log1("end clos_success_response");
    });

    let clos_error_response = Closure::once(move |js_value: JsValue| {
        log1("clos_error_response");
        console::log_1(&js_value);
    });

    #[rustfmt::skip]
    window
        .fetch_with_request(&request)
            .then(&clos_success_response)
            .catch(&clos_error_response)
        ;

    //this memory leak on purpose is the only way
    clos_error_response.forget();
    clos_success_response.forget();

}

