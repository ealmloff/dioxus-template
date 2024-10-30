use dioxus::prelude::*;
{% if is_fullstack -%}
use crate::components::{Hero, Calculator};
{%- else -%}
use crate::components::Hero;
{%- endif %}

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        {% if is_fullstack -%}
        Calculator {}
        {%- endif %}
    }
}
