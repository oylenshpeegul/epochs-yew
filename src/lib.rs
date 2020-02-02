#![recursion_limit = "512"]

use chrono::NaiveDateTime;
use epochs;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    int: i64,
    decimal: String,
    hexadecimal: String,

    apfs: String,
    unix: String,
}

pub enum Msg {
    GotDecimal(String),
    GotHexadecimal(String),
    GotApfs(String),
    GotUnix(String),
}

fn reset(m: &mut Model) {
    m.int = 0;
    m.decimal = "".into();
    m.hexadecimal = "".into();

    m.apfs = "".into();
    m.unix = "".into();
}

fn set(m: &mut Model, int: i64) {
    m.int = int;
    m.decimal = format!("{}", int);
    m.hexadecimal = format!("{:x}", int);
    if let Some(ndt) = epochs::apfs(int) {
        m.apfs = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::unix(int) {
        m.unix = format!("{}", ndt);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            int: 0,
            decimal: "".into(),
            hexadecimal: "".into(),

            apfs: "".into(),
            unix: "".into(),
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
            }
            Msg::GotHexadecimal(new_value) => {
                if let Ok(int) = i64::from_str_radix(&new_value, 16) {
                    set(self, int);
                } else {
                    reset(self);
                };
            }
            Msg::GotApfs(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_apfs(ndt));
                };
            }
            Msg::GotUnix(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_unix(ndt));
                };
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"epochs"}</h1>
                <div class="grid-container">

                    <div class="grid-item">{"Decimal"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotDecimal(e.value))
                      value={ &self.decimal }
                      placeholder="decimal digits">
                      { &self.decimal }
                    </textarea>

                    <div class="grid-item">{"Hexadecimal"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotHexadecimal(e.value))
                      value={ &self.hexadecimal }
                      placeholder="hex digits">
                      { &self.hexadecimal }
                    </textarea>

                    <div class="grid-item">{"APFS"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotApfs(e.value))
                      value={ &self.apfs }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.apfs }
                    </textarea>

                    <div class="grid-item">{"Unix"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotUnix(e.value))
                      value={ &self.unix }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.unix }
                    </textarea>

                </div>
            </div>
        }
    }
}
