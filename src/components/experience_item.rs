use yew::prelude::*;
use yew_router::components::Link;

use crate::{app::Route, data::{Data, Experience}};

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: u64,
}

pub struct ExperienceItem {
    experience: Experience,
}

impl Component for ExperienceItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            experience: Experience::create_from_id(ctx.props().id),
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