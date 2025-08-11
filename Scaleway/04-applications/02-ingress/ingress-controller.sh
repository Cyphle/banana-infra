#!/bin/bash

kubectl create namespace ingress-controller
# Namespace original du tuto: --namespace ingress-nginx --create-namespace
helm install ingress-nginx ingress-nginx/ingress-nginx --namespace ingress-controller
# Namespace original du cert-manager dans le tuto: --namespace cert-manager --create-namespace
helm install cert-manager jetstack/cert-manager --namespace ingress-controller --version v1.12.0 --set installCRDs=true
kubectl apply -f cluster-issuer.yaml -n ingress-controller