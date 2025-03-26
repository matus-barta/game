use std::{
    fs::{self, OpenOptions},
    io::Write,
};

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
