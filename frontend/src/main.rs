use yew::prelude::*;

enum Msg {
    AddOne,
    GetStuff,
}

struct Model {
    text: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            text: "nothing".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.text = "clicked".to_string();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::GetStuff => {
                self.text = "stuff".to_string();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "click" }</button>
                <button onclick={link.callback(|_| Msg::GetStuff)}>{ "stuff" }</button>
                <p>{ self.text.to_ascii_lowercase() }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
