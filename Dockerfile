FROM golang:1.21-alpine

WORKDIR /app
COPY . .
RUN ls
RUN go build -o ../imgpack

WORKDIR /
RUN rm -rf /app

EXPOSE 3000

CMD ["./imgpack", "--serve"]
