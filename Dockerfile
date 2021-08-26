##################
#     Builder    #
##################

FROM alpine:latest as builder

WORKDIR /usr/src/audymus-api

COPY src/ /usr/src/audymus-api/src/
COPY Cargo.toml /usr/src/audymus-api/ 
COPY Cargo.lock /usr/src/audymus-api/
COPY Rocket.toml /usr/src/audymus-api/

RUN apk add rustup

RUN rustup-init -y
RUN rustup override set nightly

ENV RUSTUP_TOOLCHAIN=nightly
ENV CARGO_TARGET_DIR=target

RUN ls

RUN cargo fetch --locked

RUN cargo build --frozen --release --all-features
# RUN cargo test --frozen --all-features
RUN ls ./target/release

#################
#     final     #
#################

FROM alpine:latest

RUN mkdir -p /home/audymus-api

RUN addgroup --system audymus-api && adduser --system audymus-api

ENV HOME=/home/audymus-api
ENV APP_HOME=/home/audymus-api/bin
ENV mkdir $APP_HOME
WORKDIR $APP_HOME

COPY --from=builder /usr/src/audymus-api/target/release $APP_HOME

RUN chown -R audymus-api:audymus-api $APP_HOME
RUN apk add fish

USER audymus-api


