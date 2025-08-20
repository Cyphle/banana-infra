# Stage 1: Build
FROM ubuntu:22.04 AS builder

# Installer les dépendances de build
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Installer Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers de configuration Cargo
COPY Cargo.toml Cargo.lock ./

# Copier le code source
COPY src ./src

# Build de l'application
RUN cargo build --release

# Stage 2: Runtime
FROM ubuntu:22.04

# Installer les dépendances runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Créer un utilisateur non-root
RUN groupadd -g 1000 appgroup && \
    useradd -u 1000 -g appgroup -s /bin/bash -m appuser

# Créer le répertoire de travail
WORKDIR /app

# Copier le binaire depuis le stage de build
COPY --from=builder /app/target/release/example-app /usr/local/bin/example-app

# S'assurer que le binaire est exécutable
RUN chmod +x /usr/local/bin/example-app

# Changer la propriété vers l'utilisateur non-root
RUN chown -R appuser:appgroup /app

# Changer vers l'utilisateur non-root
USER appuser

# Exposer le port
EXPOSE 8080

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /usr/local/bin/example-app --health || exit 1

CMD ["example-app"]