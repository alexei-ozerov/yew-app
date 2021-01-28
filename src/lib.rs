#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    title: String,
    value: i64,
}

enum Msg {
    AddOne,
    AddSix,
    Reset,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            title: "File Load Counter".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::AddSix => self.value += 6,
            Msg::Reset => self.value = 0,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ &self.title }</h1>
                <table class="attr">
                    <tr>
                        <td><button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add One" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::AddSix)>{ "Add Six" }</button></td>
                        <td><button onclick=self.link.callback(|_| Msg::Reset)>{ "Reset" }</button></td>
                    </tr>
                </table>
                <p class="count">{ self.value }</p>
            </div>
        }
    }
}

// Mount Application To Body of index.html
#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
