# http-client-bridge-rs
HTTP client bridge rs is a rust clone of [http-client-bridge](https://github.com/JoshMcguigan/http-client-bridge).

>HTTP client bridge is an HTTP server that allows one-way communication of arbitraty data between two groups of HTTP clients. The server listens for GET requests for data and static files from the front-end clients, and POST requests from the back-end clients. The server is configured to use a different port for the front-end and back-end clients to limit access to each function. For example, HTTP client bridge could be configured to listen to GET requests from the front-end clients on port 80, and listen to POST requests from the back-end clients on port 3000.
>
>Back-end clients can POST to any url path, and front end clients can GET the data by prepending /api/ to the path used by the back-end client. Static files are served at the root url.
    
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
