use yew::prelude::*;

use crate::components::experience_item::ExperienceItem;
pub struct ExperienceList;

impl Component for ExperienceList {
    type Message = (); 
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <div class="section container">
                <h1 class="title"> { "My Experience" } </h1>
                <h2 class="subtitle"> {"All of my roles throughout the years"} </h2>

                {self.view_experience(ctx)}
            </div>
        } 
    }
}

impl ExperienceList {
    fn view_experience(&self, _ctx: &Context<Self>) -> Html {
        let test = (0..5).map(|index| {
            html! {
                <li class ="list-item mb-5">

                    <ExperienceItem id={index}/>

                </li>
            }
        });


        html! {
            <div class="column">
                <ul class="list">
                    {for test}
                </ul>
            </div>
        }
    }
}