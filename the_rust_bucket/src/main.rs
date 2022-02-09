use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{
    page_not_found::PageNotFound
};
use yew::html::Scope;

// testing routes
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    // #[at("/")]
    // Home,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleButton,
}

struct App {
    button_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            button_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleButton => {
                self.button_active = !self.button_active;
                true
            }
        }
    }

    fn view(&self, ctx:&Context<Self>) -> Html {
        // todo()! start by running the application and debugging . . .
        html! ({
            <BrowserRouter>
                { self.view_button(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by hard working robits" }
                    </div>
                </footer>
            </BrowserRouter>
        })
    }
}

impl App {
    fn view_button(&self, link: &Scope<Self>) -> Html {
        html!({ "temporary html "})
    }
}


fn switch(routes: &Route) -> Html {
    // hard code for now . . .
    html! ({ <PageNotFound />})
}

fn main() {
    // wasm_logger : api to log user interaction with your application
    // it does not add functionality to the application, just insights
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}