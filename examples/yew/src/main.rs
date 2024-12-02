use nano_ai::client::NanoAI;
use nano_ai::types::Capabilities;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="app-container">
            <header class="app-header">
                <img src="https://github.com/user-attachments/assets/87956e6c-9c9f-428a-8bb6-0b6221b8f6a6" alt="NanoAI Logo" class="app-logo" />
            </header>
            <div class="content">
                <SessionStatsComponent />
                <ChatComponent />
            </div>
        </div>
    }
}

#[function_component]
pub fn SessionStatsComponent() -> Html {
    let session_created = use_state(|| false);
    let session_error = use_state(|| None::<String>);
    let session_stats = use_state(|| None::<Capabilities>);

    let on_fetch_session_stats = {
        let session_created = session_created.clone();
        let session_error = session_error.clone();
        let session_stats = session_stats.clone();

        Callback::from(move |_: MouseEvent| {
            let session_created = session_created.clone();
            let session_error = session_error.clone();
            let session_stats = session_stats.clone();

            spawn_local(async move {
                let mut client = NanoAI::new();
                match client.create_session(None).await {
                    Ok(_) => {
                        session_created.set(true);
                        session_error.set(None);

                        match client.get_capabilities().await {
                            Ok(stats) => session_stats.set(Some(stats)),
                            Err(err) => {
                                session_error.set(Some(format!("Failed to fetch stats: {}", err)))
                            }
                        }
                    }
                    Err(err) => {
                        session_error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let stats_display = if let Some(stats) = &*session_stats {
        html! {
            <div class="stats-container">
                <h3>{ "Stats 📊" }</h3>
                <p>{ format!("Available: {}", stats.available) }</p>
                <p>{ format!("Default Temperature: {}", stats.default_temperature) }</p>
                <p>{ format!("Default Top-K: {}", stats.default_top_k) }</p>
                <p>{ format!("Max Top-K: {}", stats.max_top_k) }</p>
            </div>
        }
    } else {
        html! {}
    };

    let session_status = if *session_created {
        html! { <p class="status-success">{ "✅ Session Ready!" }</p> }
    } else if let Some(error) = &*session_error {
        html! { <p class="status-error">{ error }</p> }
    } else {
        html! {}
    };

    html! {
        <div class="session-stats">
            <button onclick={on_fetch_session_stats} class="fetch-stats-btn">{ "⚙️ Create Session" }</button>
            { session_status }
            { stats_display }
        </div>
    }
}

#[function_component]
pub fn ChatComponent() -> Html {
    let chat_history = use_state(Vec::new);
    let session_error = use_state(|| None::<String>);
    let input_text_ref = use_node_ref();
    let input_text = use_state(String::new);

    let on_input_change = {
        let input_text_ref = input_text_ref.clone();
        let input_text = input_text.clone();
        Callback::from(move |_| {
            if let Some(input) = input_text_ref.cast::<HtmlInputElement>() {
                input_text.set(input.value());
            }
        })
    };

    let on_send_prompt = {
        let session_error = session_error.clone();
        let input_text = input_text.clone();
        let chat_history = chat_history.clone();

        Callback::from(move |evt: SubmitEvent| {
            evt.prevent_default();
            if input_text.is_empty() {
                return;
            }

            let chat_history = chat_history.clone();
            let session_error = session_error.clone();
            let prompt_text = (*input_text).clone();

            chat_history.set({
                let mut history = (*chat_history).clone();
                history.push((true, prompt_text.clone()));
                history
            });

            input_text.set(String::new());

            spawn_local(async move {
                let chat_history = chat_history.clone();
                let mut client = NanoAI::new();
                match client.create_session(None).await {
                    Ok(_) => match client.send_prompt(&prompt_text).await {
                        Ok(response) => {
                            chat_history.set({
                                let mut history = (*chat_history).clone();
                                history.push((false, response));
                                history
                            });
                            session_error.set(None);
                        }
                        Err(err) => session_error.set(Some(format!("❌ Prompt failed: {}", err))),
                    },
                    Err(err) => {
                        session_error.set(Some(err.to_string()));
                    }
                }
            });
        })
    };

    let chat_display = html! {
        <div class="chat-history">
            { for (*chat_history).iter().map(|(is_user, message)| html! {
                if *is_user {
                    <div class="chat-bubble user">{ message }</div>
                } else {
                    <div class="chat-bubble ai">{ message }</div>
                }
            })}
        </div>
    };

    html! {
        <div class="chat-container">
            { chat_display }
            <form class="chat-input-wrapper" onsubmit={on_send_prompt} >
                <input
                    type="text"
                    placeholder="💬 Type your message..."
                    ref={input_text_ref}
                    value={(*input_text).clone()}
                    oninput={on_input_change}
                    class="chat-input"
                />
                <button class="send-btn">{ "➤" }</button>
            </form>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
