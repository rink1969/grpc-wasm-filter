## grpc-wasm-filter

Proxy-Wasm plugin that filter grpc request.

### Building

```sh
$ cargo build --target wasm32-wasi --release
```

### Using in Envoy

This example can be run with [`docker compose`](https://docs.docker.com/compose/install/)
and has a matching Envoy configuration.

```sh
$ docker compose up
```
### grpc filter

grpc call HashData without filter:

```sh
$ grpcurl -emit-defaults -plaintext -d '{ "data": "RG1b4NPqbEcHI3ColBIzhhitX/M=" }' -proto ./protos/crypto.proto -import-path ./protos 127.0.0.1:50005 crypto.CryptoService/HashData
{
  "status": {
    "code": 0
  },
  "hash": {
    "hash": "zF1F5xUhveGShe28Q+Dc/D5R2B3xQBXCWwj+asMLg70="
  }
}

```

grpc call HashData with filter:

```sh
$ grpcurl -emit-defaults -plaintext -d '{ "data": "RG1b4NPqbEcHI3ColBIzhhitX/M=" }' -proto ./protos/crypto.proto -import-path ./protos 127.0.0.1:8000 crypto.CryptoService/HashData
ERROR:
  Code: Internal
  Message: transport: malformed grpc-status: strconv.ParseInt: parsing "4294967295": value out of range
```

because black list args 0x446d5be0d3ea6c47072370a89412338618ad5ff3 set in envoy.yaml as plugin configuration.
