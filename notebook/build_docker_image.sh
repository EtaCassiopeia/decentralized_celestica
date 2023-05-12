#!/bin/bash

# Check if the Dockerfile exists in the current directory
if [ ! -f "Dockerfile" ]; then
  echo "Dockerfile not found. Please make sure you are in the Rust project directory."
  exit 1
fi

docker build -t jupyterlab:latest .