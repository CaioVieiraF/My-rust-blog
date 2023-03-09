use yew::prelude::*;
use serde::{Deserialize, Serialize};
use crate::Msg;

#[derive(PartialEq, Properties, Debug)]
pub struct PostProps {
    title: String,
    body: String,
    published: i32,
}

impl PostProps {
    pub fn new(title: String, body: String, published: i32) -> PostProps {
        PostProps {title, body, published}
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    title: String,
    body: String,
    published: i32,
}

impl Post {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn published(&self) -> i32 {
        self.published.clone()
    }
}

impl Component for Post {
    type Message = Msg;
    type Properties = PostProps;

    fn create(ctx: &Context<Self>) -> Self {
        let title = &ctx.props().title;
        let body = &ctx.props().body;
        let published = &ctx.props().published;

        Self {
            title: title.clone(),
            body: body.clone(),
            published: published.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="p-5 m-4 shadow bg-dark-subtle rounded">
                <h2>{self.title.clone()}</h2>
                <p>{self.body.clone()}</p>
            </div>
        }
    }
}
