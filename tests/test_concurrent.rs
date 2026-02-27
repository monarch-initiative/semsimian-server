use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use axum::extract::{Path, Query};
use semsimian_server::{search, SearchParams, QueryParams};

#[tokio::test]
async fn test_concurrent_search_operations() {
    let num_tasks = 5;
    let requests_per_task = 2;
    let completed_requests = Arc::new(AtomicUsize::new(0));
    let start_time = Instant::now();

    println!("Starting {} concurrent search operations...", num_tasks * requests_per_task);

    let mut handles = Vec::new();
    for task_id in 0..num_tasks {
        let completed = Arc::clone(&completed_requests);
        handles.push(tokio::spawn(async move {
            for request_id in 0..requests_per_task {
                let task_start = Instant::now();

                let _response = search(
                    Path(SearchParams {
                        termset: "HP:0000001,HP:0000002".to_string(),
                        prefix: "ZFIN".to_string(),
                        metric: Some("ancestor_information_content".to_string()),
                    }),
                    Query(QueryParams {
                        limit: Some(1),
                        direction: Some("bidirectional".to_string()),
                    }),
                ).await;

                let task_elapsed = task_start.elapsed();
                let total_completed = completed.fetch_add(1, Ordering::Relaxed) + 1;

                println!("Task {} Request {} completed in {:?} (Total: {})",
                         task_id, request_id, task_elapsed, total_completed);
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let total_time = start_time.elapsed();
    let total_requests = completed_requests.load(Ordering::Relaxed);

    println!("All {} requests completed in {:?}", total_requests, total_time);
    println!("Average time per request: {:?}", total_time / total_requests as u32);

    assert_eq!(total_requests, num_tasks * requests_per_task);
}
