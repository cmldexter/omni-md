# Stage 1: Build Vite React App
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package manifests and install dependencies
COPY package*.json ./
RUN npm install

# Copy source and configurations
COPY . .

# Generate the highly optimized static production bundle
RUN npm run build

# Stage 2: Serve natively via Nginx
FROM nginx:alpine

# Copy the bundled assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose standard web port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
