use yew::prelude::*;


pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero is-bold is-large is-fullheight">
                <div class="hero-body">
                    <div class="container has-text-centered">
                        <h1 class="title">
                            { "Page Not Found" }
                        </h1>
                        <h2 class="subtitle">
                            { "Invalid path" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}