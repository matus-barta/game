use crate::{
    models::world_data::*,
    utils::error::{bad_request_error, internal_error},
    AppState,
};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use sha2::{Digest, Sha256};

/// POST -> /dev/model
/// allows to upload GLTF 3D model
/// internally generates SHA-265 of the file and uploads to S3
pub async fn create_model(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Vec<Asset>>, (StatusCode, String)> {
    let mut response_models: Vec<Asset> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(bad_request_error)? {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().unwrap_or_default().to_string();
        let content_type = field.content_type().unwrap_or_default().to_string();
        let data = field.bytes().await.unwrap_or_default();

        //TODO: Sanitize filename

        if file_name.is_empty() || data.is_empty() || content_type.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Filename, data or content_type is empty".to_string(),
            ))?;
        }
        if content_type != "model/gltf-binary" {
            return Err((StatusCode::BAD_REQUEST, "Wrong content type".to_string()))?;
            //TODO: create error into variable then pass to bad_req fn.. and wrap that fn inside Err()
        }

        let hash = Sha256::digest(&data);

        let _response_data = app_state
            .bucket
            .put_object_with_content_type(format!("/{:X}", hash), &data, &content_type)
            .await
            .map_err(internal_error)?;

        let db_response = sqlx::query_as!(
            Asset,
            r#"INSERT INTO "Asset" VALUES ($1, $2) RETURNING id, name"#,
            format!("{:X}", &hash),
            &file_name
        )
        .fetch_one(&app_state.db_pool)
        .await
        .map_err(internal_error)?;

        response_models.push(db_response);

        tracing::debug!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes with hash {:x}",
            data.len(),
            hash
        );
    }
    Ok(Json(response_models))
}
