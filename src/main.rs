use aws_sdk_s3::primitives::ByteStream;
use graphrs;
//use aws_sdk_s3::primitives::ByteStream;
//use bytes::Bytes;
use polars::prelude::{DataFrame, UniqueKeepStrategy};
//use polars::frame::UniqueKeepStrategy;
use csv::ReaderBuilder;
use std::collections::VecDeque;
use std::fs::File;

#[derive(Clone)]
struct Node {
    id: String,
    edge: String,
}

#[tokio::main]
async fn main() {
    let file_stream = graphrs::download_object("divvy-tripdata", "202303-divvy-tripdata.zip").await;
    let data = file_stream
        .collect()
        .await
        .map(|data| data.into_bytes())
        .expect("error reading data");
    graphrs::write_bytes_to_zip_file("bikes.zip", data);
    // zip file will be auto extracted into data folder
    let df: DataFrame = graphrs::read_csv("data/202303-divvy-tripdata.csv").expect("error reading csv");
    let dirty_graph: DataFrame = df.select(["start_station_id", "end_station_id"]).expect("blah");
    let mut graph: DataFrame = dirty_graph.unique(None, UniqueKeepStrategy::Any, None).expect("blah blah");
    let mut out: DataFrame = graphrs::combine_distinct_dataframes(&mut graph);
    graphrs::write_dataframe_to_csv(&mut out, "nodes.csv");
    graphrs::write_dataframe_to_csv(&mut graph, "edges.csv");
    // read nodes with Pythons script to visualize graph

    let file: File = File::open("edges.csv").expect("error opening file");
    let mut reader = ReaderBuilder::new().from_reader(file);

    // Read and print each column
    let mut column_1: Vec<String> = Vec::new();
    let mut column_2: Vec<String> = Vec::new();
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: Vec<String> = Vec::new();

    for result in reader.records() {
        let record = result.expect("a CSV record");
        let value_1 = record.get(0).unwrap_or("");
        let value_2 = record.get(1).unwrap_or("");

        column_1.push(value_1.to_string());
        column_2.push(value_2.to_string());
        let Node{id, edge} = Node{id: value_1.to_string(), edge: value_2.to_string()};
        queue.push_back(Node{id, edge});
    }

    let start_node = Node { id: "20119".to_string(), edge: "".to_string() };
    let end_node = Node { id: "20104".to_string(), edge: "".to_string() };
    queue.push_back(start_node.clone());
    visited.insert(start_node.clone(), None);

    while let Some(current_node) = queue.pop_front() {
        if &current_node == end {
            let mut path = Vec::new();
            let mut current = &current_node;
            while let Some(node) = visited.get(current) {
                path.push(current.id.clone());
                current = node.as_ref().unwrap();
            }
            path.push(start.id.clone());
            path.reverse();

            return Some((path, path.len() - 1));
        }

        if let Some(neighbors) = graph.get(&current_node) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    visited.insert(neighbor.clone(), Some(current_node.clone()));
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
}
