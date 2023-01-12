# gRPC blocking bug

1. Run the executable `cargo run`. This should work and HTTP2 pings should flow (check w/ wireshark).
2. Run `grpcurl -proto proto/echo.proto -plaintext localhost:8083 grpc.examples.echo.Echo/Echo`. This completely kills
   the server.