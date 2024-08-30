# Semsimian Server

A simple web server for the Semsimian project, written in Rust.

### Requirements: 
- [Rust](https://www.rust-lang.org/tools/install)
    - rustup >= 1.26.0
    - rustc >= 1.72.1
    - cargo >= 1.72.1

### Installation:

The following direct operational procedure may only work under Linux.

1. Clone the repository:
    ```bash
    git clone https://github.com/monarch-initiative/semsimian-server
    ```
2. Download the source data into the expected location (here we assume *nix commands and use **`wget`**, targeting the **2024-08-12** release. Substitute a more recent release as desired)
    ```bash
    mkdir -p .data/oaklib
    cd .data/oaklib
    wget https://data.monarchinitiative.org/monarch-kg/2024-08-12/phenio.db.gz
    gunzip phenio.db.gz
    ```
3. Build the project:
    ```bash
    cd semsimian-server
    cargo build 
    ```
4. Run the server:
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
    - `termset` is a comma-separated list of ontology CURIE terms.
    - `prefix` is a string that will be used to filter the results.
    - The server will return a JSON object containing a list of ontology terms that match the search term and prefix.

The full template of the search path endpoint is the following:

```
http://{semsim_server_host}:{semsim_server_port}/search/{','.join(termset)}/{prefix}:/{metric}?limit={limit}&directionality={directionality}
```

## Running using Docker

A Dockerfile is provided for convenience. While it is hosted on [Google Cloud Platform](us-central1-docker.pkg.dev/monarch-initiative/monarch-api/semsimian-server:latest),  
it can be built locally.

To build the image, run the following command from the root of the repository:
```bash
docker build -t semsimian-server .
```

To run the image on the **phenio.db** data file, locally cached under **`.data/oaklib`**, run the following command:

```bash
docker run --name semsimian_server -d -p 9999:9999 -v ./.data/oaklib:/usr/src/semsimian_server/.data/oaklib semsimian-server
```

As a reminder, the docker run **-d** flag runs the command in the background, **`-p`** maps it to port 9999.  For convenience, we also named the Docker container 'semsimian_server'.

Note that the 'host' path for the -v volume spec should use the OS-specific path, hence, for Microsoft Windows, the equivalent command should be:


```bash
docker run --name semsimian_server -d -p 9999:9999 -v .\.data\oaklib:/usr/src/semsimian_server/.data/oaklib semsimian-server 
```

After starting the server, you can check if it is running:

```bash
docker ps 
CONTAINER ID   IMAGE              COMMAND       CREATED          STATUS          PORTS                    NAMES
bff079e6473c   semsimian-server   "semserver"   22 seconds ago   Up 21 seconds   0.0.0.0:9999->9999/tcp   semsimian_server

```

and also, consult the logs:

```bash
docker logs -f semsimian_server
```

Which will show the startup caching, something like:

```
Generating cache! "PomBase:biolink:has_phenotypeflat"
Generating cache! "dictyBase:biolink:has_phenotypeflat"
Generating cache! "Xenbase:biolink:has_phenotypeflat"
etc...
```

The resulting application should now be accessible from http://localhost:9999, and may be access at the endpoints noted previously above. For example, you can execute an HTTP GET on a URL something like: 

```
http://localhost:9999/search/HP:0002104,HP:0012378/MONDO:/ancestor_information_content?limit=5&directionality=object_to_subject
```

which returns a SemSimian structured JSON document result of MONDO indexed human diseases with some relationship to the phenotypes specified by the given Human Phenotype Ontology (HP) CURIE terms.

To stop and delete the server container:

```bash
docker stop semsimian_server
docker rm semsimian_server
```
