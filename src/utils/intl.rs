use chrono::Locale;
use chrono_tz::Tz;
use wasm_bindgen::JsValue;

pub const DEFAULT_LOCALE: &'static str = "ko-KR";
pub const DEFAULT_TZ: Tz = Tz::Asia__Seoul;

#[derive(Debug)]
pub struct IntlOptions {
    pub locale: Locale,
    pub tz: Tz,
}

pub fn resolve_intl() -> IntlOptions {
    let options = js_sys::Intl::DateTimeFormat::new(&js_sys::Array::new(), &js_sys::Object::new())
        .resolved_options();

    let locale_string = js_sys::Reflect::get(&options, &JsValue::from_str("locale"))
        .map_or(None, |v| v.as_string())
        .unwrap_or(DEFAULT_LOCALE.to_owned());
    let tzstring = js_sys::Reflect::get(&options, &JsValue::from_str("timeZone"))
        .map_or(None, |v| v.as_string())
        .unwrap_or(DEFAULT_TZ.to_string());

    let locale = (&locale_string as &str).try_into().unwrap_or(Locale::ko_KR);
    let tz: Tz = tzstring.parse().unwrap_or(DEFAULT_TZ);

    IntlOptions { locale, tz }
}
