use yew::prelude::*;

use crate::data::{Data, Project};

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct ExperienceItem {
    experience: Project,
}

impl Component for ExperienceItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            experience: Project::create_from_id(ctx.props().id),
        } 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <a class="list-item">
                { "Name go here " }
            </a>
        }

        
    }
}