FROM alpine:latest 
COPY target/x86_64-unknown-linux-musl/release/mizotakhteh-api . 
RUN chmod +x ./mizotakhteh-api
CMD ["./mizotakhteh-api"]