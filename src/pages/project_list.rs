use yew::prelude::*;

use crate::components::project_item::ProjectItem;

pub struct ProjectList;

impl Component for ProjectList {
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
                            <p class="title is-1"> { "Projects I have done" }</p>
                        </div>
                    </div>
                </div>

                <div class="is-divider"></div>

                { self.view_projects(ctx) }
            </div>
        } 
    }
}

impl ProjectList {
    fn view_projects(&self, _ctx: &Context<Self>) -> Html {
        let test = (0..10).map(|index| {
            html! {
                <li class ="list-item">

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