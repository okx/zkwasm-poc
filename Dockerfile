FROM ubuntu:22.10

RUN apt update && apt install -y curl gcc-multilib clang binaryen

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

SHELL ["/bin/bash", "-c"]

RUN source $HOME/.cargo/env && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:$PATH"


