use yew::prelude::*;
use crate::data::experience::Experience;

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let tags = self.experience.tags.iter().map(|t| html!{<span class="tag is-primary"> {t} </span>});
        let link = self.experience.company_link.clone();
        html! {
            <div class="box has-text-centered">
                <p class="subtitle is-3"> 
                    { &format!("{} - ", self.experience.title) }
                    <a href={link} target="_blank">{ &self.experience.company }</a>
                </p>
                <p class="description"> { &self.experience.description } </p>
                <div class="is-divider"> </div>
                <div class="tags is-centered"> {for tags} </div>
            </div>
        }
    }
}