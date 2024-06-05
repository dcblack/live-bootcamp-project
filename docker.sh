#!/bin/bash

function Realpath()
{
  if [[ $# == 0 ]]; then set - .; fi
  # shellcheck disable=SC2016
  local PERLSCRIPT='$p=abs_path(join(q( ),@ARGV));print $p if -e $p'
  /usr/bin/env perl '-MCwd(abs_path)' -le "${PERLSCRIPT}" "$*"
}

# Works in ZSH and BASH
# shellcheck disable=SC2154
if [[ -n "${ZSH_VERSION}" ]]; then
  SETUP_PATH="$(Realpath "$0")"
  Project_setup "${SETUP_PATH}" "$@"
else
  SETUP_PATH="$(Realpath "${BASH_SOURCE[0]}")"
  Project_setup "${SETUP_PATH}" "$@"
fi

export PROJECT_DIR

# Define the location of the .env file (change if needed)
ENV_FILE="${PROJECT_DIR}/auth-service/.env"

# Check if the .env file exists
if ! [[ -f "${ENV_FILE}" ]]; then
  echo "Error: .env file not found!"
  exit 1
fi

# Read each line in the .env file (ignoring comments)
while IFS= read -r line; do
  # Skip blank lines and lines starting with #
  if [[ -n "${line}" ]] && [[ "${line}" != \#* ]]; then
    # Split the line into key and value
    key=$(echo "${line}" | cut -d '=' -f1)
    value=$(echo "${line}" | cut -d '=' -f2-)
    # Export the variable
    export "${key}=${value}"
  fi
done <"${ENV_FILE}"

# Run docker-compose commands with exported variables
docker-compose build
docker-compose up
