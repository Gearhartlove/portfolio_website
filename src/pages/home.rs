use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let html: Html = html! {
            <div>
                <div class="sidenav">
                    /* coulddo: center name better */
                    <h2 style="text-align: center; padding-top: 35px;">{"Kristoff \n Finley"}</h2>
                    // <h2 style="text-align: center;">{"Finley"}</h2>
                    <a href="\\">{"Home"}</a>
                    <a href="blog">{"Blog"}</a>
                    <a href="projects">{"Projects"}</a>
                    <a href="talks">{"Talks"}</a>
                    <a href="site_index">{"Site Index"}</a>
                </div>
                <div class="main">
                    <h1 style="text-align: center; padding-top: 50px;">{"Hallo, ich bins Kristoff"}</h1>
                    <p>{"test"}</p>
                </div>
            </div>
        };
        html
    }
}
