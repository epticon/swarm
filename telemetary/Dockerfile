#this is temporary, don't depend on rust image for the raspberry pi
FROM rust
# RUN  apk update \                                                                                                                                                                                                     
#   && apk add ca-certificates wget \
#   && update-ca-certificates \
#   && apk add openssl
WORKDIR /bin
ENTRYPOINT ["target/debug/telemetary"]