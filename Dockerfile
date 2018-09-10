FROM rustlang/rust:nightly-slim
WORKDIR /dino-cave
COPY . /dino-cave
RUN cargo build --release

FROM debian:9-slim
WORKDIR /dino-cave
COPY --from=0  /dino-cave/target/release/dino-cave .
COPY profiles.json /tmp/
CMD ["./dino-cave"]  
