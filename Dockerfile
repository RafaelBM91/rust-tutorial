FROM    rust:latest
RUN     rustup default nightly
RUN     cargo install diesel_cli --no-default-features --features postgres
WORKDIR /home/myapp
ENV     CARGO_HOME=/home/myapp/.cargo
