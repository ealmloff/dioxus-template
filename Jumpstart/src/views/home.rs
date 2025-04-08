use dioxus::prelude::*;
{% if is_fullstack -%}
use crate::components::{Hero, Echo};
{%- else -%}
use crate::components::Hero;
{%- endif %}

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        {% if is_fullstack -%}
        Echo {}
        {%- endif %}
    }
}
