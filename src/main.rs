use korean_lunar_calendar::{lunar_to_gregorian, LunarDate};
use wasm_logger;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let date = use_state(|| "".to_owned());
    let is_leap = use_state(|| false);
    let res = use_state(|| "".to_owned());

    let handle_input_date = {
        let date = date.clone();
        move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            date.set(target.value());
        }
    };
    let handle_input_is_leap = {
        let is_leap = is_leap.clone();
        move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            is_leap.set(target.checked());
        }
    };
    let onclick = {
        let date = date.clone();
        let is_leap = is_leap.clone();
        let res = res.clone();
        Callback::from(move |_| {
            let parts = (*date)
                .split("-")
                .map(str::parse::<u32>)
                .collect::<Vec<_>>();
            let (year, month, day) = match (parts.get(0), parts.get(1), parts.get(2)) {
                (Some(Ok(y)), Some(Ok(m)), Some(Ok(d))) => (y, m, d),
                _ => {
                    return;
                }
            };
            let gregorian = lunar_to_gregorian(&LunarDate {
                year: *year as i32,
                month: (*month as u8, *is_leap),
                day: *day as u8,
            });
            let gregorian = match gregorian {
                Some(gregorian) => gregorian,
                None => {
                    return;
                }
            };
            res.set(gregorian.to_string());
        })
    };

    html! {
        <div>
            <div>
                <input
                    type="text"
                    placeholder="yyyy-mm-dd"
                     value={(*date).clone()}
                    oninput={handle_input_date}
                />
                <input
                    type="checkbox"
                    id="is-leap"
                    value={if *is_leap { "checked" } else { "" }}
                    oninput={handle_input_is_leap}
                />
                <label for="is-leap">{ "윤달" }</label>
            </div>
            <button onclick={onclick}>{ "변환" }</button>
            <p>{ (*res).clone() }</p>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
