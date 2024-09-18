//! This file provides general routes and handlers for CRUD operations regarding `Board`s specifically.

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use axum::{Json, Router};
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use log::debug;
use serde_json::json;

use crate::api::AppState;
use crate::hardware::device::Device;
use crate::utils::entity::Id;

/// Consolidates all available REST API routes for `Device`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handler_devices_list), //.post(handler_create_device)
        )
        .route("/mp3player/:id/files", get(handle_mp3_player_files_list))
        .route(
            "/mp3player/:id/file/upload",
            post(handle_mp3_player_file_upload),
        )
        .route(
            "/mp3player/:id/file/delete",
            delete(handle_mp3_player_file_delete),
        )
}

/// GET /:version/devices.
/// Retrieves all devices information.
async fn handler_devices_list(State(state): State<AppState>) -> impl IntoResponse {
    debug!("REST API: [device:list]");
    let devices = state.database.read().list::<Device>().unwrap();
    Json(devices)
}

/// GET /:version/devices/mp3player/:id/files.
/// List all available files.
async fn handle_mp3_player_files_list(Path(id): Path<Id>) -> Result<impl IntoResponse, StatusCode> {
    debug!("REST API: [device:mp3_player:files] list files");

    let paths = std::fs::read_dir(format!("./misc/files/{}/", id))
        .map_err(|_| StatusCode::NOT_FOUND)?
        .filter_map(|entry| {
            match entry {
                Err(_) => None, // Handle potential errors while reading entries
                Ok(entry) => {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    // Check if the file has an MP3 extension
                    match file_name_str.ends_with(".mp3") {
                        false => None,
                        true => Some(json!({
                            "name": file_name_str,
                            "path": entry.path(),
                        })),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(paths))
}

/// POST /:version/devices/mp3player/:id/file/upload.
/// Upload a file.
async fn handle_mp3_player_file_upload(
    Path(id): Path<Id>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    debug!(
        "REST API: [device:mp3_player:upload] upload file for device {}",
        id
    );

    // Ensure the directory exists
    let dir_path = format!("./misc/files/{}/", id);
    if let Err(e) = create_dir_all(&dir_path) {
        eprintln!("Failed to create directory: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create directory",
        )
            .into_response();
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        // Check if the content type is mp3
        if content_type == "audio/mpeg" {
            // Construct file path
            let file_path = PathBuf::from(&dir_path).join(&file_name);

            // Write data to file
            let mut file = match File::create(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to create file: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create file")
                        .into_response();
                }
            };

            if let Err(e) = file.write_all(&data) {
                eprintln!("Failed to write to file: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file")
                    .into_response();
            }

            println!(
                "Saved `{name}` (`{file_name}`: `{content_type}`) to {}",
                file_path.display()
            );
        } else {
            println!("Skipped `{name}` (`{file_name}`: `{content_type}`) as it is not an MP3 file");
        }
    }

    (StatusCode::OK, "File upload successful").into_response()
}

/// DELETE /:version/devices/mp3player/:id/file/delete.
/// Delete a file
async fn handle_mp3_player_file_delete(
    Path(id): Path<Id>,
    name: String,
) -> Result<impl IntoResponse, StatusCode> {
    debug!(
        "REST API: [device:mp3_player:delete] delete file {} for device {}",
        name, id
    );

    // Construct the directory path
    let dir_path = format!("./misc/files/{}/", id);
    // Read directory entries and map errors to `StatusCode::NOT_FOUND`
    let paths = std::fs::read_dir(&dir_path).map_err(|_| StatusCode::NOT_FOUND)?;

    // Find the file matching the `name`
    let file = paths
        .filter_map(|entry| entry.ok()) // Filter out Err entries
        .find(|entry| {
            // Compare the file name with the provided name
            entry.file_name().into_string().unwrap() == name
        })
        .ok_or(StatusCode::NOT_FOUND)?; // Return NOT_FOUND if no file matches

    // Construct the full path to the file
    let file_path: PathBuf = file.path();

    // Attempt to delete the file, returning `StatusCode::NOT_FOUND` on error
    std::fs::remove_file(&file_path).map_err(|_| StatusCode::NOT_FOUND)?;

    // Return success response
    Ok(StatusCode::OK)
}
