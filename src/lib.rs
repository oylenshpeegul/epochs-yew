#![recursion_limit = "1024"]

use chrono::NaiveDateTime;
use epochs;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    int: i64,
    decimal: String,
    hexadecimal: String,

    apfs: String,
    chrome: String,
    cocoa: String,
    google_calendar: String,
    icq: String,
    java: String,
    mozilla: String,
    symbian: String,
    unix: String,
    uuid_v1: String,
    windows_date: String,
    windows_file: String,
}

pub enum Msg {
    GotDecimal(String),
    GotHexadecimal(String),

    GotApfs(String),
    GotChrome(String),
    GotCocoa(String),
    GotGoogleCalendar(String),
    GotIcq(String),
    GotJava(String),
    GotMozilla(String),
    GotSymbian(String),
    GotUnix(String),
    GotUuidV1(String),
    GotWindowsDate(String),
    GotWindowsFile(String),
}

fn reset(m: &mut Model) {
    m.int = 0;
    m.decimal = "".into();
    m.hexadecimal = "".into();

    m.apfs = "".into();
    m.chrome = "".into();
    m.cocoa = "".into();
    m.google_calendar = "".into();
    m.icq = "".into();
    m.java = "".into();
    m.mozilla = "".into();
    m.symbian = "".into();
    m.unix = "".into();
    m.uuid_v1 = "".into();
    m.windows_date = "".into();
    m.windows_file = "".into();
}

fn set(m: &mut Model, int: i64) {
    m.int = int;
    m.decimal = format!("{}", int);
    m.hexadecimal = format!("{:x}", int);
    if let Some(ndt) = epochs::apfs(int) {
        m.apfs = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::chrome(int) {
        m.chrome = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::cocoa(int) {
        m.cocoa = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::google_calendar(int) {
        m.google_calendar = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::icq(int as f64) {
        m.icq = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::java(int) {
        m.java = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::mozilla(int) {
        m.mozilla = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::symbian(int) {
        m.symbian = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::unix(int) {
        m.unix = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::uuid_v1(int) {
        m.uuid_v1 = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::windows_date(int) {
        m.windows_date = format!("{}", ndt);
    }
    if let Some(ndt) = epochs::windows_file(int) {
        m.windows_file = format!("{}", ndt);
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
            chrome: "".into(),
            cocoa: "".into(),
            google_calendar: "".into(),
            icq: "".into(),
            java: "".into(),
            mozilla: "".into(),
            symbian: "".into(),
            unix: "".into(),
            uuid_v1: "".into(),
            windows_date: "".into(),
            windows_file: "".into(),
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
            Msg::GotChrome(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_chrome(ndt));
                };
            }
            Msg::GotCocoa(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_cocoa(ndt));
                };
            }
            Msg::GotGoogleCalendar(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_google_calendar(ndt));
                };
            }
            Msg::GotIcq(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_icq(ndt).round() as i64);
                };
            }
            Msg::GotJava(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_java(ndt));
                };
            }
            Msg::GotMozilla(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_mozilla(ndt));
                };
            }
            Msg::GotSymbian(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_symbian(ndt));
                };
            }
            Msg::GotUnix(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_unix(ndt));
                };
            }
            Msg::GotUuidV1(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_uuid_v1(ndt));
                };
            }
            Msg::GotWindowsDate(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_windows_date(ndt));
                };
            }
            Msg::GotWindowsFile(new_value) => {
                if let Ok(ndt) = NaiveDateTime::parse_from_str(&new_value, "%Y-%m-%d %H:%M:%S") {
                    set(self, epochs::to_windows_file(ndt));
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

                    <div class="grid-item">{"Chrome"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotChrome(e.value))
                      value={ &self.chrome }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.chrome }
                    </textarea>

                    <div class="grid-item">{"Cocoa"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotCocoa(e.value))
                      value={ &self.cocoa }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.cocoa }
                    </textarea>

                    <div class="grid-item">{"Google Calendar"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotGoogleCalendar(e.value))
                      value={ &self.google_calendar }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.google_calendar }
                    </textarea>

                    <div class="grid-item">{"ICQ"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotIcq(e.value))
                      value={ &self.icq }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.icq }
                    </textarea>

                    <div class="grid-item">{"Java"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotJava(e.value))
                      value={ &self.java }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.java }
                    </textarea>

                    <div class="grid-item">{"Mozilla"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotMozilla(e.value))
                      value={ &self.mozilla }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.mozilla }
                    </textarea>

                    <div class="grid-item">{"Symbian"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotSymbian(e.value))
                      value={ &self.symbian }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.symbian }
                    </textarea>

                    <div class="grid-item">{"Unix"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotUnix(e.value))
                      value={ &self.unix }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.unix }
                    </textarea>

                    <div class="grid-item">{"UUID v1"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotUuidV1(e.value))
                      value={ &self.uuid_v1 }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.uuid_v1 }
                    </textarea>

                    <div class="grid-item">{"Windows Date"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotWindowsDate(e.value))
                      value={ &self.windows_date }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.windows_date }
                    </textarea>

                    <div class="grid-item">{"Windows File"}</div>
                    <textarea class="grid-item" rows="1"
                      oninput=self.link.callback(|e: InputData| Msg::GotWindowsFile(e.value))
                      value={ &self.windows_file }
                      placeholder="yyyy-mm-dd HH:MM:SS">
                      { &self.windows_file }
                    </textarea>

                </div>
            </div>
        }
    }
}
