//! **amafatt - processes the data from Amazon txt file and send an api json request to FattureInCloud.it**  

// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(non_snake_case)]

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

use wasm_bindgen_futures::future_to_promise;
use dodrio::builder::text;
use wasm_bindgen::prelude::*;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
use futures::{ Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use web_sys::{console, Request, RequestInit, RequestMode, Response,ErrorEvent};
use wasm_bindgen_futures::{JsFuture};

//use futures::{FutureExt, TryFutureExt};
use wasm_bindgen::JsCast;
//use wasm_bindgen_futures::futures_0_3::{spawn_local, JsFuture};


//endregion

fn log1(x: &str) {
    console::log_1(&JsValue::from_str(x));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RootRenderingComponent {
    tipo_doc: String,
    fatture: Fatture,
    json: String,
    response: String,
    pub new_message: String,
    pub response_data: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
///tipo-doc:  fatture
struct Fatture {
    api_uid: String,
    api_key: String,
    id_cliente: String,
    id_fornitore: String,
    nome: String,
    indirizzo_via: String,
    indirizzo_cap: String,
    indirizzo_citta: String,
    indirizzo_provincia: String,
    indirizzo_extra: String,
    paese: String,
    paese_iso: String,
    lingua: String,
    piva: String,
    cf: String,
    autocompila_anagrafica: bool,
    salva_anagrafica: bool,
    numero: String,
    data: String,
    valuta: String,
    valuta_cambio: i32,
    prezzi_ivati: bool,
    rivalsa: i32,
    cassa: i32,
    rit_acconto: i32,
    imponibile_ritenuta: i32,
    rit_altra: i32,
    marca_bollo: i32,
    oggetto_visibile: String,
    oggetto_interno: String,
    centro_ricavo: String,
    centro_costo: String,
    note: String,
    nascondi_scadenza: bool,
    ddt: bool,
    ftacc: bool,
    id_template: String,
    ddt_id_template: String,
    ftacc_id_template: String,
    mostra_info_pagamento: bool,
    metodo_pagamento: String,
    metodo_titoloN: String,
    metodo_descN: String,
    mostra_totali: String,
    mostra_bottone_paypal: bool,
    mostra_bottone_bonifico: bool,
    mostra_bottone_notifica: bool,
    lista_articoli: Vec<ListaArticoli>,
    lista_pagamenti: Vec<ListaPagamenti>,
    ddt_numero: String,
    ddt_data: String,
    ddt_colli: String,
    ddt_peso: String,
    ddt_causale: String,
    ddt_luogo: String,
    ddt_trasportatore: String,
    ddt_annotazioni: String,
    PA: bool,
    PA_tipo_cliente: String,
    PA_tipo: String,
    PA_numero: String,
    PA_data: String,
    PA_cup: String,
    PA_cig: String,
    PA_codice: String,
    PA_pec: String,
    PA_esigibilita: String,
    PA_modalita_pagamento: String,
    PA_istituto_credito: String,
    PA_iban: String,
    PA_beneficiario: String,
    extra_anagrafica: ExtraAnagrafica,
    split_payment: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ListaArticoli {
    id: String,
    codice: String,
    nome: String,
    um: String,
    quantita: i32,
    descrizione: String,
    categoria: String,
    prezzo_netto: i32,
    prezzo_lordo: i32,
    cod_iva: i32,
    tassabile: bool,
    sconto: i32,
    applica_ra_contributi: bool,
    ordine: i32,
    sconto_rosso: i32,
    in_ddt: bool,
    magazzino: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ListaPagamenti {
    data_scadenza: String,
    importo: i32,
    metodo: String,
    data_saldo: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ExtraAnagrafica {
    mail: String,
    tel: String,
    fax: String,
}

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[wasm_bindgen(start)]
pub fn run() {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let body = unwrap!(document.body());

    // Construct a new rendering component.
    let rrc = RootRenderingComponent::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&body, rrc);

    // Run the component forever.
    vdom.forget();
}

impl RootRenderingComponent {
    fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            tipo_doc: "fatture".to_string(),
            fatture: Fatture::new(),
            json: "".to_string(),
            response: "".to_string(),
            new_message: "".to_string(),
            response_data: "".to_string(),
        }
    }
}
impl Fatture {
    /// Construct a new component.
    fn new() -> Fatture {
        log1("Fatture.new");

        //return from constructor
        Fatture {
            api_uid: "604551".to_string(),
            api_key: "aa2cadd98c03971fc51efb011f1db428".to_string(),
            id_cliente: "0".to_string(),
            id_fornitore: "0".to_string(),
            nome: "Mario Rossi".to_string(),
            indirizzo_via: "Via delle Betulle, 123".to_string(),
            indirizzo_cap: "21012".to_string(),
            indirizzo_citta: "Curno".to_string(),
            indirizzo_provincia: "BG".to_string(),
            indirizzo_extra: "".to_string(),
            paese: "Italia".to_string(),
            paese_iso: "IT".to_string(),
            lingua: "it".to_string(),
            piva: "IT1234567890".to_string(),
            cf: "ABCDEF12G34H567I".to_string(),
            autocompila_anagrafica: false,
            salva_anagrafica: false,
            numero: "1a".to_string(),
            data: "10/07/2019".to_string(),
            valuta: "EUR".to_string(),
            valuta_cambio: 1,
            prezzi_ivati: false,
            rivalsa: 0,
            cassa: 0,
            rit_acconto: 0,
            imponibile_ritenuta: 0,
            rit_altra: 0,
            marca_bollo: 0,
            oggetto_visibile: "".to_string(),
            oggetto_interno: "".to_string(),
            centro_ricavo: "".to_string(),
            centro_costo: "".to_string(),
            note: "".to_string(),
            nascondi_scadenza: false,
            ddt: false,
            ftacc: false,
            id_template: "0".to_string(),
            ddt_id_template: "0".to_string(),
            ftacc_id_template: "0".to_string(),
            mostra_info_pagamento: false,
            metodo_pagamento: "Bonifico".to_string(),
            metodo_titoloN: "IBAN".to_string(),
            metodo_descN: "IT01A2345678900000000001234".to_string(),
            mostra_totali: "tutti".to_string(),
            mostra_bottone_paypal: false,
            mostra_bottone_bonifico: false,
            mostra_bottone_notifica: false,
            lista_articoli: vec![ListaArticoli {
                id: "0".to_string(),
                codice: "".to_string(),
                nome: "Articolo 1".to_string(),
                um: "".to_string(),
                quantita: 1,
                descrizione: "".to_string(),
                categoria: "".to_string(),
                prezzo_netto: 0,
                prezzo_lordo: 0,
                cod_iva: 0,
                tassabile: true,
                sconto: 0,
                applica_ra_contributi: true,
                ordine: 0,
                sconto_rosso: 0,
                in_ddt: false,
                magazzino: true,
            }],
            lista_pagamenti: vec![ListaPagamenti {
                data_scadenza: "10/07/2019".to_string(),
                importo: 0,
                metodo: "not".to_string(),
                data_saldo: "10/07/2019".to_string(),
            }],
            ddt_numero: "".to_string(),
            ddt_data: "10/07/2019".to_string(),
            ddt_colli: "".to_string(),
            ddt_peso: "".to_string(),
            ddt_causale: "".to_string(),
            ddt_luogo: "".to_string(),
            ddt_trasportatore: "".to_string(),
            ddt_annotazioni: "".to_string(),
            PA: false,
            PA_tipo_cliente: "PA".to_string(),
            PA_tipo: "nessuno".to_string(),
            PA_numero: "".to_string(),
            PA_data: "10/07/2019".to_string(),
            PA_cup: "".to_string(),
            PA_cig: "".to_string(),
            PA_codice: "".to_string(),
            PA_pec: "".to_string(),
            PA_esigibilita: "N".to_string(),
            PA_modalita_pagamento: "MP01".to_string(),
            PA_istituto_credito: "".to_string(),
            PA_iban: "".to_string(),
            PA_beneficiario: "".to_string(),
            extra_anagrafica: ExtraAnagrafica {
                mail: "info@mariorossi.it".to_string(),
                tel: "012345678".to_string(),
                fax: "012345678".to_string(),
            },
            split_payment: true,
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
                        bumpalo::format!(in bump, "amafatt{}",
                        "")
                        .into_bump_str()
                    )]}
                </h1>
                <h2>
                    {vec![text(
                        bumpalo::format!(in bump, "prototipo v0.1.0{}",
                        "")
                        .into_bump_str()
                    )]}
                </h2>
                <form>
                    <div class= "grid_container" style="grid-template-columns:auto auto">
                        //one row, two columns
                        <div class= "grid_item">
                            {vec![text(
                                bumpalo::format!(in bump, "api_uid{}",
                                "")
                                .into_bump_str()
                            )]}
                        </div>
                        <div class= "grid_item">
                            //TODO: if the value changes, must change in struct also
                            <input type="text" name="api_uid" value={self.fatture.api_uid.clone()}></input>
                        </div>

                        //one row, two columns
                        <div class= "grid_item">
                            {vec![text(
                                bumpalo::format!(in bump, "api_key{}",
                                "")
                                .into_bump_str()
                            )]}
                        </div>
                        <div class= "grid_item">
                        //TODO: if the value changes, must change in struct also
                        <input type="text" name="api_key" value={self.fatture.api_key.clone()}></input>
                        </div>
                    </div>
                </form>
                <button style= "margin:auto;display:block;" onclick={move |root, vdom, _event| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    create_json(rrc);
                    vdom.schedule_render();
                }}>"create json"</button>
                <div>
                    {vec![text(
                                bumpalo::format!(in bump, "{}",
                                self.json)
                                .into_bump_str()
                            )]}
                </div>

                <button style= "margin:auto;display:block;" onclick={move |root, vdom, _event| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
              let success_response =
                    Closure::once(move |js_value: JsValue| {
                      log1("success_response");
                      console::log_1(&js_value);
                      log1("end success_response");
                      //let resp: Response = js_value.dyn_into().unwrap();
                    });
              let error_response =
                    Closure::once(move |js_value: JsValue| {
                      log1("error_response");
                      console::log_1(&js_value);
                      log1("end of error_response");
                      let response: &ErrorEvent = js_value.as_ref().unchecked_ref();
                      let text = response.message();
                      log1(&text);
                    });

                    fetch_request("https://api.fattureincloud.it/v1/richiesta/info", "{\"api_uid\":\"604551\", \"api_key\":\"aa2cadd98c03971fc51efb011f1db428\"}")
                        .then(&success_response)
                        .catch(&error_response)
                        ;

                    error_response.forget();
                    success_response.forget();

                    vdom.schedule_render();
                }}>"send json"</button>
            </div>
        )
    }
}
///create json
fn create_json(rrc: &mut RootRenderingComponent) {
    rrc.json = unwrap!(serde_json::to_string_pretty(&rrc.fatture));
}

///this returns a promise
#[wasm_bindgen]
pub fn fetch_request(url: &str, body: &str) -> Promise {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::NoCors);
    let js_value = JsValue::from_str(body);
    opts.body(Some(&js_value));

    let request = unwrap!(Request::new_with_str_and_init(url, &opts));

    unwrap!(request.headers().set("Content-Type", "text/json"));

    let window = unwrap!(
        web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))
    );
    let request_promise = window.fetch_with_request(&request);
    //https://dev.to/werner/practical-rust-web-development-front-end-538d

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            log1("1. and_then");
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into()?;
            resp.json()
        })
        .and_then(|json_value: Promise| {
            log1("2. and_then");
            JsFuture::from(json_value)
        });
    log1("future to promise");
    future_to_promise(future)
}

