FROM ubuntu:16.04
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libcurl4-openssl-dev \
    libssh2-1
RUN curl -sf "https://raw.githubusercontent.com/japaric/rust-everywhere/master/install.sh" | \
    bash -s -- --at /usr/bin --from japaric/xargo --for x86_64-unknown-linux-gnu --tag v0.1.13
