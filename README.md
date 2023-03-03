# Postman issue 11625

This is a project which reproduce a reflection bug in Postman gRPC client.

The bug show up starting from an increasing `.proto`complexity.

![image](https://user-images.githubusercontent.com/22281426/222745944-8911741b-4f20-46ad-bb1e-06eb4533badd.png)


See https://github.com/postmanlabs/postman-app-support/issues/11625

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

Result from `grpcurl` call:
```shell
grpcurl -plaintext localhost:33037 describe
```

```proto
postman.api.v1.Grpc is a service:
// Postman gRPC service
service Grpc {
  // GetVersion
  rpc GetVersion ( .postman.api.v1.GetVersionRequest ) returns ( .postman.api.v1.GetVersionResponse );
  // SendBlocks
  rpc SendBlocks ( stream .postman.api.v1.SendBlocksRequest ) returns ( stream .postman.api.v1.SendBlocksResponse );
  // SendOperations
  rpc SendOperations ( stream .postman.api.v1.SendOperationsRequest ) returns ( stream .postman.api.v1.SendOperationsResponse );
}
grpc.reflection.v1alpha.ServerReflection is a service:
service ServerReflection {
  // The reflection service is structured as a bidirectional stream, ensuring
  // all related requests go to a single server.
  rpc ServerReflectionInfo ( stream .grpc.reflection.v1alpha.ServerReflectionRequest ) returns ( stream .grpc.reflection.v1alpha.ServerReflectionResponse );
}
```

## Contributing

If you would like to contribute to this project, you may fork it and submit your changes in a pull request.

## License

This project is licensed under the MIT license.

That's it! You should now have enough information to run and use this Rust project.
