#!/bin/bash

# Check if the Dockerfile exists in the current directory
if [ ! -f "Dockerfile" ]; then
  echo "Dockerfile not found. Please make sure you are in the Rust project directory."
  exit 1
fi

# Check if the Cargo.toml file exists in the current directory
if [ ! -f "Cargo.toml" ]; then
  echo "Cargo.toml not found. Please make sure you are in the Rust project directory."
  exit 1
fi

# Check if the docker-compose.yml file exists in the current directory
if [ ! -f "docker-compose.yml" ]; then
  echo "docker-compose.yml not found. Please make sure you are in the Rust project directory."
  exit 1
fi

# Extract the version number from Cargo.toml
version=$(grep '^version' Cargo.toml | head -1 | awk -F' = ' '{print $2}' | tr -d '"')

# Set the Docker image name
image_name="celestica"

# Build the Docker image with the extracted version number as a tag and tag it as "latest"
docker build -t "${image_name}:${version}" -t "${image_name}:latest" .

echo "Docker image built and tagged as ${image_name}:${version} and ${image_name}:latest"

# Export the CELESTICA_VERSION environment variable
export CELESTICA_VERSION=${version}
