FROM lukemathwalker/cargo-chef:latest AS chef

FROM chef AS planner
WORKDIR /recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /ffmpeg-stream-audio

# Build dependencies
COPY --from=planner /recipe/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --frozen --bin ffmpeg-stream-audio

FROM ghcr.io/kwheelans/container-utils:0.1 AS css
WORKDIR /app
RUN container-utils pico-css-download


# Final image
FROM linuxserver/ffmpeg:8.0.1
WORKDIR /ffmpeg-stream-audio
ENV PATH=/ffmpeg-stream-audio:$PATH \
VERBOSITY=Info

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /ffmpeg-stream-audio/target/release/ffmpeg-stream-audio /ffmpeg-stream-audio
COPY --from=css /app/css /ffmpeg-stream-audio/css
VOLUME /config

ENTRYPOINT ["ffmpeg-stream-audio", "/config/config.toml"]
