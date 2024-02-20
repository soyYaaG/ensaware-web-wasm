use leptos::{logging::log, *};
use leptos_router::*;

#[component]
pub fn Token() -> impl IntoView {
    let query = use_query_map();
    let get_param = move |key: &str| query.with(|params| params.get(key).cloned());

    // log!("{}", get_param("project").unwrap_or_default().clone());
    const INITIAL_COLOR: &str = "#ff0000";
    let (color, set_color) = create_signal(INITIAL_COLOR.to_string());

    #[cfg(not(feature = "ssr"))]
    let starting_color = if let Ok(Some(storage)) = window().local_storage() {
        let start_color = storage
            .get_item("color")
            .unwrap_or(Some(INITIAL_COLOR.to_string()));
        log!("starting color is {:?}", start_color);
        start_color
    } else {
        Some(INITIAL_COLOR.to_string())
    };

    #[cfg(not(feature = "ssr"))]
    if let Some(color) = starting_color {
        log!("setting with starting color to {}", color);
        set_color(color);
    }

    view! {
        <section>
            <p>"Crear token"</p>
            <p>{move || get_param("project")}</p>
            <p>{move || get_param("token")}</p>
            <p>{move || get_param("token_type")}</p>
            <p>{move || get_param("refresh_token")}</p>
            <p>{color}</p>
        </section>
    }
}
