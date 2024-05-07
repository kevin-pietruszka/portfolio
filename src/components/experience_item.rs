use yew::prelude::*;
use crate::data::Experience;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub e: Experience,
}

pub struct ExperienceItem {
    experience: Experience,
}

impl Component for ExperienceItem {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            experience: ctx.props().e.clone(),
        } 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <>
                <header class="timeline-header">
                    <span class="tag is-large is-primary"> { "DATE" }  </span>
                </header>
                <div class="timeline-item">
                    <div class="timeline-marker"></div>
                    <div class="timeline-content">
                        <div class="box">
                            <p class="heading"> {"Title - Company"} </p>
                            <p> {"Description"} </p>
                            <p> {"Tags"} </p>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}