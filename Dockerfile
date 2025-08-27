FROM rust:latest
ENV HOME=/usr/src/semsimian_server
WORKDIR /usr/src/semsimian_server

# Bypass Python version check until semsimian updates pyo3 to >= 0.22.1
ENV PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

COPY . .
RUN cargo install --path .

# Entry point
CMD ["semserver"]
