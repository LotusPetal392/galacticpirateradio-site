use askama::Template;
use axum::{
    Router,
    extract::State,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

const TRANSMISSIONS_PATH: &str = "data/recent_transmissions.json";
const GENERATION_INTERVAL_SECS: u64 = 3 * 60 * 60;
const MAX_TRANSMISSIONS: usize = 12;
const DEFAULT_SITE_URL: &str = "http://127.0.0.1:3000";
const OG_IMAGE_PATH: &str = "/static/images/gpr.png";

#[derive(Clone)]
struct AppState {
    transmissions: Arc<RwLock<TransmissionState>>,
    site_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct TransmissionState {
    last_generated_at: u64,
    entries: Vec<TransmissionEntry>,
}

#[derive(Clone, Serialize, Deserialize)]
struct TransmissionEntry {
    timestamp: u64,
    time_label: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let loaded = load_transmissions();
    let site_url = std::env::var("SITE_URL")
        .unwrap_or_else(|_| DEFAULT_SITE_URL.to_string())
        .trim_end_matches('/')
        .to_string();
    let state = AppState {
        transmissions: Arc::new(RwLock::new(loaded)),
        site_url,
    };

    generate_if_needed_and_persist(&state).await;
    start_transmission_generator(state.clone());

    let app = Router::new()
        .route("/", get(index))
        .route("/software", get(software))
        .route("/robots.txt", get(robots_txt))
        .route("/robots.txt/", get(robots_txt))
        .route("/sitemap.xml", get(sitemap_xml))
        .route("/sitemap.xml/", get(sitemap_xml))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{address}");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind server address");

    axum::serve(listener, app).await.expect("server error");
}

async fn index(State(app_state): State<AppState>) -> impl IntoResponse {
    generate_if_needed_and_persist(&app_state).await;
    let recent_transmissions = {
        let guard = app_state.transmissions.read().await;
        guard.entries.clone()
    };
    let canonical_url = absolute_url(&app_state.site_url, "/");
    let og_image_url = absolute_url(&app_state.site_url, OG_IMAGE_PATH);

    HtmlTemplate(IndexTemplate {
        title: "Galactic Pirate Radio",
        description: "Galactic Pirate Radio broadcasts transmission logs, archives, and updates from a hidden outpost at the edge of charted space.",
        current_path: "/",
        current_year: current_year(),
        canonical_url,
        og_image_url,
        og_type: "website",
        robots: "index,follow",
        site_url: app_state.site_url.clone(),
        recent_transmissions,
    })
}

async fn software(State(app_state): State<AppState>) -> impl IntoResponse {
    let canonical_url = absolute_url(&app_state.site_url, "/software");
    let og_image_url = absolute_url(&app_state.site_url, OG_IMAGE_PATH);
    HtmlTemplate(SoftwareTemplate {
        title: "Software | Ethereal Waves",
        description: "Ethereal Waves is a Linux music player built with libcosmic and GStreamer, with screenshots, feature roadmap, and keyboard shortcuts.",
        current_path: "/software",
        current_year: current_year(),
        canonical_url,
        og_image_url,
        og_type: "software",
        robots: "index,follow",
        site_url: app_state.site_url,
    })
}

async fn not_found(State(app_state): State<AppState>) -> impl IntoResponse {
    let canonical_url = absolute_url(&app_state.site_url, "/404");
    let og_image_url = absolute_url(&app_state.site_url, OG_IMAGE_PATH);
    (
        StatusCode::NOT_FOUND,
        HtmlTemplate(NotFoundTemplate {
            title: "404 Not Found",
            description: "The requested Galactic Pirate Radio page could not be found.",
            current_path: "",
            current_year: current_year(),
            canonical_url,
            og_image_url,
            og_type: "website",
            robots: "noindex,follow",
            site_url: app_state.site_url,
        }),
    )
}

async fn robots_txt(State(app_state): State<AppState>) -> impl IntoResponse {
    let body = format!(
        "User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n",
        app_state.site_url
    );
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body)
}

async fn sitemap_xml(State(app_state): State<AppState>) -> impl IntoResponse {
    let home = absolute_url(&app_state.site_url, "/");
    let software = absolute_url(&app_state.site_url, "/software");
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{home}</loc>
  </url>
  <url>
    <loc>{software}</loc>
  </url>
</urlset>
"#
    );
    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        body,
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
    description: &'static str,
    current_path: &'static str,
    current_year: i32,
    canonical_url: String,
    og_image_url: String,
    og_type: &'static str,
    robots: &'static str,
    site_url: String,
    recent_transmissions: Vec<TransmissionEntry>,
}

#[derive(Template)]
#[template(path = "software.html")]
struct SoftwareTemplate {
    title: &'static str,
    description: &'static str,
    current_path: &'static str,
    current_year: i32,
    canonical_url: String,
    og_image_url: String,
    og_type: &'static str,
    robots: &'static str,
    site_url: String,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate {
    title: &'static str,
    description: &'static str,
    current_path: &'static str,
    current_year: i32,
    canonical_url: String,
    og_image_url: String,
    og_type: &'static str,
    robots: &'static str,
    site_url: String,
}

fn absolute_url(site_url: &str, path: &str) -> String {
    format!("{site_url}{path}")
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

fn load_transmissions() -> TransmissionState {
    match fs::read_to_string(TRANSMISSIONS_PATH) {
        Ok(content) => match serde_json::from_str::<TransmissionState>(&content) {
            Ok(state) if !state.entries.is_empty() => state,
            Ok(_) | Err(_) => {
                let state = default_transmissions();
                let _ = persist_transmissions(&state);
                state
            }
        },
        Err(_) => {
            let state = default_transmissions();
            let _ = persist_transmissions(&state);
            state
        }
    }
}

fn default_transmissions() -> TransmissionState {
    let now = unix_now_secs();
    TransmissionState {
        last_generated_at: now,
        entries: vec![
            TransmissionEntry {
                timestamp: now.saturating_sub(1_200),
                time_label: clock_label_from_unix(now.saturating_sub(1_200)),
                message: "Uplink stabilized. Archive index pushed to public relay.".to_string(),
            },
            TransmissionEntry {
                timestamp: now.saturating_sub(3_300),
                time_label: clock_label_from_unix(now.saturating_sub(3_300)),
                message: "Detected repeating pattern in ambient static. Logged as anomaly A-17."
                    .to_string(),
            },
            TransmissionEntry {
                timestamp: now.saturating_sub(5_800),
                time_label: clock_label_from_unix(now.saturating_sub(5_800)),
                message: "Scheduled new broadcast: Deep Space Transmitter.".to_string(),
            },
        ],
    }
}

fn persist_transmissions(state: &TransmissionState) -> std::io::Result<()> {
    if let Some(parent) = Path::new(TRANSMISSIONS_PATH).parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(state).map_err(std::io::Error::other)?;
    fs::write(TRANSMISSIONS_PATH, json)
}

async fn generate_if_needed_and_persist(app_state: &AppState) {
    let now = unix_now_secs();
    let snapshot = {
        let mut guard = app_state.transmissions.write().await;
        if maybe_generate_transmission(&mut guard, now) {
            Some(guard.clone())
        } else {
            None
        }
    };

    if let Some(state) = snapshot
        && let Err(error) = persist_transmissions(&state)
    {
        eprintln!("failed to persist transmissions: {error}");
    }
}

fn maybe_generate_transmission(state: &mut TransmissionState, now: u64) -> bool {
    if now.saturating_sub(state.last_generated_at) < GENERATION_INTERVAL_SECS {
        return false;
    }

    let message = generate_scifi_message(now, state.entries.len());
    state.entries.insert(
        0,
        TransmissionEntry {
            timestamp: now,
            time_label: clock_label_from_unix(now),
            message,
        },
    );
    state.entries.truncate(MAX_TRANSMISSIONS);
    state.last_generated_at = now;
    true
}

fn generate_scifi_message(now: u64, entry_count: usize) -> String {
    let subjects = [
        "Long-range scanner",
        "Relay drone",
        "Pirate beacon",
        "Outer rim array",
        "Subspace receiver",
        "Navigation core",
    ];
    let actions = [
        "locked onto",
        "decoded",
        "flagged",
        "stabilized",
        "rerouted",
        "intercepted",
    ];
    let objects = [
        "a drifting colony ping",
        "an encrypted trader channel",
        "a rogue moon telemetry burst",
        "a hidden wormhole marker",
        "an ion storm distress packet",
        "a ghost-fleet handshake",
    ];

    let s = ((now / 7) as usize + entry_count * 3) % subjects.len();
    let a = ((now / 11) as usize + entry_count * 5) % actions.len();
    let o = ((now / 13) as usize + entry_count * 7) % objects.len();
    format!("{} {} {}.", subjects[s], actions[a], objects[o])
}

fn clock_label_from_unix(unix_seconds: u64) -> String {
    let seconds_today = unix_seconds % 86_400;
    let hour = seconds_today / 3_600;
    let minute = (seconds_today % 3_600) / 60;
    let second = seconds_today % 60;
    format!("{hour:02}:{minute:02}:{second:02}")
}

fn unix_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn start_transmission_generator(app_state: AppState) {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(300));
        loop {
            ticker.tick().await;
            generate_if_needed_and_persist(&app_state).await;
        }
    });
}
