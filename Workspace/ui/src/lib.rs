//! This crate contains all shared UI for the workspace.

mod hero;
pub use hero::Hero;

{% if is_fullstack -%}
mod calculator;
pub use calculator::Calculator;
{%- endif %}
