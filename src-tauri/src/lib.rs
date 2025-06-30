#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// App state
#[derive(Debug, Default)]
pub struct AppState {
    pub permissions_granted: Mutex<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PermissionStatus {
    pub screen_recording: bool,
    pub accessibility: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CaptureResult {
    pub image_data: String, // Base64 encoded image
    pub bounds: CaptureBounds,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CaptureBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

// Tauri commands
#[tauri::command]
async fn check_permissions() -> Result<PermissionStatus, String> {
    // TODO: Implement actual permission checking for macOS
    Ok(PermissionStatus {
        screen_recording: false, // Will be implemented with native code
        accessibility: false,
    })
}

#[tauri::command]
async fn request_permissions() -> Result<bool, String> {
    // TODO: Implement permission request
    // For now, return false to trigger manual steps
    Ok(false)
}

#[tauri::command]
async fn open_system_preferences() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
            .spawn()
            .map_err(|e| format!("Failed to open System Preferences: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn capture_screen_region(bounds: CaptureBounds) -> Result<CaptureResult, String> {
    // TODO: Implement native screen capture
    // For now, return a placeholder
    Ok(CaptureResult {
        image_data: "placeholder".to_string(),
        bounds,
    })
}

#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    // TODO: Implement clipboard functionality
    println!("Copying to clipboard: {}", text);
    Ok(())
}

#[tauri::command]
async fn register_global_hotkey() -> Result<(), String> {
    // TODO: Implement global hotkey registration
    println!("Registering global hotkey: Option+Space");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            app.manage(AppState::default());
            
            // Register global hotkey
            // TODO: Implement this properly
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            check_permissions,
            request_permissions,
            open_system_preferences,
            capture_screen_region,
            copy_to_clipboard,
            register_global_hotkey,
        ])
        .run(tauri::generate_context!())
        .expect("error while running FrameSense application");
}
