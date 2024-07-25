use leptos::*;
use oak_containers_hello_world_web_client::GreeterClient;
use wasm_bindgen::prelude::*;

#[component]
fn EnclaveGreeter() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (port, set_port) = create_signal(8080u16);
    let (greeting, set_greeting) = create_signal(String::new());

    let greet = create_action(move |(port, name): &(u16, String)| {
        let port = *port;
        let name = name.clone();
        async move {
            match GreeterClient::new(port).await {
                Ok(mut client) => match client.greet(&name).await {
                    Ok(response) => set_greeting.set(response),
                    Err(e) => set_greeting.set(format!("Error: {}", e)),
                },
                Err(e) => set_greeting.set(format!("Error creating client: {}", e)),
            }
        }
    });

    view! {
        <div>
            <h1>"Enclave Greeter"</h1>
            <input
                type="text"
                on:input=move |ev| set_name.set(event_target_value(&ev))
                prop:value=name
                placeholder="Enter your name"
            />
            <input
                type="number"
                on:input=move |ev| set_port.set(event_target_value(&ev).parse().unwrap_or(8080))
                prop:value=port
                placeholder="Enter port number"
            />
            <button
                on:click=move |_| greet.dispatch((port.get(), name.get()))
            >
                "Greet"
            </button>
            <p>{greeting}</p>
        </div>
    }
}

pub fn main() {
    mount_to_body(|| view! { <EnclaveGreeter /> })
}
