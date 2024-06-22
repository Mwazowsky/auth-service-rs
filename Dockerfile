FROM rust:1.78.0 AS base

ENV PROTOC_VERSION=23.3
ENV PROTOC_ZIP=protoc-${PROTOC_VERSION}-linux-x86_64.zip

# Install necessary packages and dependencies
RUN apt-get update && \
    apt-get install -y unzip libpq-dev netcat-openbsd net-tools pkg-config libssl-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Download and install protoc
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/$PROTOC_ZIP && \
    unzip -o $PROTOC_ZIP -d /usr/local bin/protoc && \
    unzip -o $PROTOC_ZIP -d /usr/local 'include/*' && \
    rm -f $PROTOC_ZIP

# Install Diesel CLI with PostgreSQL support
RUN cargo install diesel_cli --no-default-features --features postgres --verbose

# Additional steps (if any) should go here
WORKDIR /usr/src/app