pub async fn init_bucket(
    s3_access_key: String,
    s3_secret_key: String,
    s3_api: String,
    bucket: String,
) -> s3::Bucket {
    let bucket_name = bucket;

    let url = url::Url::parse(&s3_api).unwrap();
    validate_bucket_name(&bucket_name);

    let region = s3::Region::Custom {
        region: "eu-central-1".to_owned(), //TODO: env for region
        endpoint: remove_trailing_slash(url.to_string()),
    };

    let credentials =
        s3::creds::Credentials::new(Some(&s3_access_key), Some(&s3_secret_key), None, None, None)
            .unwrap();

    let mut bucket = s3::Bucket::new(&bucket_name, region.clone(), credentials.clone())
        .unwrap()
        .with_path_style();

    if !bucket.exists().await.unwrap() {
        bucket = s3::Bucket::create_with_path_style(
            &bucket_name,
            region,
            credentials,
            s3::BucketConfiguration::default(),
        )
        .await
        .unwrap()
        .bucket;
    }

    return bucket;
}

fn validate_bucket_name(bucket_name: &String) {
    for c in bucket_name.chars() {
        if c.is_uppercase() {
            panic!("Uppercase in bucket name!")
        }
    }
}

fn remove_trailing_slash(url: String) -> String {
    if url.ends_with("/") {
        return url.to_string()[0..url.len() - 1].to_string();
    } else {
        url
    }
}
