use aws_config::meta::region::{RegionProviderChain};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, Client};
use std::env;
use std::io::prelude::*;
use bytes::Bytes;
use zip::ZipArchive;
use std::path::Path;
use std::fs::File;
use polars::prelude::{CsvReader, CsvWriter, SerWriter, DataFrame, SerReader};
use polars::error::PolarsResult;
use polars::frame::UniqueKeepStrategy;



async fn build_config_and_client() -> Client {
    let provider: RegionProviderChain = RegionProviderChain::first_try(env::var("REGION").ok().map(Region::new))
    .or_default_provider()
    .or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(provider).load().await;
    let client: Client = Client::new(&shared_config);
    return client
}

pub async fn download_object(
    bucket_name: &str,
    key: &str,
) -> ByteStream {
    let client: Client = build_config_and_client().await;
    let resp =
    client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await.unwrap();
    resp.body
}

pub fn write_bytes_to_zip_file(name: &str, data: Bytes) {
    let path = std::path::Path::new(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(&data).expect("error writing data");
    unzip_file(name, "data").expect("failed to unzip file")
}

fn unzip_file(zip_path: &str, destination_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file: File = File::open(zip_path)?;
    let mut zip_archive: ZipArchive<File> = ZipArchive::new(file)?;

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let file_path = file.sanitized_name();

        let destination = Path::new(destination_path).join(&file_path);

        if file.is_dir() {
            std::fs::create_dir_all(&destination)?;
        } else {
            if let Some(parent) = destination.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent)?;
                }
            }

            let mut output_file: File = File::create(&destination)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}

pub fn read_csv(file_path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(file_path)?
            .has_header(true)
            .finish()
}

pub fn combine_distinct_dataframes(graph: &mut DataFrame) -> DataFrame {
    let renamed: &mut DataFrame = &mut graph.rename("start_station_id", "graph_id").expect("blah blah blah")
    .rename("end_station_id", "graph_id_2").expect("blah blah blah");
    let first: DataFrame = renamed.select(["graph_id"]).expect("dangit").unique(None, 
        UniqueKeepStrategy::Any, None).expect("blah blah");

    let second: DataFrame = renamed.select(["graph_id_2"]).expect("dangit").rename("graph_id_2", "graph_id")
        .expect("dang").unique(None, 
            UniqueKeepStrategy::Any, None).expect("blah blah");

    let third: DataFrame = first.vstack(&second).expect("blah blah blah")
    .unique(None, 
        UniqueKeepStrategy::Any, None).expect("blah blah");
    
    return third;
}

pub fn write_dataframe_to_csv(df: &mut DataFrame, file: &str) {
    let mut output_file: File = File::create(file).unwrap();
    CsvWriter::new(&mut output_file)
        .has_header(true)
        .finish(df)
        .unwrap();
}
