FROM golang:1.22-alpine

# Set the Current Working Directory inside the container
WORKDIR /app

# Copy the Go module files
COPY go.mod .

# Copy the static files and Go source code
COPY static ./static
COPY apps/tank-game/assets/ ./static/assets/
COPY apps/tank-game/assets/ ./static/apps/tank-game/assets/
COPY main.go .

# Build the Go app
RUN go build -o main .

# Expose port 8080 to the outside world
EXPOSE 8080

# Command to run the executable
CMD ["./main"]
