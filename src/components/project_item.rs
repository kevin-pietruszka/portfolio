use yew::prelude::*;
use yew_router::components::Link;

use crate::data::project::Project;

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
            project: Project {},
        } 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            {"Test"}
        }

        
    }
}