# Use an official Python base image
FROM continuumio/miniconda3

# Set the working directory
WORKDIR /workspace

# Copy the environment.yml file into the container
COPY environment.yml /workspace/

# Create the conda environment based on environment.yml
RUN conda env create -f environment.yml

# Make sure the new environment is activated for subsequent commands
SHELL ["conda", "run", "-n", "myenv", "/bin/bash", "-c"]

# Install Jupyter Notebook, ipykernel, and numpy in the 'myenv' environment
RUN conda install jupyter ipykernel numpy

# Add the kernel to Jupyter
RUN python -m ipykernel install --user --name=myenv --display-name "Python (myenv)"

# Copy the notebook into the container
COPY basics_of_ray.ipynb /workspace/

# Expose the port the Jupyter Notebook server will run on
EXPOSE 8888

# Configure Jupyter Notebook to disable token and password
RUN echo "c.NotebookApp.token = ''" >> /opt/conda/envs/myenv/etc/jupyter/jupyter_notebook_config.py
RUN echo "c.NotebookApp.password = ''" >> /opt/conda/envs/myenv/etc/jupyter/jupyter_notebook_config.py
RUN echo "c.NotebookApp.open_browser = False" >> /opt/conda/envs/myenv/etc/jupyter/jupyter_notebook_config.py

# Start Ray head node and multiple workers, then Jupyter notebook
CMD ["bash", "-c", "source activate myenv && ray start --head --port=6379 --num-cpus=1 && ray start --address='auto' --num-cpus=2 && conda run -n myenv jupyter notebook --ip=0.0.0.0 --port=8888 --allow-root"]
