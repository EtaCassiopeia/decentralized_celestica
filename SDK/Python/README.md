# Celestica Python SDK 

To generate a Python SDK to interact with your gRPC service, follow these steps:

1. Install `grpcio` and `grpcio-tools` Python packages.

You'll need these packages to generate and use gRPC Python code. Install them using pip:

```bash
pip install grpcio grpcio-tools
```

2. Generate Python code from the proto file.

Create a new directory for your package:

```bash
mkdir celestica
```

Run the following command to generate Python code from your `vector_service.proto` file:

```bash
python -m grpc_tools.protoc -I. --python_out=celestica --grpc_python_out=celestica --proto_path=../../proto/ vector_service.proto
```

This command will generate two files: `vector_service_pb2.py` and `vector_service_pb2_grpc.py`. The first file contains the message classes, and the second one contains the client and server classes.

Note: I had to make a small change to the generated code to make it work. In the `vector_service_pb2_grpc.py` file, I had to change the following line:

```python
from . import vector_service_pb2 as vector__service__pb2
```

3. Create a Python client using the generated code.

To create a library that hides the complexities from the developer, you can create a Python package with a simple interface. Here's an example of how to structure your package:

a) Create a `__init__.py` file inside the `celestica` directory to make it a Python package. You can leave it empty.
b) Create a `client.py` file inside the `celestica` directory with the following code:

```python
import grpc
from . import vector_service_pb2
from . import vector_service_pb2_grpc

class CelesticaClient:
    def __init__(self, url):
        self.channel = grpc.insecure_channel(url)
        self.client = vector_service_pb2_grpc.VectorServiceStub(self.channel)

    def insert(self, data, ids):
        float_arrays = [vector_service_pb2.FloatArray(values=d) for d in data]
        request = vector_service_pb2.InsertRequest(data=float_arrays, ids=ids)
        return self.client.Insert(request)

    def search(self, data, knbn, ef):
        float_arrays = [vector_service_pb2.FloatArray(values=d) for d in data]
        request = vector_service_pb2.SearchRequest(data=float_arrays, knbn=knbn, ef=ef)
        response = self.client.Search(request)
        return response.neighbours
```

## Publishing the Python SDK
To package your library using `setuptools` and make it available for use in a Jupyter Notebook, follow these steps:

1. Create a `setup.py` file in the root directory of your project (the same directory containing the `celestica` folder). Add the following content to the file:

```python
from setuptools import setup, find_packages

setup(
    name="celestica",
    version="0.1",
    packages=find_packages(),
    install_requires=[
        "grpcio>=1.41.0",
        "grpcio-tools>=1.41.0",
    ],
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
    ],
)
```

This script defines the package metadata and dependencies.

2. Create a `README.md` file in the root directory of your project. This file should contain a description of your library and instructions on how to use it.

3. (Optional) Create a `LICENSE` file in the root directory of your project. This file should contain the text of the open-source license you'd like to use. In the `setup.py` example above, we specified the MIT License.

4. Install the library in "editable" mode:

In the root directory of your project (where `setup.py` is located), run the following command:

```bash
pip install -e .
```

This command installs the library in "editable" mode, meaning that any changes you make to the library will be reflected immediately, without needing to reinstall the package.


Once you are satisfied with your library, you can distribute it to PyPI (the Python Package Index) to make it easily installable with `pip` for other users. For more information on how to distribute your package to PyPI, refer to the [official Python packaging guide](https://packaging.python.org/tutorials/packaging-projects/).



