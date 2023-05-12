# Jupiter Notebook

## Project Overview

This project demonstrates how to set up a Jupyter Notebook environment with Docker Compose to run a reverse image search using Celestica.

## Prerequisites

- Docker
- Docker Compose

## Instructions

1. Create a Docker volume and network:

```
docker volume create celestica-work
docker network create celestica-shared-net
```

These will be used to share data and enable communication between the Jupyter Notebook container and the Celestica container.

2. Build the Docker image:

Navigate to the directory containing the `build_docker_image.sh` run the following commands:

```
chmod +x build_docker_image.sh
./build_docker_image.sh
```

This will build the Docker image for the Jupyter Notebook environment.

3. Start the Jupyter Notebook container:

```
docker-compose up
```

This will start the Jupyter Notebook container and attach it to the shared network and volume created earlier.

4. Access the Jupyter Notebook:

Open a web browser and navigate to the URL provided in the terminal output. This should look something like `http://127.0.0.1:8888/?token=celestica`.

5. Run the Jupyter Notebook:

In the Jupyter Notebook interface, open the notebook file (e.g., `text2image.ipynb`) and run the cells as needed.

6. Stop the Jupyter Notebook container:

To stop the Jupyter Notebook container, press `Ctrl+C` in the terminal where you ran `docker-compose up`.
