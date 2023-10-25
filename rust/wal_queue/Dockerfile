FROM alpine:3.17

RUN apk add --no-cache \
    bash \
    ca-certificates \
    curl \
    jq \
    openssl \
    openssl-dev \
    rust \
    cargo \
    build-base \
    linux-headers \
    && rm -rf /var/cache/apk/*

RUN mkdir -p /usr/src/app

#install jdk
RUN apk add --no-cache openjdk11 --repository=http://dl-cdn.alpinelinux.org/alpine/edge/community


WORKDIR /usr/src/app

COPY . .

CMD [ "bash" ]
