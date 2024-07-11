use askama::Template;
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

/// Using template file example
#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello() -> Html<String> {
    let hello = HelloTemplate { name: "world" };
    Html(hello.render().unwrap())
}

/// No template file example
#[derive(Template)]
#[template(source = r#"Hello, {{ name }}"#, ext = "html")]
struct NoTemplate {
    name: String,
}

async fn no_template_file() -> Html<String> {
    let no_template = NoTemplate {
        name: "no template".to_string(),
    };
    Html(no_template.render().unwrap())
}

/// Conditional
#[derive(Template)]
#[template(
    source = r#"
    {%- if name == "John" -%} 
        Hello John my friend 
    {%- else if name == "Doe" -%} 
        Hello my best friend Doe
    {%- else -%}
        You are not John nor Doe
    {%- endif -%}"#,
    ext = "html"
)]
struct Conditional {
    name: String,
}

#[derive(Deserialize, Clone)]
struct ConditionalQuery {
    name: Option<String>,
}

async fn conditional(query: Query<ConditionalQuery>) -> Html<String> {
    let name = query.clone().name.clone().unwrap_or("".to_string());
    let no_template = Conditional { name };
    Html(no_template.render().unwrap())
}

/// Loop
#[derive(Template)]
#[template(
    source = r#"
    <ul>
    {% for name in names %}
        <li>{{name}}</li>
    {% endfor %}
    </ul>"#,
    ext = "html"
)]
struct ForLoop {
    names: Vec<String>,
}

async fn for_loop() -> Html<String> {
    let for_loop = ForLoop {
        names: vec![
            "Alpha".to_string(),
            "Beta".to_string(),
            "Charlie".to_string(),
        ],
    };
    Html(for_loop.render().unwrap())
}

pub fn initiate_askama_route() -> Router {
    Router::new()
        .route("/hello/", get(hello))
        .route("/no-template/", get(no_template_file))
        .route("/conditional/", get(conditional))
        .route("/loop/", get(for_loop))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, extract::Request, http::StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn hello_router() {
        let app = initiate_askama_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn no_template_router() {
        let app = initiate_askama_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/no-template/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn conditional_route() {
        let app = initiate_askama_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/conditional/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn loop_route() {
        let app = initiate_askama_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/loop/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
