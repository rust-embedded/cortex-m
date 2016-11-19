FROM ubuntu:16.04
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libcurl4-openssl-dev \
    libssh2-1
RUN curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git japaric/xargo --tag v0.2.1 --target x86_64-unknown-linux-gnu --to /usr/bin
