# select build image
FROM rust:1.33 as build

# create a new empty shell project
RUN USER=root cargo new --bin telemetary
WORKDIR /telemetary

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./.env ./.env

# build for release
RUN rm ./target/release/deps/telemetary*
RUN cargo build --release

# our final base
FROM rust:1.33

# copy the build artifact from the build stage
COPY --from=build /telemetary/target/release/telemetary .

# set the startup command to run your binary
CMD ["./telemetary"]