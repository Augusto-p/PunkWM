use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Utc};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::GLOBAL_CFG,
    utils::google::oauth::Google,
};

#[derive(Debug, Deserialize)]
pub struct CalendarList {
    pub items: Vec<CalendarListEntry>,

    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CalendarListEntry {
    pub id: String,
    pub color_id: Option<String>,

    pub time_zone: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CalendarEventResponse {
    pub id: String,
    pub summary: Option<String>,
    pub location: Option<String>,
    pub start: EventDateTime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EventDateTime {
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,

    pub date: Option<String>,

    pub time_zone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CalendarEvents {
    pub items: Vec<CalendarEventResponse>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CalendarEventEntry {
    pub id: String,
    pub summary: String,
    pub location: Option<String>,
    pub start_datetime_utc: Option<i64>,
    pub color_id: Option<String>,
}

impl EventDateTime {
    pub fn to_unix_utc(&self) -> Option<i64> {
        if let Some(dt) = &self.date_time {
            let parsed = DateTime::parse_from_rfc3339(dt).ok()?;

            return Some(parsed.timestamp());
        }

        if let Some(date) = &self.date {
            let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;

            let local_midnight = Local
                .with_ymd_and_hms(
                    d.year(),
                    d.month(),
                    d.day(),
                    0,
                    0,
                    0,
                )
                .single()?;

            return Some(local_midnight.timestamp());
        }

        None
    }
}

impl Google {
    pub async fn get_daily(&mut self) -> Vec<CalendarEventEntry> {
        if self.ensure_access_token().await.is_none() {
            return Vec::new();
        }
        let token = match self.access_token() {
            Some(t) => t.to_string(),
            None => return Vec::new(),
        };
        let calendars = self.get_calendars_list(&token).await;

        let futures = calendars
            .into_iter()
            .map(|calendar| {
                self.get_events_by_calendar(calendar, token.clone())
            });

        let results = join_all(futures).await;

        let mut events: Vec<_> = results
            .into_iter()
            .flatten()
            .flatten()
            .collect();

        events.sort_by_key(|e| {
            e.start_datetime_utc.unwrap_or(i64::MAX)
        });

        events
    }

    async fn get_events_by_calendar(&self, calendar: CalendarListEntry, token: String,) -> Option<Vec<CalendarEventEntry>> {
        let max_events = {
            let cfg = GLOBAL_CFG.read().ok()?;
            cfg.google().get_events()
        };

        let now = Utc::now().to_rfc3339();
        let response = self
            .client
            .get(format!(
                "https://www.googleapis.com/calendar/v3/calendars/{}/events",
                calendar.id
            ))
            .query(&[
                ("maxResults", max_events.to_string()),
                ("orderBy", "startTime".to_string()),
                ("singleEvents", "true".to_string()),
                ("timeMin", now),
            ])
            .bearer_auth(token)
            .header("Accept", "application/json")
            .send()
            .await
            .ok()?;

        if !response.status().is_success() {
            return None;
        }

        let body: CalendarEvents = response.json().await.ok()?;

        Some(
            body.items
                .into_iter()
                .map(|event| CalendarEventEntry {
                    id: event.id,
                    summary: event.summary.unwrap_or_default(),
                    location: event.location,
                    start_datetime_utc: event.start.to_unix_utc(),
                    color_id: calendar.color_id.clone(),
                })
                .collect(),
        )
    }

    async fn get_calendars_list(&self, token: &str) -> Vec<CalendarListEntry> {
        let mut all_items = Vec::new();

        let mut page_token = None;

        loop {
            let mut req = self
                .client
                .get(
                    "https://www.googleapis.com/calendar/v3/users/me/calendarList",
                )
                .query(&[("maxResults", "250")])
                .bearer_auth(token)
                .header("Accept", "application/json");

            if let Some(ref token) = page_token {
                req = req.query(&[("pageToken", token)]);
            }

            let response = match req.send().await {
                Ok(r) => r,
                Err(_) => break,
            };

            if !response.status().is_success() {
                break;
            }

            let body: CalendarList =
                match response.json().await {
                    Ok(b) => b,
                    Err(_) => break,
                };

            all_items.extend(body.items);

            match body.next_page_token {
                Some(token) => {
                    page_token = Some(token)
                }
                None => break,
            }
        }

        all_items
    }
}