FROM rust:1.87

# Set working directory
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
  libpq-dev \
  pkg-config \
  build-essential

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy project files
COPY . .

# Build the app
RUN cargo build --release

# Run the app
CMD ["./target/release/user_auth"]
