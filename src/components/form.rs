use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub title: String,
    pub date: String,
    pub is_leap: bool,

    #[prop_or_default]
    pub is_download_disabled: bool,

    #[prop_or_default]
    pub on_change_title: Callback<String>,
    #[prop_or_default]
    pub on_change_date: Callback<String>,
    #[prop_or_default]
    pub on_change_is_leap: Callback<bool>,
    #[prop_or_default]
    pub on_click_download: Callback<()>,
}

#[function_component]
pub fn Form(props: &FormProps) -> Html {
    let handle_input_title = {
        let on_change_title = props.on_change_title.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            on_change_title.emit(target.value());
        })
    };

    let handle_input_date = {
        let on_change_date = props.on_change_date.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            on_change_date.emit(target.value());
        })
    };

    let handle_input_is_leap = {
        let on_change_is_leap = props.on_change_is_leap.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            on_change_is_leap.emit(target.checked());
        })
    };

    let handle_click_download = {
        let on_click_download = props.on_click_download.clone();
        Callback::from(move |_| {
            on_click_download.emit(());
        })
    };

    html! {
        <form class="sm:flex-1 flex flex-col gap-4">
            <div>
                <label>
                    <span class="text-gray-700">{ "제목" }</span>
                    <input
                        type="text"
                        placeholder="기념일"
                        value={props.title.clone()}
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
                                value={if props.is_leap { "checked" } else { "" }}
                                oninput={handle_input_is_leap}
                                class="rounded bg-gray-200 border-transparent focus:border-transparent focus:bg-gray-200 text-gray-700 focus:ring-1 focus:ring-offset-2 focus:ring-gray-500"
                            />
                        </label>
                        <input
                            type="text"
                            placeholder="1993.03.25"
                            value={props.date.clone()}
                            oninput={handle_input_date}
                            required={true}
                            class="block w-full rounded-md bg-gray-100 placeholder-gray-300 border-transparent focus:border-gray-500 focus:bg-white focus:ring-0"
                        />
                    </div>
                </label>
            </div>
            <button
                onclick={handle_click_download}
                disabled={props.is_download_disabled}
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
    }
}
