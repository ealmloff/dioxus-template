use dioxus::prelude::*;
{% if is_fullstack -%}
use ui::{Hero, Calculator};
{%- else -%}
use ui::Hero;
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
