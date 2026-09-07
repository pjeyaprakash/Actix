

# Build Image
    -- docker build -t billing-backend:1.0.0 .

# Save Image as .tar
    -- docker save billing-backend:1.0.0 -o billing-backend-1.0.0.tar

# Load Image from Another machine without DockerHub
    -- docker load -i billing-backend-1.0.0.tar

# Up the Docker Compose
    -- docker compose up -d

# Down the Docker Compose
    -- docker compose down -v