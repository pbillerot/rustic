# Un container très simple :
# la dernière version allégée de debian
# et l'application compilée par ailleurs dans le répertoire app

ARG BASE_IMAGE=debian:bookworm-slim

FROM ${BASE_IMAGE} AS builder

# DEBIAN CONFIG
RUN apt-get update

WORKDIR /app/
COPY . .

CMD ["./silex"]

# exemple de commande qui met le container en attente
# CMD ["sh", "-c", "tail -f /dev/null"]

# l'expose est inutile
# EXPOSE 8080