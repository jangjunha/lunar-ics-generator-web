use chrono::{NaiveDate, NaiveTime, Utc};
use ical::{
    generator::{Emitter, IcalCalendarBuilder, IcalEventBuilder},
    ical_property,
    property::Property,
};
use itertools::Itertools;
use korean_lunar_calendar::{lunar_to_gregorian, LunarDate};
use uuid::Uuid;
use wasm_logger;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod utils;
use utils::{download_string_blob, resolve_intl};

fn convert_and_repeat(
    lunar_date_string: &str,
    is_leap: bool,
    limit: usize,
) -> Option<impl Iterator<Item = NaiveDate>> {
    let parts = lunar_date_string
        .split(".")
        .map(str::parse::<u32>)
        .collect::<Vec<_>>();
    let (year, month, day) = match (parts.get(0), parts.get(1), parts.get(2)) {
        (Some(Ok(y)), Some(Ok(m)), Some(Ok(d))) => (y, m, d),
        _ => {
            log::warn!("Cannot parse lunar date");
            return None;
        }
    };
    let lunar_base = LunarDate {
        year: *year as i32,
        month: (*month as u8, is_leap),
        day: *day as u8,
    };
    Some(
        (0..(100 * limit as i32))
            .filter_map(move |i| {
                lunar_to_gregorian(&LunarDate {
                    year: lunar_base.year + i,
                    ..lunar_base
                })
            })
            .take(limit),
    )
}

#[function_component]
fn App() -> Html {
    let title = use_state(|| "".to_owned());
    let date = use_state(|| "".to_owned());
    let is_leap = use_state(|| false);

    let handle_input_title = {
        let title = title.clone();
        move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            title.set(target.value());
        }
    };
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

    let gregorian_dates = use_memo(
        |(date, leap)| convert_and_repeat(date, *leap, 100).map_or(vec![], |r| r.collect_vec()),
        ((*date).clone(), *is_leap),
    );

    let onclick = {
        let title = title.clone();
        use_callback(
            move |_, gregorian_dates| {
                let intl = resolve_intl();
                let utc_now = Utc::now();

                let cal = {
                    let mut cal = IcalCalendarBuilder::version("2.0")
                        .gregorian()
                        .prodid("-//lunar-ical//heek.kr//")
                        .build();
                    cal.events.extend(gregorian_dates.iter().map(|d| {
                        let uuid = Uuid::new_v4();
                        let date_string = d
                            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                            .format("%Y%m%d")
                            .to_string();
                        IcalEventBuilder::tzid(intl.tz.to_string())
                            .uid(uuid.to_string())
                            .changed(utc_now.format("%Y%m%dT%H%M%SZ").to_string())
                            .one_day(date_string)
                            .set(ical_property!("SUMMARY", &*title))
                            .build()
                    }));
                    cal
                };
                download_string_blob(&cal.generate(), "text/calendar");
            },
            gregorian_dates.clone(),
        )
    };

    html! {
        <div class="prose mx-4 sm:mx-auto my-16">
            <h1>{ "음력 기념일 ICS 생성기" }</h1>
            <section class="flex flex-col sm:flex-row gap-8">
                <form class="sm:flex-1 flex flex-col gap-4">
                    <div>
                        <label>
                            <span class="text-gray-700">{ "제목" }</span>
                            <input
                                type="text"
                                placeholder="기념일"
                                value={(*title).clone()}
                                oninput={handle_input_title}
                                class="mt-1 block w-full rounded-md bg-gray-100 placeholder-gray-300 border-transparent focus:border-gray-500 focus:bg-white focus:ring-0"
                            />
                        </label>
                    </div>
                    <div>
                        <label>
                            <span class="text-gray-700">{ "음력 시작일" }<span class="text-red-600 ml-1">{ "*" }</span></span>
                            <div class="mt-1 relative">
                                <label class="absolute -top-[1.8rem] right-0 flex items-center justify-end gap-2">
                                    <span class="text-gray-700">{ "윤달" }</span>
                                    <input
                                        type="checkbox"
                                        value={if *is_leap { "checked" } else { "" }}
                                        oninput={handle_input_is_leap}
                                        class="rounded bg-gray-200 border-transparent focus:border-transparent focus:bg-gray-200 text-gray-700 focus:ring-1 focus:ring-offset-2 focus:ring-gray-500"
                                    />
                                </label>
                                <input
                                    type="text"
                                    placeholder="1993.03.25"
                                    value={(*date).clone()}
                                    oninput={handle_input_date}
                                    required={true}
                                    class="block w-full rounded-md bg-gray-100 placeholder-gray-300 border-transparent focus:border-gray-500 focus:bg-white focus:ring-0"
                                />
                            </div>
                        </label>
                    </div>
                    <button
                        onclick={onclick}
                        disabled={gregorian_dates.is_empty()}
                        class="rounded px-3 py-2 bg-blue-400 disabled:bg-gray-200 text-white font-semibold"
                    >{ "다운로드" }</button>
                    <section>
                        <h4>{ "알림" }</h4>
                        <ul class="[&>*]:my-1">
                            <li>{ "반복이 아닌 개별 일정으로 생성됩니다." }</li>
                            <li>{ "2050년 11월 30일까지 생성됩니다." }</li>
                        </ul>
                    </section>
                </form>
                <section class="sm:flex-1">
                    <h4 class="mt-1">{ "미리보기" }</h4>
                    if gregorian_dates.is_empty() {
                        <p class="text-sm text-gray-300">{ "음력 날짜를 입력하면 양력 반복 날짜가 표시됩니다." }</p>
                    } else {
                        <ul>
                            { gregorian_dates.iter().map(|d| html! {
                                <li class="my-0.5 font-mono text-sm">{ d.format_localized("%x", resolve_intl().locale) }</li>
                            }).collect::<Html>() }
                        </ul>
                    }
                </section>
            </section>
            <footer>
                <ul class="list-none px-0 flex justify-center gap-4">
                    <li>{ "© jangjunha" }</li>
                    <li>
                        <a
                            href="https://github.com/jangjunha/lunar-ics-generator-web"
                            target="_blank"
                            class="text-blue-500"
                        >{ "GitHub에서 보기" }</a>
                    </li>
                </ul>
            </footer>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
