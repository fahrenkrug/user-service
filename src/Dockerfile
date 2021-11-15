# Source: https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/

FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin user-service
WORKDIR ./user-service
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/user-service*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/user-service/target/x86_64-unknown-linux-musl/release/user-service ${APP}/user-service

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./user-service"]