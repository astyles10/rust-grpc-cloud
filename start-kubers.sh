#!/bin/bash

# Note minikube node is persistent

REGISTRY_EXISTS=$(docker images -q "registry")
if [ -z "$REGISTRY_EXISTS" ]; then
  # Init local registry
  docker run -d -p 5000:5000 --restart=always --name registry registry:2
fi

# Build docker image and deploy to local registry
docker start registry
docker build . -t localhost:5000/rust-grpc-server -f ./dockerfiles/test_rpc_server.dockerfile
docker push localhost:5000/rust-grpc-server

# Need to init and start new container with --insecure-registry if not done before, otherwise https is required
minikube delete
minikube start --insecure-registry="192.168.49.1:5000"

# Deploy and expose grpc container
kubectl create deployment hello-node --image=192.168.49.1:5000/rust-grpc-server:latest
kubectl get deployments.apps
kubectl get pods
kubectl expose deployment hello-node --type=LoadBalancer --port 50051 --target-port 50051
MINIKUBE_IP="$(minikube ip)"
# How to determine port number?
ncat -z $MINIKUBE_IP 31935

# Cleanup
kubectl delete deployments.apps hello-node
kubectl delete svc hello-node

# kubectl create -f deploy.yaml