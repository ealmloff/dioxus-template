mod hero;
pub use hero::Hero;

{% if is_router %}
mod navbar;
pub use navbar::Navbar;
{% endif %}

{% if is_fullstack %}
mod calculator;
pub use calculator::Calculator;
{% endif %}

