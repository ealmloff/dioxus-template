use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// The Navbar component wraps the children you pass to it with the navbar layout.
/// 
/// # Example
/// ```rust
/// use dioxus::prelude::*;
/// 
/// rsx! {
///     Navbar {
///         // Navbar accepts children which will be rendered inside the navbar layout
///         "This is rendered inside the navbar"
///     }
/// }
/// ```
// This component accepts a special `children` prop that is used to pass the children to the component.
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            // You can wrap raw expressions like the `children` variable in curly braces to render them inside
            // rsx. {children} will render the children passed to the component.
            {children}
        }
    }
}
