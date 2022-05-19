use yew::prelude::*;

pub enum Msg {
    Mutate,
    Serve,
}

pub struct Textbox {
    text: String,
}

impl Component for Textbox {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: String::from("empty")
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Mutate=> {
                self.text.push_str(" no longer");
                true
            }
            // Serve => {
            //
            //     true
            // }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        return html! {
            <div>
                <button onclick={link.callback(|_| Msg::Mutate)}> { "mutate" } </button>
                <button onclick={link.callback(|_| Msg::Serve)}> { "serve" } </button>
                <p> { self.text.clone() } </p>
                <input/>
            </div>
        }
    }
}