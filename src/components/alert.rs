use leptos::*;
use leptos_icons::Icon;

#[derive(Clone)]
pub enum AlertType {
    Error,
    Info,
    Warning,
}

#[derive(Clone)]
pub struct AlertTypeProps {
    pub alert_type: AlertType,
    pub icon: icondata::Icon,
    pub message: String,
}

#[component]
pub fn Alert(alert_props: AlertTypeProps) -> impl IntoView {
    let alert_type = match alert_props.alert_type {
        AlertType::Error => "alert-error",
        AlertType::Info => "alert-info",
        AlertType::Warning => "alert-warning",
    };

    view! {
        <div role="alert" class=format!("alert {}", alert_type)>
            <Icon icon=alert_props.icon class="h-8 w-8" />
            <span>{alert_props.message}</span>
        </div>
    }
}
