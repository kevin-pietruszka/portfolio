use yew::prelude::*;

use crate::{components::experience_item::ExperienceItem, data::Experience};
pub struct ExperienceList;

impl Component for ExperienceList {
    type Message = (); 
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <div class="section">
                <div class="hero">
                    <div class="hero-body">
                        <div class="container has-text-centered">
                            <p class="title is-1"> { "My Roles" }</p>
                        </div>
                    </div>
                </div>

                <div class="is-divider"></div>

                {self.view_experience(ctx)}
            </div>
        } 
    }
}

impl ExperienceList {
    fn view_experience(&self, _ctx: &Context<Self>) -> Html {
        let test = (0..2).map(|index| {
            let experience = Experience::new(
                "2020-01-01",                   // begin
                "2021-01-01",                   // end
                "Software Engineer",            // title
                "Worked on building web applications using Rust and Yew framework.", // description
                vec!["Rust".to_string(), "Yew".to_string()],  // tags
            );

            html! {
                <ExperienceItem e={experience}/>
            }
        });

        html! {
            <div class="timeline is-centered">
                { for test }
                <div class="timeline-header">
                    <span class="tag is-medium is-primary"> {"End"} </span>
                </div>
            </div>
        }
    }
}