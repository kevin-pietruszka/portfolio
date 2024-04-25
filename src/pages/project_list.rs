use yew::prelude::*;

use crate::components::project_item::ProjectItem;

pub struct ProjectList;

impl Component for ProjectList {
    type Message = (); 
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        html! {
            <div class="section container">
                <h1 class="title"> { "Projects" } </h1>
                <h2 class="subtitle"> { "The projects I have done in my free time and in classes" } </h2>

                {self.view_projects(ctx)}
            </div>
        } 
    }
}

impl ProjectList {
    fn view_projects(&self, _ctx: &Context<Self>) -> Html {
        let test = (0..5).map(|index| {
            html! {
                <li class ="list-item mb-5">

                    <ProjectItem id={index}/>

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