FROM rust:1.61.0
RUN useradd -ms /bin/bash newuser
WORKDIR /home/src/app
COPY . .
RUN cargo build --release
USER newuser
CMD target/release/wik-dps-tp01
