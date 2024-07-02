use color_eyre::eyre::eyre;
use leptos::{component, create_resource, view, web_sys, IntoView, Suspense};

#[component]
pub fn Main() -> impl IntoView {
    let message = create_resource(
        || (),
        |()| async { fetch_message().await.map_err(|e| e.to_string()) },
    );

    view! {
        <div>
            <h1>{ "Hello, World!" }</h1>
        </div>

        <Suspense fallback=move || view!{ <p> "Loading..." </p> }>
            { move || match message()
                       .unwrap_or_else(|| Ok("Loading ...".to_string())) {
                Ok(text) => view!{ <p>{ text }</p> },
                Err(e) => view!{ <p style:color="red" >{ e }</p> },
            }}
        </Suspense>
    }
}

/// Get the path for an endpoint.
fn get_path_for(endpoint: &str) -> color_eyre::Result<String> {
    let window = web_sys::window().ok_or(eyre!("cannot retrieve window object"))?;

    let host = window
        .location()
        .origin()
        .map_err(|_| eyre!("cannot get origin"))?;

    Ok(format!("{}/{}", host, endpoint.trim_start_matches('/')))
}

/// Fetch a message from the backend.
async fn fetch_message() -> color_eyre::Result<String> {
    let response = reqwest::get(get_path_for("/api/message")?).await?;

    if response.status() != 200 {
        return Err(eyre!(
            "Failed to fetch message, error code {}.",
            response.status().as_str()
        ));
    }

    let body = response.text().await?;

    let body = serde_json::from_str::<shared::dto::Message>(&body)?;

    Ok(body.text)
}
