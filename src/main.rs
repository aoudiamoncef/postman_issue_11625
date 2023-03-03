use std::pin::Pin;

use postman_issue_11625::postman::api::v1::{
    self as grpc, grpc_server::GrpcServer, FILE_DESCRIPTOR_SET,
};

use tonic::codegen::futures_core;
use tonic_web::GrpcWebLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:33037".parse().unwrap();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;
    let svc = GrpcServer::new(PostmanService::default());
    println!("PostmanServer listening on {}", addr);
    tonic::transport::Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(reflection_service)
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Default)]
pub struct PostmanService {}

#[tonic::async_trait]
impl grpc::grpc_server::Grpc for PostmanService {
    async fn get_version(
        &self,
        request: tonic::Request<grpc::GetVersionRequest>,
    ) -> Result<tonic::Response<grpc::GetVersionResponse>, tonic::Status> {
        Ok(tonic::Response::new(grpc::GetVersionResponse {
            id: request.into_inner().id,
            message: Some(grpc::get_version_response::Message::Result("1".to_owned())),
        }))
    }

    type SendBlocksStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<grpc::SendBlocksResponse, tonic::Status>>
                + Send
                + 'static,
        >,
    >;

    async fn send_blocks(
        &self,
        _request: tonic::Request<tonic::Streaming<grpc::SendBlocksRequest>>,
    ) -> Result<tonic::Response<Self::SendBlocksStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("not implemented"))
    }

    type SendOperationsStream = Pin<
        Box<
            dyn futures_core::Stream<Item = Result<grpc::SendOperationsResponse, tonic::Status>>
                + Send
                + 'static,
        >,
    >;

    async fn send_operations(
        &self,
        _request: tonic::Request<tonic::Streaming<grpc::SendOperationsRequest>>,
    ) -> Result<tonic::Response<Self::SendOperationsStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("not implemented"))
    }
}
