use crate::app::routes::*;
use gloo_net::http::Request;
use serde_json;
use yew::prelude::*;
use yew_router::prelude::*;

fn rem_first_and_last(value: &str) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

#[function_component(APOD)]
pub fn apod() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let apod: UseStateHandle<serde_json::Value> = use_state(|| serde_json::json!({}));
    {
        let apod = apod.clone();
        use_effect_with((), move |_| {
            let apod = apod.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let res: serde_json::Value = Request::get("https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY")
                    .send().await.unwrap().json().await.unwrap();
                apod.set(res)
            });
            || (log::info!("LOL"))
        });
    }

    if apod.get("explanation") != None {
        let title = rem_first_and_last(&apod.get("title").unwrap().to_string());
        let hdurl = rem_first_and_last(&apod.get("hdurl").unwrap().to_string());
        let explanation = rem_first_and_last(&apod.get("explanation").unwrap().to_string());
        // let copyright = apod.get("copyright").unwrap().to_string();
        // let media_type = apod.get("media_type").unwrap().to_string();
        // let service_version = apod.get("service_version").unwrap().to_string();
        // let url = apod.get("url").unwrap().to_string();
        html! {
            <div>
                <button class="underline text-sm p-3 rounded-full w-auto m-3 p-1" {onclick}>{ "Phone Home" }</button>
                <div class="flex items-center">
                    <div class="container w-2/3 shadow-2xl rounded-xl py-3">
                        <div class="flex flex-col">
                            <h1 class="font-bold text-4xl self-center mb-5">{ title }</h1>
                            <img class="h-[60vh] object-scale-down" src={hdurl} alt="Astronomy Photo of the Day"/>
                            <p class="pt-5 self-center">{ explanation }</p>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <button class="underline text-sm p-3 rounded-full w-auto m-3 p-1" {onclick}>{ "Phone Home" }</button>
        }
    }
}
