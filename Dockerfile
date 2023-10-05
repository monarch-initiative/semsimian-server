FROM rust:latest
ENV HOME=/usr/src/semsimian_server
WORKDIR /usr/src/semsimian_server

RUN mkdir -p $HOME/.data/oaklib && \
    wget https://data.monarchinitiative.org/monarch-kg-dev/latest/phenio.db.gz -O $HOME/.data/oaklib/phenio.db.gz && \ 
    gunzip $HOME/.data/oaklib/phenio.db.gz

COPY . .
RUN cargo install --path .

CMD ["semserver"]
