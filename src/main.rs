use chrono::{NaiveDate, NaiveTime, Utc};
use ical::{
    generator::{Emitter, IcalCalendar, IcalCalendarBuilder, IcalEventBuilder},
    ical_property,
    property::Property,
};
use itertools::Itertools;
use korean_lunar_calendar::{lunar_to_gregorian, LunarDate};
use uuid::Uuid;
use wasm_logger;
use yew::prelude::*;

mod components;
use components::{Form, Layout, Preview};

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

fn build_ics<'a, I>(title: &str, dates: I) -> IcalCalendar
where
    I: Iterator<Item = &'a NaiveDate>,
{
    let intl = resolve_intl();
    let utc_now = Utc::now();

    let mut cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//lunar-ical//heek.kr//")
        .build();
    cal.events.extend(dates.map(|d| {
        let uuid = Uuid::new_v4();
        let date_string = d
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .format("%Y%m%d")
            .to_string();
        IcalEventBuilder::tzid(intl.tz.to_string())
            .uid(uuid.to_string())
            .changed(utc_now.format("%Y%m%dT%H%M%SZ").to_string())
            .one_day(date_string)
            .set(ical_property!("SUMMARY", title))
            .build()
    }));
    cal
}

#[function_component]
fn App() -> Html {
    let title = use_state(|| AttrValue::from(""));
    let date = use_state(|| AttrValue::from(""));
    let is_leap = use_state(|| false);

    let handle_change_title = use_callback(
        move |v, state| {
            state.set(v);
        },
        title.clone(),
    );
    let handle_change_date = use_callback(
        move |v, state| {
            state.set(v);
        },
        date.clone(),
    );
    let handle_change_is_leap = use_callback(
        move |v, state| {
            state.set(v);
        },
        is_leap.clone(),
    );

    let gregorian_dates = use_memo(
        |(date, leap)| convert_and_repeat(date, *leap, 100).map_or(vec![], |r| r.collect_vec()),
        ((*date).clone(), *is_leap),
    );

    let handle_click_download = {
        let title = title.clone();
        use_callback(
            move |(), gregorian_dates| {
                let cal = build_ics(&*title, gregorian_dates.iter());
                download_string_blob(&cal.generate(), "text/calendar");
            },
            gregorian_dates.clone(),
        )
    };

    html! {
        <Layout>
            <Form
                title={(*title).clone()}
                date={(*date).clone()}
                is_leap={*is_leap}
                is_download_disabled={gregorian_dates.is_empty()}
                on_change_title={handle_change_title}
                on_change_date={handle_change_date}
                on_change_is_leap={handle_change_is_leap}
                on_click_download={handle_click_download}
            />
            <Preview dates={(*gregorian_dates).clone()} />
        </Layout>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
