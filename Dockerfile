# Base image for PHP
FROM php:8.2-apache AS php-base

# Install PHP dependencies
RUN apt-get update && apt-get install -y \
    curl \
    zip \
    unzip \
    git \
    libpq-dev \
    libonig-dev \
    libxml2-dev \
    libzip-dev \
    libpng-dev \
    libjpeg-dev \
    libfreetype6-dev \
    libssl-dev \
    && docker-php-ext-configure gd \
    && docker-php-ext-install pdo pdo_mysql mbstring exif pcntl bcmath gd zip

# Install Composer
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer

# Base image for Rust
FROM rust:1.86 AS rust-base

# Install trunk
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown


# Set working directory
WORKDIR /app

# Copy Rust project files
COPY ./frontend/. ./frontend
WORKDIR /app/frontend

# Build frontend using trunk
RUN trunk build --release

# Final image for PHP with frontend
FROM php-base AS final

# Set working directory
WORKDIR /var/www/html

# Copy PHP application files
COPY --from=rust-base /app/public /var/www/html/public
COPY ./ /var/www/html

RUN chown -R www-data:www-data /var/www/html
RUN chmod -R 755 /var/www/html

# Install PHP dependencies
RUN composer install --no-dev --optimize-autoloader

RUN rm /var/www/html/.htaccess
