use yew::prelude::*;
use lib_tft_set5_wasm as tft;

mod unit;

struct Model {
    link: ComponentLink<Self>,
    set: tft::Set5,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { set: tft::Set5::new(), link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
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
                { for self.set.units.iter().map(|u| Unit::render(&u)) }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}