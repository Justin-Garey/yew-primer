use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
