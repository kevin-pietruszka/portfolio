use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let about_me = 
        "
        Hello, I am a recent graduate from the Georgia Institute of Technology where I partaked in the \
        BSMS program. I focused on the threads of Intelligence and Information Internetworks during my time \
        as an undergrad and Computing Systems for my Master's. I have experience working in full stack \
        development and DevOps roles and am currently exploring what I want to do in life. My current passion is \
        working with Rust and my first project with it was actually this website (see footer). Feel free to contact \
        me with any of the methods to the right.
        ";
        html! {
            <div class="section">
                <div class="hero">
                    <div class="hero-body">
                        <div class="container has-text-centered">
                            <figure class="image is-128x128 is-inline-block">
                                <img class="is-rounded" src="img/portrait.jpeg" />
                            </figure>
                            <p class="title is-1"> { "Kevin Pietruszka" }</p>
                            <p class="subtitle"> {"Recent College Graduate"} </p>
                        </div>
                    </div>
                </div>

                <div class="is-divider"></div>

                <div class="container is-max-desktop">
                    <div class="columns is-centered">
                        <div class="column is-one-quarter">
                            <div class="content has-text-centered">
                                <p class="title">{ "About Me" }</p>
                                <p>{about_me}</p>
                            </div>
                        </div>

                        <div class="is-divider-vertical"></div>

                        <div class="column is-one-quarter">
                            <div class="content has-text-centered">
                                <p class="title">{"Contact Me"}</p>
                                <p> <a target="_blank" rel="noreferrer noopener" href="mailto:kevinpiet13@gmail.com"> {"Email: " } </a> {"kevinpiet13@gmail.com"} </p>
                                <p> <a target="_blank" rel="noreferrer noopener" href="https://github.com/kevin-pietruszka">{"Github: "}</a> {"kevin-pietruszka"} </p>
                                <p> <a target="_blank" rel="noreferrer noopener" href="https://www.linkedin.com/in/kevin-pietruszka">{"LinkedIn: "}</a> {"Kevin Pietruszka"} </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}