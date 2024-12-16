// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

{% if is_router %}
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
    #[layout(Navbar)]
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

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
{% if is_tailwind -%}
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
{%- endif %}

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it wit the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
/// 
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } {% if is_tailwind -%}
        document::Link { rel: "stylesheet", href: TAILWIND_CSS } {%- endif %}
        {% if is_router -%}
        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
        {%- else -%} Hero {}
        {%- endif %} {% if is_fullstack -%}
        Echo {}{%- endif %}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                // The RSX macro also supports text nodes surrounded by quotes
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

{% if is_router -%}
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        {% if is_fullstack -%} Echo {} {%- endif %}
    }
}

/// The Blog page component that will be rendered when the current route is `[Route::Blog]`
/// 
/// The component takes a `id` prop of type `i32` from the route enum. Whenever the id changes, the component function will be
/// re-run and the rendered HTML will be updated.
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            // The `Link` component lets us link to other routes inside our app. It takes a `to` prop of type `Route` and
            // any number of child nodes.
            Link {
                // The `to` prop is the route that the link should navigate to. We can use the `Route` enum to link to the
                // blog page with the id of -1. Since we are using an enum instead of a string, all of the routes will be checked
                // at compile time to make sure they are valid.
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
/// 
/// 
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
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

{% if is_fullstack -%}
/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    // 
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                // `oninput` is an event handler that will run when the input changes. It can return either nothing or a future
                // that will be run when the event runs.
                oninput:  move |event| async move {
                    // When we call the echo_server function from the client, it will fire a request to the server and return
                    // the response. It handles serialization and deserialization of the request and response for us.
                    let data = echo_server(event.value()).await.unwrap();

                    // After we have the data from the server, we can set the state of the signal to the new value.
                    // Since we read the `response` signal later in this component, the component will rerun.
                    response.set(data);
                },
            }

            // Signals can be called like a function to clone the current value of the signal
            if !response().is_empty() {
                p {
                    "Server echoed: "
                    // Since we read the signal inside this component, the component "subscribes" to the signal. Whenever
                    // the signal changes, the component will rerun.
                    i { "{response}" }
                }
            }
        }
    }
}

// Server functions let us define public APIs on the server that can be called like a normal async function from the client.
// Each server function needs to be annotated with the `#[server]` attribute, accept and return serializable types, and return
// a `Result` with the error type [`ServerFnError`].
//
// When the server function is called from the client, it will just serialize the arguments, call the API, and deserialize the
// response.
#[server]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    // The body of server function like this comment are only included on the server. If you have any server-only logic like
    // database queries, you can put it here. Any imports for the server function should either be imported inside the function
    // or imported under a `#[cfg(feature = "server")]` block.
    Ok(input)
}
{%- endif %}
