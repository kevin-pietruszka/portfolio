use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/projects/:id")]
    Project { id: u64 },
    #[at("/projects")]
    Projects,
    #[at("/skills")]
    Skills,
    #[at("/experience/:id")]
    Experience { id: u64 },
    #[at("/experience")]
    Experiences,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(route: Route) {
    match route {
        Route::Project { id } => {
            println!("Navigating to Project with ID: {}", id);
        },
        Route::Projects => {
            println!("Navigating to Projects List");
        },
        Route::Skills => {
            println!("Navigating to Skills List");
        },
        Route::Experience { id } => {
            println!("Navigating to Experience with ID: {}", id);
        },
        Route::Experiences => {
            println!("Navigating to Experiences List");
        },
        Route::Home => {
            println!("Navigating to Home");
        },
        Route::NotFound => {
            println!("Page Not Found (404)");
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}