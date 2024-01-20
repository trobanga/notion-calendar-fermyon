use std::str::FromStr;

use anyhow::Result;
use http::Response;
use spin_sdk::{
    http::{IntoResponse, Request},
    http_component, variables,
};
use uuid::Uuid;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_notion_calendar_fermyon(req: Request) -> Result<impl IntoResponse> {
    let mut args = req.path().split('/');
    args.next(); // empty

    let (id, format) = if let Some(arg) = args.next() {
        if arg == "org" {
            (args.next().unwrap(), notion_calendar::CalendarFormat::Org)
        } else if arg == "ical" {
            (args.next().unwrap(), notion_calendar::CalendarFormat::Ical)
        } else {
            (arg, notion_calendar::CalendarFormat::Ical)
        }
    } else {
        panic!();
    };

    let id = Uuid::from_str(&id)?;
    let api_token = variables::get("notion_api_token").expect("Could not get notion_api_token");
    let notion_db_id = variables::get("notion_db_id").expect("Could not get notion_id_id");
    let ical_prod_id = variables::get("ical_prod_id").expect("Defaults to NotionCalendar");

    let notion_calendar =
        notion_calendar::NotionCalendar::new(api_token, &notion_db_id, ical_prod_id)?;

    let calendar = notion_calendar
        .calendar_for_user(&id.to_string(), format)
        .await?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain; charset=utf-8")
        .body(calendar)?)
}
