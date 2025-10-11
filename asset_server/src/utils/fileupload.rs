use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use axum::{extract::Multipart, http::StatusCode, response::IntoResponse};

pub async fn file_upload(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_name = String::new();
    let mut chunk_number = 0;
    let mut total_chunks = 0;
    let mut chunk_data = Vec::new();

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(err) => {
            tracing::error!("Error reading multipart field: {:?}", err);
            return StatusCode::BAD_REQUEST;
        }
    } {
        let field_name = field.name().unwrap_or_default().to_string();
        match field_name.as_str() {
            "fileName" => file_name = sanitize_filename(&field.text().await.unwrap_or_default()),
            "chunkNumber" => {
                chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0)
            }
            "totalChunks" => {
                total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0)
            }
            "chunk" => {
                chunk_data = field
                    .bytes()
                    .await
                    .unwrap_or_else(|_| Vec::new().into())
                    .to_vec()
            }
            _ => {}
        }
    }

    if file_name.is_empty() || chunk_data.is_empty() {
        tracing::error!("Filename of chunk data empty");
        return StatusCode::BAD_REQUEST;
    }

    let temp_dir = format!("./uploads/temp/{}", file_name);
    fs::create_dir_all(&temp_dir).unwrap_or_else(|_| {});

    let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
    let mut file = File::create(&chunk_path).unwrap();
    file.write_all(&chunk_data).unwrap();

    if is_upload_complete(&temp_dir, total_chunks) {
        assemble_file(&temp_dir, &file_name, total_chunks).unwrap();
    }

    StatusCode::OK
}

pub fn is_upload_complete(temp_dir: &str, total_chunks: usize) -> bool {
    match fs::read_dir(temp_dir) {
        Ok(entries) => entries.count() == total_chunks,
        Err(_) => false,
    }
}

pub fn assemble_file(temp_dir: &str, file_name: &str, total_chunks: usize) -> std::io::Result<()> {
    let output_path = format!("./uploads/{}", file_name);
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&output_path)?;

    for chunk_number in 0..total_chunks {
        let chunk_path = format!("{}/chunk_{}", temp_dir, chunk_number);
        let chunk_data = fs::read(&chunk_path)?;
        output_file.write_all(&chunk_data)?;
    }

    fs::remove_dir_all(temp_dir)?;
    Ok(())
}

pub fn sanitize_filename(filename: &String) -> String {
    filename.to_string() //TODO: do something smarter
}
