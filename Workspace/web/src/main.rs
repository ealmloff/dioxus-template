use dioxus::prelude::*;

{% if is_router -%}
use ui::Navbar;
{%- else -%}
{% if is_fullstack -%}
use ui::{Hero, Echo};
{%- else -%}
use ui::Hero;
{%- endif %}
{%- endif %}
{% if is_router -%}
use views::{Blog, Home};
{%- endif %}

{% if is_router %}
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(WebNavbar)]
    // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Blog { id: i32 },
}
{% endif %}

// The asset macro minifies and bundles assets like CSS and JS
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
/// 
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        {% if is_router -%}
        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
        {%- else -%}
        Hero {}
        {% if is_fullstack -%}
        Echo {}
        {%- endif %}
        {%- endif %}
    }
}

{% if is_router -%}
/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
/// 
/// The WebNavbar component that will be rendered on all pages of our app since every page is under the layout.
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            // The `Link` component lets us link to other routes inside our app. It takes a `to` prop of type `Route` and
            // any number of child nodes.
            Link {
                // The `to` prop is the route that the link should navigate to. We can use the `Route` enum to link to the
                // home page. Since we are using an enum instead of a string, all of the routes will be checked
                // at compile time to make sure they are valid.
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
{%- endif %}
