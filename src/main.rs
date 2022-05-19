use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use pages::{
    page_not_found::PageNotFound, home::Home, posts::Posts, accumulator::Accumulator,
    textbox::Textbox,
};
use yew::html::Scope;

// testing routes
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/acc")]
    Accumulator,
    #[at("/textbox")]
    TextBox,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar
}

struct App {
    navbar_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "" }
                    </div>
                </footer>
            </BrowserRouter>
        };
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        return html!{}
    }
}


//noinspection ALL
fn switch(routes: &Route) -> Html {
    // hard code for now . . .
    return match routes.clone() {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Accumulator => {
            html! { <Accumulator />}
        }
        Route::TextBox => {
            html! { <Textbox /> }
        }
    }
}

fn main() {
    // wasm_logger : api to log user interaction with your application
    // it does not add functionality to the application, just insights
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
