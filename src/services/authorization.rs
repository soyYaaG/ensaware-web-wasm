use leptos::leptos_dom::helpers::location;

use crate::utils::url::{get_url, RequestPath};

pub fn login(provider: String) {
    let url = get_url(RequestPath::Authorization, "v1".to_string());

    let _ = location().set_href(&format!("{}/{}", url, provider));
}
