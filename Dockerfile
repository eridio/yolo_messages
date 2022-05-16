FROM rust as builder

RUN apt-get update -y
RUN apt install libssl-dev
RUN apt install -y clang llvm-dev libclang-dev

COPY ./src /home/src/
COPY ./Cargo.toml ./home/Cargo.toml
COPY ./.env /home/.env
EXPOSE 8086

WORKDIR /home/


RUN cargo build --release

RUN  cp ./target/release/yolo_message /bin/yolo_message


FROM gcr.io/distroless/cc-debian10

COPY --from=builder --chown=1:1 ${HOME}/bin/yolo_message  /app/main
COPY --from=builder --chown=1:1 /home/.env app/.env
EXPOSE 8086
WORKDIR /app
USER 1000
CMD [ "./main" ]
