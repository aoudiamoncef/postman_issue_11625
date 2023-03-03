# Postman issue 11625

This is a project which reproduce a reflection bug in Postman gRPC client.

## Prerequisites

Before running this project, you will need the following:
- Rust programming language installed on your system
- Cargo package manager installed on your system
- Protoc compiler installed on your system

## Installation

To install the project, please follow these steps:

Clone the repository to your local machine using git clone command.
Navigate to the root of the project.

Run the following command in your terminal:

```shell
cargo build
```

Once the build is finished, run the following command to start the application:

```shell
cargo run
```

## Usage

Use Postman client and launch a server reflection. 

Decode the generated proto file descriptor:
```shell
protoc < src/api.bin --decode=google.protobuf.FileDescriptorSet google/protobuf/descriptor.proto > descriptor.txt
```

## Contributing

If you would like to contribute to this project, you may fork it and submit your changes in a pull request.

## License

This project is licensed under the MIT license.

That's it! You should now have enough information to run and use this Rust project.