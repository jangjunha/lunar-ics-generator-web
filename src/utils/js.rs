use wasm_bindgen::JsValue;
use web_sys::{window, Blob, BlobPropertyBag, Url};

pub fn download_string_blob(content: &str, mime: &str) {
    let content = JsValue::from_str(content);
    let options = {
        let mut options: BlobPropertyBag = BlobPropertyBag::new();
        options.type_(mime);
        options
    };
    let blob = Blob::new_with_str_sequence_and_options(
        &js_sys::Array::from_iter(std::iter::once(content)),
        &options,
    )
    .unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let _ = window()
        .expect("window is undefined")
        .open_with_url_and_target(&url, "_blank");
}
