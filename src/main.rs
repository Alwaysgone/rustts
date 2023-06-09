use std::{collections::HashMap, fs::File, io::Write};

use log::{info, error};
use dotenv::dotenv;
use serde::Deserialize;
use url::Url;
use sycamore::prelude::*;
use sycamore::suspense::*;
use sycamore::rt::JsCast;
use web_sys::HtmlSelectElement;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct TTSResponse {
    Error: u8,
    Speaker: String,
    Cached: u8,
    Text: String,
    tasktype: String,
    success: Option<u8>,
    URL: Url,
    MP3: String,
}

#[component]
async fn TTS<G: Html>(cx: Scope<'_>) -> View<G> {
    let tts_text = create_signal(cx, String::from(""));
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let speaker_selection = document.get_element_by_id("speaker").unwrap()
    // .dyn_ref::<HtmlSelectElement>()
    // .unwrap();
    view! { cx,
            label(for="tts_text_input") {
                "TTS text input:"
            }
            textarea(id="tts_text_input", cols="40", rows="5", bind:value=tts_text)
            button(id="tts_submit", on:click=move |_| {
                    let window = web_sys::window().expect("no global `window` exists");
                    let document = window.document().expect("should have a document on window");
                    // let speaker_selection = document.get_element_by_id("speaker").unwrap();
                    // speaker_selection.child_nodes().for_each(|cn| {
                    //     web_sys::console::log_1(&format!("Child node {:?}", cn).into());    
                    // });
                    // let speaker = speaker_selection.options[speaker_selection.selectedIndex].value;
                    let speaker_selection = document.get_element_by_id("speaker").unwrap();
                    let speaker_content = &format!("{:?}", speaker_selection);
                    web_sys::console::log_1(&format!("Speaker: {}", speaker_content).into());
                   
                    // let speaker = speaker_selection.node_value().unwrap();
                    let speaker = "Salli";
                    web_sys::console::log_1(&format!("Would run TTS with speaker {} for text \"{}\"", speaker, tts_text).into());
                }) {
                "Create voice sample"
            }
    }
}

fn main() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let main_div = document.get_element_by_id("tts_input_div").unwrap();
    sycamore::render_to(|cx| {
        view! { cx,
            Suspense(fallback=view! {cx,
                "Loading DemandCurve..."
            }) {
                TTS {}
            }
        }
    }, &main_div);
}

// #[tokio::main]
async fn main2() {
    dotenv().ok();
    env_logger::init();
    let client = reqwest::Client::new();
    //https://ttsmp3.com/
    let url = "https://ttsmp3.com/makemp3_new.php";
    let mut params = HashMap::new();
    params.insert("msg", "Words to say to test the url encoding.");
    params.insert("lang", "Salli");
    params.insert("source", "ttsmp3");
    // let mut params = "msg=".to_owned();
    // params.push_str(&urlencoding::encode("Sample"));
    // params.push_str("&lang=Salli");
    // params.push_str("&source=ttsmp3");
    // info!("Params: {}", params);
    match client.post(url)
    // .header("Content-type", "application/x-www-form-urlencoded")
    .header("Accept", "application/json")
    .form(&params)
   .send().await {
        Ok(response) => {
            info!("Got response: {:?}", response);
            let tts_response = response.json::<TTSResponse>().await;
            // let body = response.text().await;
            match tts_response {
                Ok(content) =>{
                    info!("TTSResponse: {:?}", content);
                    match client.get(content.URL)
                    .send()
                    .await {
                        Ok(mp3_response) => {
                            match mp3_response.bytes().await {
                                Ok(mp3_response_content) => {
                                    let mut f = File::create("C:/Develop/Git/rustts/downloads/".to_owned() + &content.MP3).unwrap();
                                    f.write_all(&mp3_response_content).unwrap();
                                },
                                Err(e) => error!("Could not read mp3 response body: {}", e),
                            }
                        },
                        Err(e) => error!("Could not get mp3 file: {}", e),
                    }
                },
                Err(e) => error!("Could not read body: {}", e),
            }
        },
        Err(e) => error!("Got an error: {}", e),
    }

}
