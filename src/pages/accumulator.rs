use yew::prelude::*;

pub enum Msg {
    AddOne,
    SubOne,
}

pub struct Accumulator {
    value: i16,
}

impl Component for Accumulator {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // value changes so we need to "rerender" it to the page
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        // Gives the component's "Scope" which allows the sending of messages, etc to the component
        let link = ctx.link();
        return html! {
            <div>
                <p>{"test"}</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
