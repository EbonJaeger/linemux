//! Demonstrates file event stream for a given set of files.
//!
//! Usage:
//!     lines /path/to/file1 /path/to/file2 ...
//!
//! The files could be present or not, but assume some data will eventually be
//! be written to them in order to generate lines.

use std::time::Duration;

use futures_util::stream::StreamExt;
use tokio;

use linemux::MuxedLines;

#[tokio::main(threaded_scheduler)]
pub async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut events = MuxedLines::new();

    for f in args {
        events.add_file(&f).await.unwrap();
    }

    while let Some(lineset) = events.next().await {
        let source = lineset.source().display();

        for line in lineset.iter() {
            println!("({}) {}", source, line);
        }

        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}