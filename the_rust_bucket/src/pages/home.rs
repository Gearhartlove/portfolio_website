use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let html: Html = html! {
            <div class="sidenav">
                <h2 style="text-align: center;">{"Kristoff Finley"}</h2>
                <a href="\\">{"Home"}</a>
                <a href="acc">{"Blog"}</a>
                <a href="projects">{"Projects"}</a>
                <a href="acc">{"Talks"}</a>
                <a href="acc">{"Site Index"}</a>
            </div>
        };
        html
    }
    // fn view(&self, _ctx: &Context<Self>) -> Html {
    //     return html! {
    //         <div class="row">
    //             <br/>
    //             <br/>
    //             <br/>
    //             <br/>
    //             <br/>
    //             <p class="centered"> { "Hi! My name's Kristoff and welcome to my website!" } </p>
    //             <p class="centered"> { "Below you can find my projects, posts, and podcasts" } </p>
    //             <p class="centered" style="font-size:12px;"> { "<made with rust and love>" } </p>
    //             <br/>
    //             <br/>
    //             <br/>
    //             <div class="column"
    //             style="background-color:#aaa;">
    //                 <div class="center">
    //                     <h3> { "Posts" } </h3>
    //                     <a href="https://github.com/Gearhartlove/the-rust-bucket/discussions/1"
    //                     target="_blank"> { "[3/11/21]What is the rust bucket"} </a>
    //                     <br/>
    //                 </div>
    //             </div>
    //             <div class="column">
    //                 <div class="center">
    //                     <h3> { "Projects" } </h3>
    //                     <a href="https://github.com/Gearhartlove/the-rust-bucket"
    //                     target="_blank"> { "the rust bucket 🌐" } </a>
    //                     <br/>
    //                     <a href="https://github.com/Gearhartlove/tenebrae_cl"
    //                     target="_blank"> { "tenebrae_cl 🎮" } </a>
    //                     <br/>
    //                     <a href="https://github.com/Gearhartlove/firefly"
    //                     target="_blank"> { "firefly 🪰✍️" } </a>
    //                     <br/>
    //                     <a href="https://github.com/Gearhartlove/ezla"
    //                     target="_blank"> { "ezla 🧮" } </a>
    //                     <br/>
    //                 </div>
    //             </div>
    //
    //         </div>
    //     };
    // }
}