use yew::prelude::*;
use yew_router::prelude::*;

use yew::html::Scope;

use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::project_list::ProjectList;
use crate::pages::experience_list::ExperienceList;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/projects")]
    Projects,
    #[at("/experience")]
    Experience,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(route: Route) -> Html {
    match route {
        Route::Projects => {
            html!{<ProjectList />}
        },
        Route::Experience => {
            html!{ <ExperienceList />}
        },
        Route::Home => {
            html!{ <Home /> }
        },
        Route::NotFound => {
            html!{<PageNotFound />}
        },
    }
}

pub enum Msg {
    ToggleNavbar,
}
pub struct App {
    navbar_active: bool,
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {

        let Self { navbar_active, .. } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-fixed-top" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Portfolio" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Experience}>
                            { "Experience" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Projects}>
                            { "Projects" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }

    fn view_footer(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a target="_blank" rel="noreferrer noopener" href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a target="_blank" rel="noreferrer noopener" href="https://bulma.io">{ "Bulma" }</a>
                    { " with the lux theme provided by " }
                    <a target="_blank" rel="noreferrer noopener" href="https://jenil.github.io/bulmaswatch/">{ "Bulmaswatch" }</a>
                    {"."}
                </div>
            </footer>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            },
        }
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <Switch<Route> render={switch} />

                { self.view_footer() }

            </BrowserRouter>
        } 
    }
}