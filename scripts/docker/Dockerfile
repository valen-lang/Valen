# Build:
#   docker build -t valec --file scripts/docker/Dockerfile .
# Run (compile a Vale program):
#   docker run --rm -v "$PWD:/work" valec build mymod=/work/hello.vale --output_dir /work/target
# Rebuild after editing sources without re-pulling LLVM:
#   docker build -t valec --file scripts/docker/Dockerfile --build-arg RECOPY_DUMMY=$(date +%s) .

FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive

# Base toolchain + LLVM 21 from apt.llvm.org
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        ca-certificates curl wget gnupg lsb-release \
        git cmake build-essential pkg-config \
 && wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
 && echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-21 main" > /etc/apt/sources.list.d/llvm.list \
 && apt-get update \
 && apt-get install -y --no-install-recommends \
        llvm-21 llvm-21-dev clang-21 libpolly-21-dev \
        zlib1g-dev libzstd-dev libcurl4-openssl-dev \
 && rm -rf /var/lib/apt/lists/*

# Rust toolchain (rustup, default nightly — the frontend needs nightly features)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

ENV LLVM_CONFIG=/usr/bin/llvm-config-21

# Copy sources and build
ARG RECOPY_DUMMY=0
COPY . /Vale
WORKDIR /Vale

RUN cargo build --manifest-path Cargo.toml

# Drop the built valec onto $PATH for the entrypoint
RUN cp target/debug/valec /usr/local/bin/valec

WORKDIR /work
ENTRYPOINT ["valec"]
