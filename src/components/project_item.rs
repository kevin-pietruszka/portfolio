use yew::prelude::*;
use crate::data::project::Project;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub p: Project,
}

pub struct ProjectItem {
    project: Project,
}

impl Component for ProjectItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            project: ctx.props().p.clone(),
        } 
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let image_path = self.project.image_path.clone();

        html! {
            <div class="box">
                <div class="columns">
                    <div class="column is-centered has-text-centered">
                        <figure class="image is-128x128">
                            <img src={image_path} />
                        </figure>
                        <p class="title is-4"> {self.project.title.as_str()} </p>
                    </div>

                    <div class="column is-centered has-text-centered"> 
                        {self.project.description.as_str() }

                    </div>
                </div>
            </div>
        }

        
    }
}