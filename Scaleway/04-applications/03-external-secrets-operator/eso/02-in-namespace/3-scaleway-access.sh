#!/bin/bash

# Charger les variables d'environnement depuis le fichier .env
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo ".env introuvable. Veuillez créer le fichier .env avec SCW_ACCESS_KEY et SCW_SECRET_KEY."
    exit 1
fi

echo -n "$SCW_ACCESS_KEY" > ./access-key
echo -n "$SCW_SECRET_KEY" > ./secret-access-key
kubectl create secret -n banana generic scwsm-secret --from-file=./access-key --from-file=./secret-access-key