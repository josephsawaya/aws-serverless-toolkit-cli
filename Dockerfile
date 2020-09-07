FROM rust

COPY . /app
WORKDIR /app
RUN apt-get update
RUN apt-get install -y git
RUN apt-get install -y gcc-mingw-w64-x86-64
RUN apt-get install -y clang cmake
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add x86_64-apple-darwin
RUN ./osxcross_setup.sh
ENTRYPOINT ./build.sh
