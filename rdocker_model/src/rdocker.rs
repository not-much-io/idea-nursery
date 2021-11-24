#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    #[prost(message, optional, tag = "1")]
    pub desc:   ::core::option::Option<EnvDescriptor>,
    #[prost(enumeration = "environment::Status", tag = "9")]
    pub status: i32,
    #[prost(enumeration = "environment::Error", tag = "10")]
    pub error:  i32,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Registered = 0,
        SettingUpInMemoryFs = 1,
        SettingUpTransparentProxy = 2,
        SettingUpFsSync = 3,
        Ready = 4,
        Errored = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        None = 0,
        Unspecified = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvDescriptor {
    #[prost(string, tag = "1")]
    pub env_id:      ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub local_ip:    ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub local_user:  ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub local_path:  ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub remote_ip:   ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub remote_user: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub remote_path: ::prost::alloc::string::String,
}
// ========== Setting Up ==========

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEnvRequest {
    #[prost(message, optional, tag = "1")]
    pub env_desc: ::core::option::Option<EnvDescriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterEnvResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpInMemoryFsRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpInMemoryFsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpFsSyncRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpFsSyncResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpTransparentProxyRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUpTransparentProxyResponse {}
// ========== Tearing Down ==========

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TearDownTransparentProxyRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TearDownTransparentProxyResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TearDownFsSyncRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TearDownFsSyncResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TearDownInMemoryFsRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TearDownInMemoryFsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterEnvRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnregisterEnvResponse {}
// ========== Reading State ==========

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadEnvRequest {
    #[prost(string, tag = "1")]
    pub env_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadEnvResponse {
    #[prost(message, optional, tag = "1")]
    pub env: ::core::option::Option<Environment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvsResponse {
    #[prost(message, repeated, tag = "1")]
    pub env_descs: ::prost::alloc::vec::Vec<EnvDescriptor>,
}
#[doc = r" Generated client implementations."]
pub mod r_docker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " TODO: Currently just local network funcionality, using local ips, etc."]
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
        #[doc = " Register a new environment in remote"]
        #[doc = " Safe operation that fails if any conflicts with existing envs"]
        #[doc = " Reuses processes that other envs have already set up"]
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
        #[doc = " Set up in-memory fs on the rdockerd host"]
        pub async fn set_up_in_memory_fs(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUpInMemoryFsRequest>,
        ) -> Result<tonic::Response<super::SetUpInMemoryFsResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/SetUpInMemoryFs");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Set up two way fs sync between local project dir and remote in-memory fs"]
        pub async fn set_up_fs_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUpFsSyncRequest>,
        ) -> Result<tonic::Response<super::SetUpFsSyncResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/SetUpFsSync");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Set up transparent proxying from remote through local"]
        pub async fn set_up_transparent_proxy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUpTransparentProxyRequest>,
        ) -> Result<tonic::Response<super::SetUpTransparentProxyResponse>, tonic::Status> {
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
                http::uri::PathAndQuery::from_static("/rdocker.RDocker/SetUpTransparentProxy");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Tear down transparent proxying from remote through local"]
        #[doc = " If other env exists that uses this process, it will NOT be cleaned up"]
        pub async fn tear_down_transparent_proxy(
            &mut self,
            request: impl tonic::IntoRequest<super::TearDownTransparentProxyRequest>,
        ) -> Result<tonic::Response<super::TearDownTransparentProxyResponse>, tonic::Status>
        {
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
                http::uri::PathAndQuery::from_static("/rdocker.RDocker/TearDownTransparentProxy");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Tear down fn sync between local project dir and remote in-memory fs"]
        pub async fn tear_down_fs_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::TearDownFsSyncRequest>,
        ) -> Result<tonic::Response<super::TearDownFsSyncResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/TearDownFsSync");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Tear down fn sync between local project dir and remote in-memory fs"]
        #[doc = " Will error if fs sync still active on the fs"]
        pub async fn tear_down_in_memory_fs(
            &mut self,
            request: impl tonic::IntoRequest<super::TearDownInMemoryFsRequest>,
        ) -> Result<tonic::Response<super::TearDownInMemoryFsResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/TearDownInMemoryFs");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Unregister and existing environment in remote"]
        #[doc = " This cleanup will remove any mention of the enviornment having even been on the remote"]
        #[doc = " Will fail if proper tear down hasn't been executed before this"]
        pub async fn unregister_env(
            &mut self,
            request: impl tonic::IntoRequest<super::UnregisterEnvRequest>,
        ) -> Result<tonic::Response<super::UnregisterEnvResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/UnregisterEnv");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " Get the current state of an env"]
        pub async fn read_env(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadEnvRequest>,
        ) -> Result<tonic::Response<super::ReadEnvResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/ReadEnv");
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        #[doc = " List all the envs in remote"]
        pub async fn list_envs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvsRequest>,
        ) -> Result<tonic::Response<super::ListEnvsResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/rdocker.RDocker/ListEnvs");
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
        #[doc = " Register a new environment in remote"]
        #[doc = " Safe operation that fails if any conflicts with existing envs"]
        #[doc = " Reuses processes that other envs have already set up"]
        async fn register_env(
            &self,
            request: tonic::Request<super::RegisterEnvRequest>,
        ) -> Result<tonic::Response<super::RegisterEnvResponse>, tonic::Status>;
        #[doc = " Set up in-memory fs on the rdockerd host"]
        async fn set_up_in_memory_fs(
            &self,
            request: tonic::Request<super::SetUpInMemoryFsRequest>,
        ) -> Result<tonic::Response<super::SetUpInMemoryFsResponse>, tonic::Status>;
        #[doc = " Set up two way fs sync between local project dir and remote in-memory fs"]
        async fn set_up_fs_sync(
            &self,
            request: tonic::Request<super::SetUpFsSyncRequest>,
        ) -> Result<tonic::Response<super::SetUpFsSyncResponse>, tonic::Status>;
        #[doc = " Set up transparent proxying from remote through local"]
        async fn set_up_transparent_proxy(
            &self,
            request: tonic::Request<super::SetUpTransparentProxyRequest>,
        ) -> Result<tonic::Response<super::SetUpTransparentProxyResponse>, tonic::Status>;
        #[doc = " Tear down transparent proxying from remote through local"]
        #[doc = " If other env exists that uses this process, it will NOT be cleaned up"]
        async fn tear_down_transparent_proxy(
            &self,
            request: tonic::Request<super::TearDownTransparentProxyRequest>,
        ) -> Result<tonic::Response<super::TearDownTransparentProxyResponse>, tonic::Status>;
        #[doc = " Tear down fn sync between local project dir and remote in-memory fs"]
        async fn tear_down_fs_sync(
            &self,
            request: tonic::Request<super::TearDownFsSyncRequest>,
        ) -> Result<tonic::Response<super::TearDownFsSyncResponse>, tonic::Status>;
        #[doc = " Tear down fn sync between local project dir and remote in-memory fs"]
        #[doc = " Will error if fs sync still active on the fs"]
        async fn tear_down_in_memory_fs(
            &self,
            request: tonic::Request<super::TearDownInMemoryFsRequest>,
        ) -> Result<tonic::Response<super::TearDownInMemoryFsResponse>, tonic::Status>;
        #[doc = " Unregister and existing environment in remote"]
        #[doc = " This cleanup will remove any mention of the enviornment having even been on the remote"]
        #[doc = " Will fail if proper tear down hasn't been executed before this"]
        async fn unregister_env(
            &self,
            request: tonic::Request<super::UnregisterEnvRequest>,
        ) -> Result<tonic::Response<super::UnregisterEnvResponse>, tonic::Status>;
        #[doc = " Get the current state of an env"]
        async fn read_env(
            &self,
            request: tonic::Request<super::ReadEnvRequest>,
        ) -> Result<tonic::Response<super::ReadEnvResponse>, tonic::Status>;
        #[doc = " List all the envs in remote"]
        async fn list_envs(
            &self,
            request: tonic::Request<super::ListEnvsRequest>,
        ) -> Result<tonic::Response<super::ListEnvsResponse>, tonic::Status>;
    }
    #[doc = " TODO: Currently just local network funcionality, using local ips, etc."]
    #[derive(Debug)]
    pub struct RDockerServer<T: RDocker> {
        inner:                        _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings:   (),
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
                "/rdocker.RDocker/SetUpInMemoryFs" => {
                    #[allow(non_camel_case_types)]
                    struct SetUpInMemoryFsSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::SetUpInMemoryFsRequest>
                        for SetUpInMemoryFsSvc<T>
                    {
                        type Response = super::SetUpInMemoryFsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetUpInMemoryFsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .set_up_in_memory_fs(request)
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
                        let method = SetUpInMemoryFsSvc(inner);
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
                "/rdocker.RDocker/SetUpFsSync" => {
                    #[allow(non_camel_case_types)]
                    struct SetUpFsSyncSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::SetUpFsSyncRequest> for SetUpFsSyncSvc<T> {
                        type Response = super::SetUpFsSyncResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetUpFsSyncRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .set_up_fs_sync(request)
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
                        let method = SetUpFsSyncSvc(inner);
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
                "/rdocker.RDocker/SetUpTransparentProxy" => {
                    #[allow(non_camel_case_types)]
                    struct SetUpTransparentProxySvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker>
                        tonic::server::UnaryService<super::SetUpTransparentProxyRequest>
                        for SetUpTransparentProxySvc<T>
                    {
                        type Response = super::SetUpTransparentProxyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetUpTransparentProxyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .set_up_transparent_proxy(request)
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
                        let method = SetUpTransparentProxySvc(inner);
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
                "/rdocker.RDocker/TearDownTransparentProxy" => {
                    #[allow(non_camel_case_types)]
                    struct TearDownTransparentProxySvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker>
                        tonic::server::UnaryService<super::TearDownTransparentProxyRequest>
                        for TearDownTransparentProxySvc<T>
                    {
                        type Response = super::TearDownTransparentProxyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TearDownTransparentProxyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .tear_down_transparent_proxy(request)
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
                        let method = TearDownTransparentProxySvc(inner);
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
                "/rdocker.RDocker/TearDownFsSync" => {
                    #[allow(non_camel_case_types)]
                    struct TearDownFsSyncSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::TearDownFsSyncRequest>
                        for TearDownFsSyncSvc<T>
                    {
                        type Response = super::TearDownFsSyncResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TearDownFsSyncRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .tear_down_fs_sync(request)
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
                        let method = TearDownFsSyncSvc(inner);
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
                "/rdocker.RDocker/TearDownInMemoryFs" => {
                    #[allow(non_camel_case_types)]
                    struct TearDownInMemoryFsSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::TearDownInMemoryFsRequest>
                        for TearDownInMemoryFsSvc<T>
                    {
                        type Response = super::TearDownInMemoryFsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TearDownInMemoryFsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .tear_down_in_memory_fs(request)
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
                        let method = TearDownInMemoryFsSvc(inner);
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
                "/rdocker.RDocker/UnregisterEnv" => {
                    #[allow(non_camel_case_types)]
                    struct UnregisterEnvSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::UnregisterEnvRequest> for UnregisterEnvSvc<T> {
                        type Response = super::UnregisterEnvResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnregisterEnvRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .unregister_env(request)
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
                        let method = UnregisterEnvSvc(inner);
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
                "/rdocker.RDocker/ReadEnv" => {
                    #[allow(non_camel_case_types)]
                    struct ReadEnvSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::ReadEnvRequest> for ReadEnvSvc<T> {
                        type Response = super::ReadEnvResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadEnvRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .read_env(request)
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
                        let method = ReadEnvSvc(inner);
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
                "/rdocker.RDocker/ListEnvs" => {
                    #[allow(non_camel_case_types)]
                    struct ListEnvsSvc<T: RDocker>(pub Arc<T>);
                    impl<T: RDocker> tonic::server::UnaryService<super::ListEnvsRequest> for ListEnvsSvc<T> {
                        type Response = super::ListEnvsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEnvsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .list_envs(request)
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
                        let method = ListEnvsSvc(inner);
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
