#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvId {
    #[prost(string, tag = "1")]
    pub env_label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvDescriptor {
    #[prost(message, optional, tag = "1")]
    pub env_id: ::core::option::Option<EnvId>,
    #[prost(string, tag = "2")]
    pub local_ip: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub local_user: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub local_path: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub remote_ip: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub remote_user: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub remote_path: ::prost::alloc::string::String,
    #[prost(int32, tag = "8")]
    pub fs_max_size: i32,
    #[prost(enumeration = "env_descriptor::Status", tag = "9")]
    pub status: i32,
}
/// Nested message and enum types in `EnvDescriptor`.
pub mod env_descriptor {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Registered = 0,
        SettingUpInMemoryFs = 1,
        SettingUpTransparentProxy = 2,
        SettingUpFsSync = 3,
        Ready = 4,
        Error = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEnvRequest {
    #[prost(message, optional, tag = "1")]
    pub env_desc: ::core::option::Option<EnvDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEnvResponse {
    #[prost(enumeration = "register_env_response::Status", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `RegisterEnvResponse`.
pub mod register_env_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        EnvWithIdExists = 0,
        SshTestFailed = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearEnvRequest {
    #[prost(message, optional, tag = "1")]
    pub env_id: ::core::option::Option<EnvId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearEnvResponse {
    #[prost(enumeration = "clear_env_response::Result", tag = "2")]
    pub results: i32,
    #[prost(string, optional, tag = "3")]
    pub error_msg: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ClearEnvResponse`.
pub mod clear_env_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        Success = 0,
        Error = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupInMemoryFsRequest {
    #[prost(message, optional, tag = "1")]
    pub env_id: ::core::option::Option<EnvId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupInMemoryFsResponse {
    #[prost(message, optional, tag = "1")]
    pub env_id: ::core::option::Option<EnvId>,
    #[prost(enumeration = "setup_in_memory_fs_response::Result", tag = "2")]
    pub result: i32,
}
/// Nested message and enum types in `SetupInMemoryFsResponse`.
pub mod setup_in_memory_fs_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        Success = 0,
        PathAlreadyExists = 1,
        NotEnoughMemory = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupTransparentProxyRequest {
    #[prost(message, optional, tag = "1")]
    pub env_id: ::core::option::Option<EnvId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupTransparentProxyResponse {
    #[prost(message, optional, tag = "1")]
    pub env_id: ::core::option::Option<EnvId>,
}
#[doc = r" Generated client implementations."]
pub mod r_docker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct RDockerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RDockerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?
                .connect()
                .await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RDockerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RDockerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RDockerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn echo(
            &mut self,
            request: impl tonic::IntoRequest<super::EchoRequest>,
        ) -> Result<tonic::Response<super::EchoResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/Echo");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn register_env(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterEnvRequest>,
        ) -> Result<tonic::Response<super::RegisterEnvResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/RegisterEnv");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn clear_env(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearEnvRequest>,
        ) -> Result<tonic::Response<super::ClearEnvResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/ClearEnv");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn setup_in_memory_fs(
            &mut self,
            request: impl tonic::IntoRequest<super::SetupInMemoryFsRequest>,
        ) -> Result<tonic::Response<super::SetupInMemoryFsResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/SetupInMemoryFs");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        pub async fn setup_transparent_proxy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetupTransparentProxyRequest>,
        ) -> Result<tonic::Response<super::SetupTransparentProxyResponse>, tonic::Status> {
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
            let path =
                http::uri::PathAndQuery::from_static("/rdocker.RDocker/SetupTransparentProxy");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod r_docker_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RDockerServer."]
    #[async_trait]
    pub trait RDocker: Send + Sync + 'static {
        async fn echo(
            &self,
            request: tonic::Request<super::EchoRequest>,
        ) -> Result<tonic::Response<super::EchoResponse>, tonic::Status>;
        async fn register_env(
            &self,
            request: tonic::Request<super::RegisterEnvRequest>,
        ) -> Result<tonic::Response<super::RegisterEnvResponse>, tonic::Status>;
        async fn clear_env(
            &self,
            request: tonic::Request<super::ClearEnvRequest>,
        ) -> Result<tonic::Response<super::ClearEnvResponse>, tonic::Status>;
        async fn setup_in_memory_fs(
            &self,
            request: tonic::Request<super::SetupInMemoryFsRequest>,
        ) -> Result<tonic::Response<super::SetupInMemoryFsResponse>, tonic::Status>;
        async fn setup_transparent_proxy(
            &self,
            request: tonic::Request<super::SetupTransparentProxyRequest>,
        ) -> Result<tonic::Response<super::SetupTransparentProxyResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RDockerServer<T: RDocker> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RDocker> RDockerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RDockerServer<T>
    where
        T: RDocker,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rdocker.RDocker/Echo" => {
                    #[allow(non_camel_case_types)]
                    struct EchoSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::EchoRequest> for EchoSvc<T> {
                        type Response = super::EchoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EchoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).echo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EchoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rdocker.RDocker/RegisterEnv" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterEnvSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::RegisterEnvRequest> for RegisterEnvSvc<T> {
                        type Response = super::RegisterEnvResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterEnvRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .register_env(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterEnvSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rdocker.RDocker/ClearEnv" => {
                    #[allow(non_camel_case_types)]
                    struct ClearEnvSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::ClearEnvRequest> for ClearEnvSvc<T> {
                        type Response = super::ClearEnvResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearEnvRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .clear_env(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearEnvSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rdocker.RDocker/SetupInMemoryFs" => {
                    #[allow(non_camel_case_types)]
                    struct SetupInMemoryFsSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::SetupInMemoryFsRequest>
                        for SetupInMemoryFsSvc<T>
                    {
                        type Response = super::SetupInMemoryFsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetupInMemoryFsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .setup_in_memory_fs(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetupInMemoryFsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rdocker.RDocker/SetupTransparentProxy" => {
                    #[allow(non_camel_case_types)]
                    struct SetupTransparentProxySvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker>
                        tonic::server::UnaryService<super::SetupTransparentProxyRequest>
                        for SetupTransparentProxySvc<T>
                    {
                        type Response = super::SetupTransparentProxyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetupTransparentProxyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .setup_transparent_proxy(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetupTransparentProxySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RDocker> Clone for RDockerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RDocker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RDocker> tonic::transport::NamedService for RDockerServer<T> {
        const NAME: &'static str = "rdocker.RDocker";
    }
}
