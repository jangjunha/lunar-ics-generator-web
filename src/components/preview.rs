use chrono::NaiveDate;
use yew::prelude::*;

use crate::utils::resolve_intl;

#[derive(Properties, PartialEq)]
pub struct PreviewProps {
    #[prop_or_default]
    pub dates: Vec<NaiveDate>,
}

#[function_component]
pub fn Preview(props: &PreviewProps) -> Html {
    html! {
        <section class="sm:flex-1">
            <h4 class="mt-1">{ "미리보기" }</h4>
            if props.dates.is_empty() {
                <p class="text-sm text-gray-300">{ "음력 날짜를 입력하면 양력 반복 날짜가 표시됩니다." }</p>
            } else {
                <ul>
                    { props.dates.iter().map(|d| html! {
                        <li class="my-0.5 font-mono text-sm">
                            { d.format_localized("%x", resolve_intl().locale) }
                        </li>
                    }).collect::<Html>() }
                </ul>
            }
        </section>
    }
}
