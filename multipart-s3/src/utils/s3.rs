use crate::rusoto_s3::S3;
use rusoto_core::{ProvideAwsCredentials, Region, RusotoError};
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3Client};
use std::io::Read;
use std::io::Write;

pub struct Client {
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    // construct S3 testing client
    pub fn new() -> Client {
        let region = Region::ApNortheast2;

        Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub fn url(&self, key: &str) -> String {
        format!(
            "https://{}.s3.{}.amazonaws.com/{}",
            std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            "ap-northeast-2",
            key
        )
    }

    pub fn put_object(&self, localfilepath: &str, key: &str) -> String {
        let mut file = std::fs::File::open(localfilepath).unwrap();
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents);
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            body: Some(contents.into()),
            ..Default::default()
        };
        let res = self
            .s3
            .put_object(put_request)
            .sync()
            .expect("Failed to put test object");

        self.url(key)
    }

    pub fn delete_object(&self, key: String) {
        let delete_object_req = DeleteObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        };

        let res = self
            .s3
            .delete_object(delete_object_req)
            .sync()
            .expect("Couldn't delete object");
    }
}
