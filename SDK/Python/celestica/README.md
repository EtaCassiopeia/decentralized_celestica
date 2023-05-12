# Celestica Python SDK

## Using the Python SDK

You can use the `CelesticaClient` class to interact with the gRPC service in a more convenient way. Here's an example of how to use the library:

```python
from celestica.client import CelesticaClient

def main():
    url = "localhost:50051"  # Replace with the address and port of your gRPC server
    client = CelesticaClient(url)

    # Insert data
    data = [[0.1, 0.2, 0.3]]
    ids = [1]
    client.insert(data, ids)

    # Search data
    knbn = 1
    ef = 10
    neighbours = client.search(data, knbn, ef)
    print("Neighbours:", neighbours)

if __name__ == "__main__":
    main()
```

## Use the library in Jupyter Notebook:

You can launch a Jupyter Notebook, and you'll be able to import and use the library:

```python
from celestica.client import CelesticaClient

url = "localhost:50051"  # Replace with the address and port of your gRPC server
client = CelesticaClient(url)

# Insert data
data = [[0.1, 0.2, 0.3]]
ids = [1]
client.insert(data, ids)

# Search data
knbn = 1
ef = 10
neighbours = client.search(data, knbn, ef)
print("Neighbours:", neighbours)
```