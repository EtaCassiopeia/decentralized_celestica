FROM continuumio/miniconda3:4.10.3p1

WORKDIR /project

# Update the package index and install the required libraries
RUN apt-get update && apt-get install -y libgl1-mesa-glx unzip

RUN conda install \
    xarray \
    netCDF4 \
    bottleneck \
    numpy \
    pandas \
    matplotlib \
    jupyterlab

RUN pip install opencv-python-headless
RUN pip install git+https://github.com/openai/CLIP.git

COPY ./notebooks /project/notebooks
COPY ./data /project/data
CMD ["jupyter-lab","--ip=0.0.0.0","--no-browser","--allow-root"]