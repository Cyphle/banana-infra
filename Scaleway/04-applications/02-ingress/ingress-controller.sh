#!/bin/bash

# TODO: Il faut changer le namespace et virer le --create-namespace
# Il faut aussi regrouper les trucs dans le même namespace genre 'ingress controller'
# Il faut aussi mettre le cluster issuer dans ce namespace
# La définition de l'ingress est à mettre dans le projet banana-front

# Création de l'ingress controller
helm install ingress-nginx ingress-nginx/ingress-nginx --namespace ingress-nginx --create-namespace

# Le cert manager qui va générer les certificats SSL Let's encrypt
helm install cert-manager jetstack/cert-manager --namespace cert-manager --create-namespace --version v1.12.0 --set installCRDs=true
# To uninstall `helm uninstall cert-manager -n cert-manager`


# Pour check
# kubectl get cert -n banana
# kubectl describe certificate banana-tls -n banana