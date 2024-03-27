# KVR Cache

Following the recent discussions around Redis license policy shifting, I had a need for a very simple and lightweight key/value in-memory store that could be accessed over HTTP.

I decided to see what I could achieve in Rust and see what sort of realistic performance could be achieved.

## Build

Within the kvr-cache folder, execute the following command:

run cargo build

## Run

kvr-cache 127.0.0.1:7000

*(On Windows, this will be kvr-cache.exe)*

## Plans for the future

* Support for direct TCP connections.
* Support for gRPC.
* Options to utilise storage other than in-memory.

## Performance tests

Testing locally; with 25 connections and 12 workers - retrieving a simple value allows for roughly 50,000 requests per second.

*This performance test was unoptimised, on a local machine.*
