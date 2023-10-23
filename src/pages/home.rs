use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::*;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::APOD));

    html! {
        <div class="h-screen flex items-center">
            <div class="container w-2/3 shadow-2xl rounded-xl py-3 bg-white">
                <div class="flex flex-col">
                    <h1 class="font-bold text-4xl self-center mb-5">{ "Welcome to the Yew-Primer!" }</h1>
                    <p class="self-center my-5">{ "Yew has been on my radar for a few months as a promising web solution.
                    I thought I would give it a try and see how it compared to my typical React 
                    usage. It's been nice, especially being in Rust which is such a 
                    lovely language. So far I've figured out how to setup a new yew app, 
                    create the file structure, add in routing, call an api, and use console 
                    logging. Also in this app will be some basic styling. This webapp can be 
                    used as a base for a new project, a reference for the basic building blocks, 
                    or to learn the basics of yew." }</p>
                    <button class="bg-gray-800 hover:bg-gray-600 text-white font-bold p-3 rounded-full w-auto self-center mt-5" onclick={onclick.clone()}>{ "See the Astronomy Photo of the Day!" }</button>
                </div>
            </div>
        </div>
    }
}
