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
                    <h1 style="padding-top: 50px;">{"Hallo, ich bins Kristoff"}</h1>
                    <div style="max-height:500px; max-width:500px;">
                        <img alt="Kristoff Portrait." src="assets/kristoff_home.jpg" class="portrait" />
                    </div>
                    <p style="font-size: 19px;">{"at vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellattest"}</p>
                </div>
            </div>
        };
        html
    }
}
