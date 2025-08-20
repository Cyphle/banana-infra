#!/bin/bash

helm upgrade --install external-secrets  external-secrets/external-secrets \
    -n external-secrets \
    --create-namespace \
    --set installCRDs=true