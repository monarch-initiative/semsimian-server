# Semsimian Server

A simple web server for the Semsimian project, written in Rust.

### Requirements: 
- [Rust](https://www.rust-lang.org/tools/install)
    - rustup >= 1.26.0
    - rustc >= 1.72.1
    - cargo >= 1.72.1

### Installation:
1. Clone the repository:
    ```bash
    git clone https://github.com/monarch-initiative/semsimian-server
    ```
2. Build the project:
    ```bash
    cd semsimian-server
    cargo build 
    ```
3. Run the server:
    ```bash
    cargo run
    ```

### Usage:

The server will be running on `0.0.0.0:7878` by default.  
The address and port can be configured in the `Rocket.toml` file, or by setting the `ROCKET_ADDRESS` and `ROCKET_PORT` environment variables. 

The server exposes two endpoints:

- `/compare/<termset1><termset2>`  
    - `termset1` and `termset2` are comma-separated lists of ontology terms.
    - The server will return a JSON object containing the similarity score between the two termsets.

- `/search/<termset>/<prefix>`  
    - `termset` is a comma-separated list of ontology terms.
    - `prefix` is a string that will be used to filter the results.
    - The server will return a JSON object containing a list of ontology terms that match the search term and prefix.

__**Docker Image**__

A Dockerfile is provided for convenience. 
To build the image, run the following command from the root of the repository:
```bash
docker build -t semsimian-server .
```

To run the image, run the following command:
```bash
docker run -p 9999:9999 semsimian-server
```