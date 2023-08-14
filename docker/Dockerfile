ARG BASE_IMAGE=debian:bookworm-slim

FROM ${BASE_IMAGE} AS builder

# DEBIAN CONFIG
RUN apt-get update
# - TIME ZONE Europe/Paris
RUN apt-get install -yq tzdata
RUN ln -fs /usr/share/zoneinfo/Europe/Paris /etc/localtime
RUN dpkg-reconfigure -f noninteractive tzdata

ENV TZ=Europe/Paris
# - LANG fr_FR
RUN apt-get install -y locales locales-all
ENV LC_ALL fr_FR.UTF-8
ENV LANG fr_FR.UTF-8
ENV LANGUAGE fr_FR.UTF-8

# INSTALLATION DE CRONTAB
RUN apt-get install -yq git

WORKDIR /app/
RUN git clone https://github.com/pbillerot/silex.git

EXPOSE 8080

WORKDIR /app/silex/production
CMD ["./silex"]
# CMD ["sh", "-c", "tail -f /dev/null"]