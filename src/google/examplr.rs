mod google;
// use crate::google::oauth::get_auth_url;
// use crate::google::oauth::wait_for_code;
// use crate::google::oauth::exchange_code_for_token;
use crate::google::credentials::read_credentials;
use crate::google::oauth::get_access_token;
use crate::google::calendar::get_calenders_list;
use crate::google::calendar::get_events_list_by_calender;
use crate::google::calendar::get_daily;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_events = 12;
    // let scope = vec!["https://www.googleapis.com/auth/calendar.readonly".to_string()];
    if let Some(credenciales) = read_credentials("credentials.json".to_string()) {
        // let auth_url = get_auth_url(&credenciales, scope);
        // println!("URL: {}", auth_url);
        // let code = wait_for_code(credenciales.installed.redirect_uris[0].clone()).await;
        // let _ = exchange_code_for_token(code.clone(), &credenciales).await;
        // println!("Código recibido: {}", code);
        if let Some(access_token) = get_access_token(&credenciales).await {
            // let calendars = get_calenders_list(&access_token).await;
            // let events = get_events_list_by_calender(&access_token, calendars[5].clone(), max_events).await;
            let daily = get_daily(&access_token, max_events).await;
            println!("{:#?}", daily);



        }


    } else {
        println!("No se pudo leer el archivo de credenciales");
    }
    Ok(())
}
