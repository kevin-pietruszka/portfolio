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
                <div class="card-image">
                    <figure class="image">
                        <img src={image_path} />
                    </figure>
                </div>

                <div class="card-content">
                    <div class="content has-text-centered">
                        <p class="title is-4"> {self.project.title.as_str() } </p>
                        <p> {self.project.description.as_str()} </p>
                    </div>
                </div>
                
                if self.project.is_public {
                    <div class="card-footer">
                        <a href={self.project.github_link.clone()} class="card-footer-item">
                            <span class="icon">
                                <i class="fab fa-github"> </i>
                            </span>
                            <span> {"Github"} </span>
                        </a>
                    </div>
                }
            </div>
        }

        
    }
}