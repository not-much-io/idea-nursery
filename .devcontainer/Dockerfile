FROM rust:1

# To check status of tools by nightly versions see: https://rust-lang.github.io/rustup-components-history/index.html
ENV TOOLCHAIN_VERSION=nightly-2020-06-06

# Project deps
RUN apt-get update \
    && apt-get -y install pkg-config libssl-dev make lld

# Use a much faster linker... if it works
ENV RUSTFLAGS "-C link-arg=-fuse-ld=lld"

# Install nightly toolchain
RUN rustup update \
    && rustup toolchain install ${TOOLCHAIN_VERSION} \
    && rustup default ${TOOLCHAIN_VERSION} \
    && rustup component add rust-analysis rust-src rustfmt clippy

# Editor setup
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git iproute2 procps lsb-release \
    #
    # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
    && groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # Project deps
    && apt-get install net-tools \
    #
    # [Optional] Add sudo support for the non-root user
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME\
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    #
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

USER vscode

ENV DEBIAN_FRONTEND=dialog
