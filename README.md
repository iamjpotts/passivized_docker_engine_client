# Docker Engine Client

A client library for connecting to a Docker Engine, typically a local instance at `/var/run/docker.sock`. The library
can pull and tag images, create and start containers, get their output.

Useful for building automated tests that require a server app, such as a Rust application that uses a Postgres database.
The tests can use this library to create, start, and use a containerized app, and delete the container when the tests
are finished.

## Examples

### Hello, World

[examples/hello.rs](examples/hello.rs)

Pulls and runs the famous `hello-world` Docker container, waits for it to finish, and logs its output.

### BusyBox File Listing

[examples/busybox.rs](examples/busybox.rs)

Similar to Hello World, except runs a file listing command, and logs the file listing.

### Image List

[examples/image-list.rs](examples/image-list.rs)

Gets a list of all images currently stored in the Docker Engine, and lists the first ten images that have a name (tag).

### Nginx

[examples/nginx.rs](examples/nginx.rs)

Pulls and runs a `nginx` image, waits for it to start by waiting for port `80` to be in service, then gets and displays
the html content served by nginx.


## Examples in Tests

### Complex Multi-Container Configuration with Custom Network

[tests/test_registry.rs](tests/test_registry.rs)

* Docker-in-Docker (DIND)
* Extracting files created within a container
* Running a private registry, secured with TLS and htpasswd authentication
* Pushing and pulling with the private registry

### Health Check

[tests/test_health_check.rs](tests/test_health_check.rs)

Adds a failing health check and gets its status.

### Pruning Unused Volumes

[tests/test_volumes_prune.rs](tests/test_volumes_prune.rs)

## Platform Support

### Linux

The library is fully featured on Linux.

### Mac

Mac support is experimental. No features are disabled, but some automated tests are disabled.

On Macs, the custom local networks that Docker creates for containers are not easily reachable from the host network.
Therefore, many automated tests are disabled until a workaround is implemented.

### Windows

Windows support is experimental. Some features are conditional compiled away because Docker Engine does not support them
on Windows. The examples will not work on Windows in a default configuration.

Additionally, on Windows, Docker Engine only listens on a named pipe, and must be reconfigured to enable an HTTP
listener. Automated tests enable an insecure listener on port 2375. If you enable HTTP in production, you will want to
follow the Docker documentation to enable a TLS-secured listener on port 2376.

This library supports connecting to TLS-secured endpoints, including with self-signed certificate authorities.

_Connect to HTTP Endpoint_

See [.github/workflows/ci.yml](.github/workflows/ci.yml) for an example.

After reconfiguring and restarting Docker Engine, set the `DOCKER_HOST` environment variable
and use `DockerEngineClient::new` in your code.

Alternatively, instead of an environment variable, use `DockerEngineClient::with_server("http://localhost:2375")` which
still requires reconfiguration of Docker Engine.

_Connect to HTTP Endpoint with TLS (HTTPS)_

Connect with `DockerEngineClient::with_tls_config("https://localhost:2376", tls_config)` where `tls_config` is an
instance of `native_tls::TlsConnector` you configured with the self-signed certificate authority that was used to sign
the certificate that you configured Docker Engine to use for TLS.

If you went to the trouble of creating a server TLS certificate trusted by a CA your clients already trust, such as a
corporate CA or public CA, you can set the `DOCKER_HOST` environment variable and use `DockerEngineClient::new`
without having to explicitly set any `TlsConnector` configuration.

## Testing Infrastructure

[CI](.github/workflows/ci.yml) uses GitHub Actions.

* [Password file for Temporary Docker Registry](apache/README.md)
* [Certificates for Temporary Docker Registry](certificate/README.md)
