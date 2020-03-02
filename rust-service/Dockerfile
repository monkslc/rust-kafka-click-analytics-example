FROM fedora:32 as builder

RUN dnf install -y rust cargo
RUN dnf install -y gcc-c++
RUN dnf install -y make
RUN dnf install -y cmake

# RUN export CXX=/usr/bin/gcc

WORKDIR /usr/src/rust-service

# create an empty shell project to download dependencies
# this is so we don't invalidate the cache and have to reload the dependencies
# when just changing source code
RUN USER=root cargo init

# install the dependencies
COPY ./Cargo.* ./
RUN cargo build
RUN rm -r src/

# copy source tree
COPY ./src ./src

# build
RUN cargo build

# CMD ["./target/release/rust-service"]

FROM fedora:32

COPY --from=builder /usr/src/rust-service/target/debug/rust-service .

CMD ["./rust-service"]
