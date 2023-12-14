#!/bin/bash

docker build --no-cache . -f ./dockerfiles/test_rpc_server.dockerfile --tag test_rpc_server:latest

docker compose up -d
