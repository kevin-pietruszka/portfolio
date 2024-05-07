use yew::prelude::*;

use crate::data::Experience;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct ExperiencePage {
    experience: Experience,
}

impl Component for ExperiencePage {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let experience = Experience::new(
            "2020-01-01",                   // begin
            "2021-01-01",                   // end
            "Software Engineer",            // title
            "Worked on building web applications using Rust and Yew framework.", // description
            vec!["Rust".to_string(), "Yew".to_string()],  // tags
        );
    
        Self {
            experience: experience
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="Empty"> </div>
        }
    }
}