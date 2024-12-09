use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use oauth2::AuthorizationCode;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::RedirectUrl;
use oauth2::Scope;
use oauth2::TokenResponse;
use native_tls::TlsConnector;
use mailparse::parse_mail;
use tiny_http::{Server, Response};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//#[tauri::command]
/*fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}*/

fn hi() -> String {
   
    "hi".to_string()
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("https://example.com/{}", name) // Generate a link
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

