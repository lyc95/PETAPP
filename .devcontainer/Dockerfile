FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

ARG DEBIAN_FRONTEND=noninteractive

# System packages (postgresql-client for psql CLI, libpq-dev for sqlx)
RUN apt-get update && apt-get install -y \
    openjdk-17-jdk-headless \
    unzip \
    xz-utils \
    android-tools-adb \
    postgresql-client \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Rust (system-wide so both root and vscode user can use it)
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain stable \
    --no-modify-path \
    && chmod -R a+w /usr/local/rustup /usr/local/cargo

WORKDIR /workspace
