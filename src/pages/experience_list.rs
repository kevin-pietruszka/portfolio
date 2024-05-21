use yew::prelude::*;

use crate::components::experience_item::ExperienceItem;
use crate::data::experience::Experience;
pub struct ExperienceList {
    experiences: Vec<Experience>,
}

impl Component for ExperienceList {
    type Message = (); 
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        Self {
            experiences: Experience::data(), 
        }
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
        
        let test = self.experiences.iter().map(|e| html! {<ExperienceItem e={e.clone()} />});

        html! {
            <div class="container">
                <div class="columns is-centered">
                    <div class="column is-two-fifths">
                            { for test }
                    </div>
                </div>
            </div>
        }
    }
}