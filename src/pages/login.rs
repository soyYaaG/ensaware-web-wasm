use icondata::{LuFormInput, LuLogIn, TbFaceIdError};
use leptos::*;
use leptos_icons::Icon;
use leptos_router::*;

use crate::{
    components::alert::{Alert, AlertType, AlertTypeProps},
    services::authorization::login,
};

#[derive(Params, PartialEq)]
struct LoginQuery {
    error: String,
}

#[component]
pub fn Login() -> impl IntoView {
    let on_click = move |_| {
        login("google".to_string());
    };

    let alert = AlertTypeProps {
        alert_type: AlertType::Error,
        icon: TbFaceIdError,
        message: "Error al iniciar sesión".to_string(),
    };

    let query = use_query::<LoginQuery>();
    let error = move || {
        query.with(|params| {
            params
                .as_ref()
                .map(|params| params.error.clone())
                .unwrap_or_default()
        })
    };

    view! {
        <section class="h-screen flex flex-row">
            <aside class="hidden md:flex w-1/2">
            <img
                alt="Un móvil con un código QR sostenido por una persona en su mano."
                class="object-cover w-full;"
                src="assets/img/login-ensaware.jpg"
            />
            </aside>

            <section class="flex flex-col items-center justify-center p-4 w-full md:w-1/2">
                <h1 class="text-5xl mb-10">"Ensaware"</h1>

                <div class="flex flex-row mb-10 text-3xl text-gray-700 items-center">
                    <Icon icon=LuFormInput class="h-8 w-8 mr-4" />
                    <h2>"Iniciar Sesión"</h2>
                </div>


                <Show
                    when=move || (error().len() > 0)
                    fallback=|| leptos::View::default()
                >
                    <div class="my-4">
                        <Alert alert_props={alert.clone()} />
                    </div>
                </Show>

                <img
                    alt="Logo de la Corporación Universitaria Americana	."
                    class="mb-10 h-96 w-96"
                    src="assets/img/logo-americana.png"
                />

                <p class="mb-10">
                    "La aplicación Ensaware solo permite iniciar sesión con el correo electrónico institucional."
                </p>

                <button
                    class="btn btn-secondary p-4 w-full md:w-1/2"
                    on:click=on_click
                >
                    <Icon icon=LuLogIn class="h-4 w-4" />
                    "Iniciar Sesión"
                </button>
            </section>
        </section>
    }
}
