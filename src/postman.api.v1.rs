/// SendBlocksRequest holds parameters to SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksRequest {
    /// string field
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// object field
    #[prost(string, tag = "2")]
    pub block: ::prost::alloc::string::String,
}
/// SendBlocksResponse holds response from SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksResponse {
    /// string field
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// object field
    #[prost(oneof = "send_blocks_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_blocks_response::Message>,
}
/// Nested message and enum types in `SendBlocksResponse`.
pub mod send_blocks_response {
    /// object field
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        Result(super::BlockResult),
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Block response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResult {
    /// string field
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// SendOperationsRequest holds parameters to SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsRequest {
    /// string field
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// object field
    #[prost(string, tag = "2")]
    pub operation: ::prost::alloc::string::String,
}
/// SendOperationsResponse holds response from SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsResponse {
    /// string field
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// object field
    #[prost(oneof = "send_operations_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_operations_response::Message>,
}
/// Nested message and enum types in `SendOperationsResponse`.
pub mod send_operations_response {
    /// object field
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        Result(super::OperationResult),
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Operation response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationResult {
    /// string field
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetVersionRequest holds request from GetVersion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// string value
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// string field
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// GetVersionResponse holds response from GetVersion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    /// string value
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// object field
    #[prost(oneof = "get_version_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<get_version_response::Message>,
}
/// Nested message and enum types in `GetVersionResponse`.
pub mod get_version_response {
    /// object field
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(string, tag = "2")]
        Result(::prost::alloc::string::String),
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Generated client implementations.
pub mod grpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Postman gRPC service
    #[derive(Debug, Clone)]
    pub struct GrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GrpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GrpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GrpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// GetVersion
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/postman.api.v1.Grpc/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("postman.api.v1.Grpc", "GetVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// SendBlocks
        pub async fn send_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/postman.api.v1.Grpc/SendBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("postman.api.v1.Grpc", "SendBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// SendOperations
        pub async fn send_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendOperationsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/postman.api.v1.Grpc/SendOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("postman.api.v1.Grpc", "SendOperations"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GrpcServer.
    #[async_trait]
    pub trait Grpc: Send + Sync + 'static {
        /// GetVersion
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendBlocks method.
        type SendBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::SendBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// SendBlocks
        async fn send_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SendBlocksStream>, tonic::Status>;
        /// Server streaming response type for the SendOperations method.
        type SendOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::SendOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// SendOperations
        async fn send_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendOperationsStream>,
            tonic::Status,
        >;
    }
    /// Postman gRPC service
    #[derive(Debug)]
    pub struct GrpcServer<T: Grpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Grpc> GrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GrpcServer<T>
    where
        T: Grpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/postman.api.v1.Grpc/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: Grpc>(pub Arc<T>);
                    impl<T: Grpc> tonic::server::UnaryService<super::GetVersionRequest>
                    for GetVersionSvc<T> {
                        type Response = super::GetVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/postman.api.v1.Grpc/SendBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct SendBlocksSvc<T: Grpc>(pub Arc<T>);
                    impl<
                        T: Grpc,
                    > tonic::server::StreamingService<super::SendBlocksRequest>
                    for SendBlocksSvc<T> {
                        type Response = super::SendBlocksResponse;
                        type ResponseStream = T::SendBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).send_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/postman.api.v1.Grpc/SendOperations" => {
                    #[allow(non_camel_case_types)]
                    struct SendOperationsSvc<T: Grpc>(pub Arc<T>);
                    impl<
                        T: Grpc,
                    > tonic::server::StreamingService<super::SendOperationsRequest>
                    for SendOperationsSvc<T> {
                        type Response = super::SendOperationsResponse;
                        type ResponseStream = T::SendOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Grpc> Clone for GrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Grpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Grpc> tonic::server::NamedService for GrpcServer<T> {
        const NAME: &'static str = "postman.api.v1.Grpc";
    }
}
