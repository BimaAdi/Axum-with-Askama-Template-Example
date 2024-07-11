use axum_askama::{init_router, settings::get_setting};
use dotenv::dotenv;
use listenfd::ListenFd;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = init_router();

    let environtment = get_setting("ENVIRONTMENT".to_string(), false);
    let host = get_setting("HOST".to_string(), true).unwrap();
    let port = get_setting("PORT".to_string(), true).unwrap();
    let env_value = environtment.unwrap_or("prod".to_string());
    let listener: TcpListener = if env_value == *"dev".to_string() {
        // Development mode (autoreload)
        let mut listenfd = ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            // if we are given a tcp listener on listen fd 0, we use that one
            Some(x) => {
                x.set_nonblocking(true).unwrap();
                TcpListener::from_std(x).unwrap()
            }
            // otherwise fall back to local listening
            None => TcpListener::bind(format!("{}:{}", host.clone(), port.clone()))
                .await
                .unwrap(),
        }
    } else {
        // Production mode
        tokio::net::TcpListener::bind(format!("{}:{}", host.clone(), port.clone()))
            .await
            .unwrap()
    };

    println!("Server running on {}:{}", host, port);
    axum::serve(listener, app).await.unwrap();
}
