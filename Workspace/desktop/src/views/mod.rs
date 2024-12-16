//! The views module contains the components for all Layouts and Routes for our app. Each route in our [`Route`]
//! enum will render one of these components. The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.

mod home;
pub use home::Home;

mod blog;
pub use blog::Blog;