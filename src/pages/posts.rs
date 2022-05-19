use yew::prelude::*;

pub struct Posts;

impl Component for Posts {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <div>
                <p> { "these are my posts " } </p>
            </div>
        }
    }
}