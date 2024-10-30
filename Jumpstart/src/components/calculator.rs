use dioxus::prelude::*;
use dioxus_logger::tracing::info;

/// Calculator component that calls to our fullstack server to perform calculations.
#[component]
pub fn Calculator() -> Element {
    let mut first_number = use_signal(|| 1);
    let mut second_number = use_signal(|| 2);
    let mut result: Signal<Option<i32>> = use_signal(|| None);

    rsx! {
        div {
            id: "calculator",

            // Inputs
            label { "First Number: "}
            input {
                r#type: "number",
                value: first_number,
                oninput: move |event| first_number.set(event.value().parse().expect("input should only be a number")),
            }
            br {}
            label { "Second Number: "}
            input {
                r#type: "number",
                value: second_number,
                oninput: move |event| second_number.set(event.value().parse().expect("input should only be a number")),
            }
            br {}

            // Submit button
            button {
                onclick: move |_| async move {
                    if let Ok(data) = add_numbers(first_number(), second_number()).await {
                        info!("Client received calculated number: {}", data);
                        result.set(Some(data));
                    }
                },
                "Add Numbers"
            }

            // Result
            if let Some(result) = result() {
                p { "Result: {result}" }
            }
        }
    }
}

/// Add two numbers together on the server.
#[server(AddNumbers)]
async fn add_numbers(first: i32, last: i32) -> Result<i32, ServerFnError> {
    info!("Server is calculating `{} + {}`", first, last);
    Ok(first + last)
}