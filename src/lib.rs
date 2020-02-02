#![recursion_limit = "256"]

use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    int_value: i64,
    decimal_str: String,
    hexadecimal_str: String,
}

pub enum Msg {
    GotDecimal(String),
    GotHexadecimal(String),
}

fn reset(m: &mut Model) {
    m.int_value = 0;
    m.decimal_str = "".into();
    m.hexadecimal_str = "".into();
}
fn set(m: &mut Model, int: i64) {
    m.int_value = int;
    m.decimal_str = format!("{}", int);
    m.hexadecimal_str = format!("{:x}", int);
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            int_value: 0,
            decimal_str: "".into(),
            hexadecimal_str: "".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotDecimal(new_value) => {
                if let Ok(int) = new_value.parse::<i64>() {
                    set(self, int);
                } else {
                    reset(self);
                };
            },
            Msg::GotHexadecimal(new_value) => {
                if let Ok(int) = i64::from_str_radix(&new_value, 16) {
                    set(self, int);
                } else {
                    reset(self);
                };
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="grid-container">
                <div class="grid-item">{"Decimal"}</div>
                <textarea class="grid-item" rows="1"
                oninput=self.link.callback(|e: InputData| Msg::GotDecimal(e.value))
                value={ &self.decimal_str }
                placeholder="decimal digits">
                { &self.decimal_str }
                </textarea>

                <div class="grid-item">{"Hexadecimal"}</div>
                <textarea class="grid-item" rows="1"
                oninput=self.link.callback(|e: InputData| Msg::GotHexadecimal(e.value))
                value={ &self.hexadecimal_str }
                placeholder="hex digits">
                { &self.hexadecimal_str }
                </textarea>
            </div>
        }
    }
}
