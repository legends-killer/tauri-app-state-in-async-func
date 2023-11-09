// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager};

async fn async_fn () {}

// get_state(|foo_state| xx::xx(foo_state).await);
async fn my_async_fn(foo_state: Arc<Mutex<FooState>>) -> Result<String, String> {
    // call some async function
    // update foo_state
    let mut t = foo_state.lock().unwrap();
    let v = t.bar.as_mut();
    *foo_state.lock().unwrap() = FooState {
        bar: String::from("hello"),
    };
    async_fn().await;
    Ok("".to_string())
}

// ERROR:
// future cannot be sent between threads safely
// within `impl Future<Output = Result<String, String>>`, the trait `Send` is not implemented for `MutexGuard<'_, FooState>`
#[tauri::command]
async fn greet(app_state: tauri::State<'_, AppState>) -> Result<String, String> {
    let res = my_async_fn(app_state.foo_state.clone()).await;
    Ok("".to_string())
}

pub struct FooState {
    pub bar: String,
}

pub struct AppState {
    // auto_ptr
    // atomic reference count
    pub foo_state: Arc<Mutex<FooState>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            foo_state: Arc::new(Mutex::new(FooState {
                bar: Default::default(),
            })),
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
