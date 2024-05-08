use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="section">
                <div class="hero">
                    <div class="hero-body">
                        <div class="container has-text-centered">
                            <figure class="image is-128x128 is-inline-block">
                                <img class="is-rounded" src="https://bulma.io/assets/images/placeholders/128x128.png" />
                            </figure>
                            <p class="title is-1"> { "Kevin Pietruszka" }</p>
                            <p class="subtitle"> {"Subtitle"} </p>
                        </div>
                    </div>
                </div>

                <div class="is-divider"></div>

                <div class="container">
                    <div class="box">
                        <div class="columns">
                            <div class="column is-three-fifths">
                                <div class="content has-text-centered">
                                    <p class="title">{ "About Me" }</p>
                                    <p>{"Description here"}</p>
                                </div>
                            </div>

                            <div class="is-divider-vertical"></div>

                            <div class="column">
                                <div class="content has-text-centered">
                                    <p class="title"> {"Contact Me"} </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}