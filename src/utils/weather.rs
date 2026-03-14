use reqwest;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};

use reqwest::header::{
    HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE, REFERER
};
#[derive(Debug, Deserialize)]
struct AutoCompleteResponse {
    key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    temp: String,
    phrase: String,
    icon: String,
    wind_direction: String,
    wind_speed: String
}


pub async fn get_weather(
    city: String,
    state: String,
    country: String,
    lang: String,
    units: String,
) -> Option<Weather> {

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0"),
    );
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(&lang).ok()?);
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://www.accuweather.com/"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .ok()?;

    // ---------- AUTOCOMPLETE ----------
    let response_autocomplete = client
        .get("https://www.accuweather.com/web-api/autocomplete")
        .query(&[
            ("query", format!("{}, {}, {}", city, state, country)),
            ("lang", lang.clone()),
        ])
        .send()
        .await
        .ok()?;

    if !response_autocomplete.status().is_success() {
        return None;
    }

    let autocomplete: Vec<AutoCompleteResponse> =
        response_autocomplete.json().await.ok()?;

    let key = autocomplete.first()?.key.clone();
    let key_decoded = urlencoding::decode(&key).ok()?.to_string();

    // ---------- REDIRECT ----------
    let response_redirect = client
        .get("https://www.accuweather.com/web-api/three-day-redirect")
        .query(&[
            ("key", key_decoded),
            ("lang", lang.clone()),
        ])
        .send()
        .await
        .ok()?;

    if !response_redirect.status().is_success() {
        return None;
    }

    let redirect_html = response_redirect.text().await.ok()?;
    let document_redirect = Html::parse_document(&redirect_html);

    let selector_link = Selector::parse("link").ok()?;
    let link_redirect = document_redirect
        .select(&selector_link)
        .next()?
        .value()
        .attr("href")?
        .replace("weather-forecast", "current-weather");

    // ---------- WEATHER PAGE ----------
    let response = client
        .get(link_redirect)
        .query(&[
            ("unit", if units == "imperial" { "f" } else { "c" }),
            ("lang", &lang),
        ])
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let body = response.text().await.ok()?;
    let document = Html::parse_document(&body);

    let temp = document
        .select(&Selector::parse("div.display-temp").ok()?)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())?;

    let phrase = document
        .select(&Selector::parse("div.phrase").ok()?)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())?;

    let icon = document
        .select(&Selector::parse("svg.icon").ok()?)
        .next()
        .and_then(|e| e.value().attr("data-src"))
        .map(|src| {
            src.split('/')
                .last()
                .unwrap_or("")
                .replace(".svg", "")
        })?;

    // ---------- WIND ----------
    let panel_sel = Selector::parse("p.panel-item").ok()?;
    let value_sel = Selector::parse("span.value").ok()?;

    let wind_text = document
        .select(&panel_sel)
        .find(|e| e.text().collect::<String>().contains("/h"))?
        .select(&value_sel)
        .next()
        .map(|v| v.text().collect::<String>().trim().to_string())?;

    let mut parts = wind_text.split_whitespace();
    let wind_direction = parts.next().unwrap_or("").to_string();
    let wind_speed = parts.collect::<Vec<_>>().join(" ");

    Some(Weather {
        temp,
        phrase,
        icon,
        wind_direction,
        wind_speed,
    })
}