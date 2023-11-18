use pgrx::iter::TableIterator;
use pgrx::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// pgx specific macros
pg_module_magic!();

const SERVICE_URL: &str = "http://localhost:8000";

#[pg_extern]

fn mr_service_url() -> &'static str {
    SERVICE_URL
}
#[derive(Serialize)]
struct Request {
    src: String,
    dest: String,
    weight: f64,
}
#[derive(Deserialize)]
struct Response {
    node: String,
    ego: String,
    score: f64,
}

#[pg_extern]
fn mr_node_score(
    ego: &'static str,
    target: &'static str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("{}/node_score/{}/{}", SERVICE_URL, ego, target);
    let resp = reqwest::blocking::get(url).unwrap().text();
    let body = resp?;
    let json: Value = serde_json::from_str(&body)?;
    let r: Response = serde_json::from_value(json)?;
    Ok(r.score)
}

#[pg_extern]
fn mr_scores(
    ego: &'static str,
) -> Result<
    TableIterator<'static, (name!(node, String), name!(ego, String), name!(score, f64))>,
    Box<dyn std::error::Error + 'static>,
> {
    let url = format!("{}/scores/{}", SERVICE_URL, ego);
    let body: String = reqwest::blocking::get(url).unwrap().text()?;
    let json: Value = serde_json::from_str(&body)?;
    let r: Vec<Response> = serde_json::from_value(json)?;
    let v: Vec<(String, String, f64)> = r
        .iter()
        .map(|row| (row.node.clone(), row.ego.clone(), row.score))
        .collect();
    Ok(TableIterator::new(v))
}

#[pg_extern]
fn mr_edge(
    src: &'static str,
    dest: &'static str,
    weight: f64,
) -> Result<String, Box<dyn std::error::Error>> {
    let req = Request {
        src: src.to_string(),
        dest: dest.to_string(),
        weight,
    };
    let url = format!("{}/edge", SERVICE_URL);
    let client = reqwest::blocking::Client::new();
    let body = client.put(url).json(&req).send()?.text()?;
    let json: Value = serde_json::from_str(&body)?;
    let message: String =
        json.get("message")
        .and_then(|v| serde_json::to_string(v).ok())
        .unwrap_or(format!("Warning: cannot decode HTTP reply: {}", body).to_string());
    Ok(message)
}
