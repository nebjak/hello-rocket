# Hello Rocket

This is a simple example of a Rocket web server. It serves a single route, `/`, and returns a simple string.
It can be used as a starting point for a new Rocket project.

## Running the server

To run the server, execute the following command:

```sh
cargo run
```

This will start the server at `http://localhost:8080`.

## Docker

To build a Docker image, execute the following command:

```sh
docker build -t rocket-hello .
```

To run the Docker image, execute the following command:

```sh
docker run -p 8080:8080 rocket-hello
```

This will start the server at `http://localhost:8080`.

## License

This project is licensed under the MIT license.


