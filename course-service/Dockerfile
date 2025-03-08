FROM golang:1.21

WORKDIR /app

# Copy go mod and sum files
COPY go.mod go.sum* ./

# Download dependencies
RUN go mod download

# Copy source code
COPY . .

# Build the application
RUN go build -o main .

# Expose port
EXPOSE 8080

# Command to run the executable
CMD ["./main"]