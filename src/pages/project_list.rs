use yew::prelude::*;

use crate::{components::project_item::ProjectItem, data::project::Project};

pub struct ProjectList {
    projects: Vec<Project>,
}

impl Component for ProjectList {
    type Message = (); 
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            projects: Project::data(),
        }
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

                <div class="columns is-centered">
                    <div class="column is-four-fifths">
                        { self.view_projects(ctx) }
                    </div>
                </div>
            </div>
        } 
    }
}

impl ProjectList {
    fn view_projects(&self, _ctx: &Context<Self>) -> Html {
        let mut children = self.projects.iter().map(|project| {
            html! {
                <ProjectItem p={project.clone()}/>
            }
        });

        let number_of_columns = 3;

        let length = children.len();
        let partitions = length / number_of_columns;
        let mut remainder = length % number_of_columns;

        let columns = (0..number_of_columns).into_iter().map(|c| {
            if remainder > 0 {
                remainder -= 1;
                html! {
                    <div class="column">
                        {for children.by_ref().take(partitions + 1)}
                    </div>
                }
            } else {
                html! {
                    <div class="column">
                        {for children.by_ref().take(partitions)}
                    </div>
                }
            }
        });

        html! {
            <div class="columns">
                {for columns}
            </div>
        }
    }
}