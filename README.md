# Celestica Vector Search

![Rust](https://github.com/EtaCassiopeia/decentralized_celestica/actions/workflows/rust.yml/badge.svg)


Celestica is a vector database for storing and searching high-dimensional vectors. It is based on [Hierarchical Navigable Small World (HNSW) graphs](https://arxiv.org/abs/1603.09320). It leverages `IPFS/IPLD` for storing the vectors and uses gRPC for communication.

## Prerequisites

- Rust: Install Rust using [rustup](https://rustup.rs/)
- Docker and Docker Compose: Install [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
- gRPCurl (optional): Install [gRPCurl](https://github.com/fullstorydev/grpcurl#installation) for testing gRPC API

## Running the Application

1. Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/EtaCassiopeia/decentralized_celestica.git
cd decentralized_celestica
```

2. Build and run the application:

### Docker and docker-compose

Create a shared Docker network:

```
docker network create celestica-shared-net
```

This project can be built and run using Docker and docker-compose. There are two options to do this:

#### Option 1: Build the image separately and use it to run the service

1.  Make sure Docker is installed and running on your machine.
2.  Navigate to the project's root directory.
3.  Run the provided shell script to build the Docker image and automatically update the `CELESTICA_VERSION` environment variable in the `docker-compose.yml` file:

```bash
chmod +x build_docker_image.sh

./build_docker_image.sh
```

This script will build the Docker image, tag it with the version number from `Cargo.toml` and as "latest", and then run `docker-compose up` to start the services.

#### Option 2: Build the image via docker-compose and use it

1.  Make sure Docker and `docker-compose` are installed and running on your machine.
2.  Navigate to the project's root directory.
3.  Uncomment the `build: .` line in the `docker-compose.yml` file under the `celestica` service.
4.  Run `docker-compose up --build` to build the image and start the services:

```bash
docker-compose up --build
```

This command will build the Docker image as part of the `docker-compose` process and start the services using the newly built image.

Note: If you choose Option 2, make sure to update the `CELESTICA_VERSION` environment variable in the `docker-compose.yml` file manually to match the version in the `Cargo.toml` file.

## Usage

### REST API

To interact with the REST API, you can use `curl` or any HTTP client.

#### Inserting a Vector

```bash 
curl -X POST http://localhost:8080/insert \
     -H "Content-Type: application/json" \
     -d '{
  "data": [[[0.1, 0.2, 0.3],1],[[0.4, 0.5, 0.6],2]]}'
```

#### Search for similar vectors

```bash
curl -X POST http://localhost:8080/search \
     -H "Content-Type: application/json" \
     -d '{"data": [[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]], "knbn": 2, "ef": 50}'
```

### gRPC API

To interact with the gRPC API, you can use `grpcurl` or any gRPC client.

#### Inserting a Vector

```bash
grpcurl -plaintext -d '{"data": [{"values": [0.1, 0.2, 0.3]}, {"values": [0.4, 0.5, 0.6]}], "ids": [1, 2]}' \
    -import-path . \
    -proto proto/vector_service.proto \
    localhost:50051 vector_service.VectorService/Insert

```

#### Search for similar vectors

```bash
grpcurl -plaintext -d '{"data": [{"values": [0.1, 0.2, 0.3]}, {"values": [0.4, 0.5, 0.6]}], "knbn": 2, "ef": 50}' \
    -import-path . \
    -proto proto/vector_service.proto \
    localhost:50051 vector_service.VectorService/Search

```

### gRPC CLI

The `d_celestica` binary supports an argument called `grpc_cli` which allows you to start the gRPC CLI and interact with the gRPC service running in the background. You can access this service via the exposed ports.

1.  First, run the Docker container:

```shell
docker-compose exec celestica d_celestica --grpc_cli
```
This command starts the Docker container, exposing the REST and gRPC interfaces on ports 8080 and 50051 respectively, and launches the gRPC CLI.

1.  Within the gRPC CLI, you can use the following subcommands:

-   `insert`: Insert a vector.

    Example:
    
    ```shell
    insert -k 1 -v 1.0,2.0,3.0
    ```
    
-   `search`: Search for neighbors.

    Example:

    ```shell
    search -v 1.0,2.0,3.0 -k 5 -e 200
    ```

-   `exit`: Exit the application.

For each subcommand, provide the required arguments as specified in the code snippet provided in the question. The gRPC CLI will interact with the gRPC service and display the results.


### Python Example
You can use the following Python code to interact with the REST API:

```python
import requests

# Insert a vector
data = {
    "key": "some_key",
    "vector": [0.1, 0.2, 0.3],
}
response = requests.post("http://localhost:8080/insert", json=data)
print(response.json())

# Search for similar vectors
data = {
    "query": [0.1, 0.2, 0.3],
    "knbn": 10,
    "ef": 50,
}
response = requests.post("http://localhost:8080/search", json=data)
print(response.json())
```

## Starting and testing an IPFS node

The following steps will guide you on how to start an IPFS node using `docker-compose` and test its functionality.

### Start the IPFS node

1.  Run the following command to start the IPFS node:

```bash
docker-compose up ipfs-node
```

### Check the running containers

1.  Use the following command to list the running Docker containers:

```bash
docker ps
```

### Set the container ID

1.  Find the container ID for the IPFS node in the output of the `docker ps` command, and set the `cid` environment variable:

```bash
export cid=<your_container_id>
```

Replace `<your_container_id>` with the actual container ID.

### Check the connected IPFS peers

1.  Use the following command to list the connected IPFS peers:

```bash
docker exec $cid ipfs swarm peers
```

### Add a file to IPFS

1.  Change the current directory to the IPFS staging directory:

```bash
cd $(echo $ipfs_staging)
```

1.  Create a new file called `hello` and write some content to it:

```bash
echo "hello from dockerized ipfs" > hello
```

1.  Add the `hello` file to the IPFS node:

```bash
docker exec $cid ipfs add /export/hello
```

This command will return a content identifier (CID) for the file, such as `QmcDge1SrsTBU8b9PBGTGYguNRnm84Kvg8axfGURxqZpR1`.

### Retrieve the file from IPFS

1.  Use the following command to retrieve the content of the file using the IPFS node:

```bash
docker exec $cid ipfs cat /ipfs/<your_file_cid>
```

Replace `<your_file_cid>` with the actual file CID obtained in step 7.

1.  Alternatively, you can use `curl` to retrieve the content of the file through the IPFS gateway:

```bash
curl http://localhost:8080/ipfs/<your_file_cid>
```

Replace `<your_file_cid>` with the actual file CID obtained in step 7.