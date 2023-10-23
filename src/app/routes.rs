use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::apod::APOD;
use crate::pages::home::Home;

// Define the Route
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    RouteHome,
    #[at("/home")]
    Home,
    #[at("/PhotoOfTheDay")]
    APOD,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::RouteHome => html! {
            <Redirect<Route> to={Route::Home}/>
        },
        Route::Home => html! {
            <Home />
        },
        Route::APOD => html! {
            <APOD />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}