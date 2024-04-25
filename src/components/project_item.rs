use yew::prelude::*;
use yew_router::components::Link;

use crate::{app::Route, data::{Data, Project}};

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct ProjectItem {
    project: Project,
}

impl Component for ProjectItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            project: Project::create_from_id(ctx.props().id),
        } 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <div class="card">
                <div class="card-content">
                    <Link<Route> classes={classes!("title", "is-block")} to={Route::Home}>
                        { "Test" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Home}>
                        { "Test description" }
                    </Link<Route>>
                </div>
            </div>
        }

        
    }
}