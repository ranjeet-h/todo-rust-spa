# 🛠️ Setup & Deployment Guide

This guide explains how to build your finalized "RAM-only" binary, package it into a Docker image, and share it with other computers.

---

## 1. Build the Production Image

Before sharing, you must build the single production binary that includes your optimized frontend.

```bash
docker build -t todoapp:latest .
```

---

## 2. Share with a Secondary Computer

You have two main ways to move this image to another machine.

### Method A: The Registry (Best for Cloud/Teams)

1. **Tag and Push:**
   ```bash
   docker tag todoapp:latest your-username/rust-todo-app:v1
   docker push your-username/rust-todo-app:v1
   ```
2. **On the shared computer:** Only the `docker-compose.yml` is needed.

### Method B: The File Transfer (Best for Offline/USB)

1. **Save to a file:**
   ```bash
   docker save -o rust-todo-app.tar todoapp:latest
   ```
2. **Transfer:** Copy both `rust-todo-app.tar` and `docker-compose.yml` to the second computer.
3. **Load:** On the second computer, run:
   ```bash
   docker load -i rust-todo-app.tar
   ```

---

## 3. Running on the Secondary Computer

Ensure the second computer has **Docker Desktop** installed.

1. **Place the files:** Create a new folder and put the `docker-compose.yml` (and the `.tar` file if using Method B) inside it.
2. **Start the environment:**
   ```bash
   docker compose up -d
   ```

> [!NOTE]
> Docker Compose handles the MongoDB setup and network linking automatically. You don't need to install Rust, Node, or Mongo on the second computer.

---

## 4. Accessing from your Home Network

Once the app is running on one computer, you can access it from any device (phone, tablet, laptop) on the same Wi-Fi:

1. Find the local IP address of the host (e.g., `192.168.1.50`).
2. Open `http://192.168.1.50:8080` on the other device.
