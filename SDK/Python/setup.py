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
