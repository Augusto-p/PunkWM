use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::google::oauth::TokenResponse;
use chrono::{NaiveDate, Local, TimeZone, Utc, Datelike};
use chrono::DateTime;
// use iana_time_zone::get_timezone;

#[derive(Debug, Deserialize)]
pub struct CalenderList {
    pub items: Vec<CalenderListEntry>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    
    
}


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CalenderListEntry {
    pub id: String,
    pub color_id: Option<String>,
    pub time_zone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CalenderEventEntryResponce {
    pub id: String,
    pub summary: String,
    pub location: Option<String>,
    pub start: EventDateTime,
}


#[derive(Debug, Deserialize, Clone)]
pub struct EventDateTime {
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>, // algunos eventos son all-day
    pub date: Option<String>,
    pub time_zone: Option<String>,
}



#[derive(Debug, Deserialize)]
pub struct CalenderEvent {
    pub items: Vec<CalenderEventEntryResponce>,
}



pub async fn get_calenders_list(access_token: &TokenResponse) -> Vec<CalenderListEntry> {
    let client = Client::new();
    let mut all_items = Vec::new();
    let mut page_token: Option<String> = None;
    loop {
        let mut req = client
            .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
            .query(&[("maxResults", "250")])
            .bearer_auth(&access_token.access_token)
            .header("Accept", "application/json");

        if let Some(ref token) = page_token {
            req = req.query(&[("pageToken", token)]);
        }

        let res = match req.send().await {
            Ok(r) => r,
            Err(_) => break,
        };

        let body: CalenderList = match res.json().await {
            Ok(b) => b,
            Err(_) => break,
        };

        all_items.extend(body.items);

        match body.next_page_token {
            Some(token) => page_token = Some(token),
            None => break,
        }
    }

    all_items
}


pub async fn get_events_list_by_calender(
    access_token: &TokenResponse,
    calendar: CalenderListEntry,
    max_events: u16,
) -> Vec<CalenderEventEntry> {

    let client = Client::new();
    let mut events = Vec::new();

    let now = Utc::now().to_rfc3339();


    let req = client
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
        .bearer_auth(&access_token.access_token)
        .header("Accept", "application/json");

    let res = match req.send().await {
        Ok(r) => r,
        Err(_) => {return events},
    };
    
    let body: CalenderEvent = match res.json().await {
            Ok(b) => b,
            Err(e) => {
                    println!("2>{}",e);
                    return events},
            };

    for event in body.items{
        let utc = event.start.to_unix_utc();
        let ev = CalenderEventEntry{
            id: event.id,
            summary: event.summary,
            location: event.location,
            start_datetime_utc: utc,
            color_id: calendar.color_id.clone()
        };
        events.push(ev);
    }
    
    events
}


pub async fn get_daily(access_token: &TokenResponse,max_events: u16) -> Vec<CalenderEventEntry>{
    let mut events = Vec::new();
    let calendars = get_calenders_list(&access_token).await;

    for calendar in calendars{
        let calender_events = get_events_list_by_calender(&access_token, calendar, max_events).await;
        events.extend(calender_events)
    }
    
    events.sort_by(|a, b| {
        let a_time = a.start_datetime_utc.unwrap_or(i64::MAX);
        let b_time = b.start_datetime_utc.unwrap_or(i64::MAX);
        a_time.cmp(&b_time)
    });
    events.into_iter().take(max_events.into()).collect()

}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CalenderEventEntry {
    pub id: String,
    pub summary: String,
    pub location: Option<String>,
    pub start_datetime_utc: Option<i64>,
    pub color_id: Option<String>
}




impl EventDateTime {
    pub fn to_unix_utc(&self) -> Option<i64> {
        if let Some(dt) = &self.date_time {
            let parsed = DateTime::parse_from_rfc3339(dt).ok()?;
            return Some(parsed.with_timezone(&Utc).timestamp());
        }
        if let Some(date) = &self.date {
            let d = NaiveDate::parse_from_str(date, "%Y-%m-%d")
                .expect("fecha inválida");

            let local_midnight = Local
                .with_ymd_and_hms(d.year(), d.month(), d.day(), 0, 0, 0)
                .single()
                .expect("hora ambigua o inexistente");

            let utc = local_midnight.with_timezone(&Utc);

            return Some(utc.timestamp());
        }
        None
    }
}
