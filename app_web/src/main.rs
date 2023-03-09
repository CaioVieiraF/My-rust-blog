mod components;

use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures;
use components::{Post, PostProps, PageInfo};

pub enum Msg {}

#[function_component(App)]
fn app() -> Html {

    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let req: Vec<Post> = Request::get("/posts/").send().await.unwrap().json().await.unwrap();
                posts.set(req);
            });
            || ()
        }, ());
    }

    html! {
        <main class="container">
            <div class="row">
                <div class="col-9">
                    {
                    (*posts).clone().into_iter().filter(|p| p.published() == 1).map(|post| {


                        let props = PostProps::new(post.title(), post.body(), post.published());

                        html! {<Post ..props />}
                    }).collect::<Html>()
                    }
                </div>

                <PageInfo />
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
