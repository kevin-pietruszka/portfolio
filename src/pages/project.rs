use yew::prelude::*;

use crate::data::project::Project;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct ProjectPage {
    project: Project,
}

impl Component for ProjectPage {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            project: Project {},
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="Empty"> </div>
        }
    }
}