use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/software", get(software))
        .route("/about", get(about_redirect))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind server address");

    axum::serve(listener, app).await.expect("server error");
}

async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {
        title: "Home",
        current_path: "/",
        current_year: current_year(),
    })
}

async fn software() -> impl IntoResponse {
    HtmlTemplate(SoftwareTemplate {
        title: "Software",
        current_path: "/software",
        current_year: current_year(),
    })
}

async fn about_redirect() -> Redirect {
    Redirect::permanent("/software")
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        HtmlTemplate(NotFoundTemplate {
            title: "404 Not Found",
            current_path: "",
            current_year: current_year(),
        }),
    )
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("template render error: {error}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
    current_path: &'static str,
    current_year: i32,
}

#[derive(Template)]
#[template(path = "software.html")]
struct SoftwareTemplate {
    title: &'static str,
    current_path: &'static str,
    current_year: i32,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    title: &'static str,
    current_path: &'static str,
    current_year: i32,
}

fn current_year() -> i32 {
    const SECONDS_PER_DAY: i64 = 86_400;
    let seconds_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let days_since_epoch = seconds_since_epoch.div_euclid(SECONDS_PER_DAY);
    year_from_unix_days(days_since_epoch)
}

fn year_from_unix_days(days_since_epoch: i64) -> i32 {
    // Convert Unix days to Gregorian year using a civil date algorithm.
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096).div_euclid(365);
    let mut year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let month = mp + if mp < 10 { 3 } else { -9 };

    year += if month <= 2 { 1 } else { 0 };
    year as i32
}
