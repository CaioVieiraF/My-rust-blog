use yew::prelude::*;
use crate::Msg;

#[derive(PartialEq)]
pub struct PageInfo {
    header: String,
    content: String,
}

impl Component for PageInfo {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        PageInfo { header: String::from("Name"), content: String::from("info") }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="col bg-dark-subtle shadow rounded m-3 p-3">
                <h5>{self.header.clone()}</h5>
                <p>{self.content.clone()}</p>
            </div>
        }
    }
}


