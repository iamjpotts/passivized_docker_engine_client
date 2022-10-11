
# Docker Registry Authentication

Some tests configure and use a Docker registry server that enforces username and password authentication.

The Docker Registry supports Apache `htpasswd` files, and a static `htpasswd` file is pre-generated and
stored with the source code.


## How Password Is Generated

The credentials used by the tests are not secrets. The script below was used to generate the password
file. It does not need to be re-run unless there is a future software incompatibility.

* [generate.sh](generate.sh)