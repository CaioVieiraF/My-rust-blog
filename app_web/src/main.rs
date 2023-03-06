use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures;

#[derive(PartialEq, Properties)]
struct PostProps {
    title: String,
    body: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Post {
    id: i32,
    title: String,
    body: String,
    published: u8,
}

#[function_component(PostThread)]
fn post_hread(props: &PostProps) -> Html {
    let body = props.body.clone();
    let title = props.title.clone();
    println!("{body} {title}");
    html! {
        <div>
            <h3>{ title }</h3>
            <p>{ body }</p>
        </div>
    }
}

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

    println!("{:?}", posts);

    html! {
        <div>
            {
            (*posts).clone().into_iter().map(|post| {
                let props = PostProps { title: post.title, body: post.body };

                html! {<PostThread ..props />}
            }).collect::<Html>()
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
