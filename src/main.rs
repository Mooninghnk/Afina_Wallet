#![allow(non_snake_case)]

use dioxus::desktop::wry::http::Error;
use dioxus::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U64;
use std::collections::HashMap;
use tracing::Level;
//Route enum with all of our routs for Router
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

//main func where we lounch the main diox app
fn main() {
    // Init logger

    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(
            Provider::<Http>::try_from(
                "https://mainnet.infura.io/v3/6c04a2cde6d145a6a684e3736393fea2",
            )
            .unwrap(),
        )
    });
    rsx! {
        Router::<Route> {}
    }
}

//fetches the last block_number form eth blockchain and diplayes it on the page
#[component]
fn BlockNum() -> Element {
    //async closure to get our block num
    let block_number = use_resource(|| async move {
        //signal passed across all components
        use_context::<Signal<Provider<Http>>>()()
            .get_block_number()
            .await
            .unwrap()
    });
    match &*block_number.read_unchecked() {
        Some(x) => rsx! {
            "{x}"
        },
        None => None,
    }
}

#[component]
fn Geb_Bln() -> Element {
    let mut inp = use_signal(|| String::new());
    let mut balance = use_signal(|| String::new());

  
    rsx!{
        input {
            value: "{inp}",
            oninput: move | event | inp.set(event.value())
        },
        
        button {
            onclick: move |event| {
                spawn(async move {
                   balance.set(use_context::<Signal<Provider<Http>>>()()
                    .get_balance(inp().trim(), None)
                    .await
                    .unwrap().to_string())
         
                });
            },
            //add conversion for balance
            h1{"{balance}"}
        }
        
    }
    
}




#[component]
fn Home() -> Element {
    rsx! {
        BlockNum {}
        Geb_Bln {}
    }
}
