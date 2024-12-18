#!/bin/bash

# this script exists to create the `logs` directory to store log files
# log files are created and named based on the date for each day.
# logs are saved in JSON formats for easy extraction


# switch to parent directory
cd ..

LOGS_DIR="logs"
# Check if the logs directory already exists
if [ -d "$LOGS_DIR" ]; then
    echo "The directory '$LOGS_DIR' already exists."
else
    # Create the logs directory
    mkdir "$LOGS_DIR"
    echo "The directory '$LOGS_DIR' has been created."
fi