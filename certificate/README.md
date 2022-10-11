
# Certificate Generation and Persistence for Tests

While it is possible to automagically generate certificates using the [rcgen](https://github.com/est31/rcgen)
crate, that library (as of version 0.10.0) has a dependency on the [ring](https://github.com/briansmith/ring)
crate, which has a non-trivial set of licenses.

To avoid potential problems with the licenses applying to `ring`, `rcgen` is not used to generate
test certificates.

## Generating and Persisting Test Certificates

The tests require a self-signed certificate authority, and a private key / server certificate pair signed by
that same CA.

Certificates are defined in json files, generated using [cfssl](https://github.com/cloudflare/cfssl), and
committed into git.

### Install `cfssl` and Re-generate Certificates

    $ ./download-cfssl.sh
    $ ./create-ca.sh
    $ ./create-host.sh

Note: You should not have to regenerate any certificates unless they expire, the ciphers become insecure,
or the certificates otherwise become rejected by future versions of cryptography libraries.

### Definitions

* [profiles.json](profiles.json)
* [ca.json](ca.json)
* [testregistry.locallan.json](testregistry.locallan.json)

### Subnets and IP Addresses

Because the certificates are static and persisted with the source code, and an IP address is required for
the Subject Alternative Name due to not having any reasonable way to establish a DNS server during testing,
an arbitrary fixed IP address is defined for the Docker registry that is temporily stood up during tests.

The `hosts` entry of `testregistry.locallan.json` contains this IP address.

The tests also create a Docker network with a subnet in which the registry IP address is valid. Like the
registry host IP address, the subnet and its gateway are arbirary and static.

They are not likely to collide with a CI server, but it is not impossible to have a collision.
