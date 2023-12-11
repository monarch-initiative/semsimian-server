FROM rust:latest
ENV HOME=/usr/src/semsimian_server
WORKDIR /usr/src/semsimian_server

COPY . .
RUN cargo install --path .

CMD ["semserver"]
