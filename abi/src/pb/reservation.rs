/// core reservasion object, contains all the information for a reservasion
/// 预定系统核心对象，包换全部信息
/// if ListResponse op is DELETE, onlu id will be populate
/// 如果 ListResponse 选项(op)是`DELETE`, 只会填入ID
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// unique id
    /// if put into ReserveRequest, id should be empty
    /// 如果放入 ReserveRequest，ID应该是空的
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// resource id
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    /// status, used for differentiating purpose
    /// 预留状态，用于区分的作用
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    pub status: i32,
    /// user id
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
    /// start time
    #[prost(message, optional, tag = "5")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    /// end time
    #[prost(message, optional, tag = "6")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// extra note`额外说明
    #[prost(string, tag = "7")]
    pub note: ::prost::alloc::string::String,
}
/// To make a reservation, send a ReserveRequest with Reservation object(id should be empty)
/// 如果进行预定，应该发送一个 ReserveRequest对象（id应该是空的）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveRequest {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// Created reservation will be returned in ReserveResponse
/// 创建预定，将会返回在ReserveResponse中
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To update a reservation, send an UpdateRequest, only note fields is updateble
/// 要更新Reservation, 发送UpdateRequest请求，只有`note`字段是可以更新的
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(string, tag = "2")]
    pub note: ::prost::alloc::string::String,
}
/// Update reservation will be returned in UpdateResponse
/// 更新reservation，将会返回在UpdateResponse中
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To change a reservation from pending toconfirmed, send a ConfirmRequest
/// 要将Reservation从待定更为确认,请发送确认请求(ConfirmRequest)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Confirmed reservation will be returned in ConfirmResponse
/// 确认的预订将在ConfirmResponse中返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To cancel a reservation, send a CancelRequest
/// 要取消Reservation，请发送取消请求(CancelRequest)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Cancel reservation will be returned in CancelResponse
/// 取消的预订将在CancelResponse中返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// To get a reservation, send a GetRequest
/// 要请求一个reservation, 请发送GetRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}
/// Reservation will be returned in GetResponse
/// 预定将在GetResponse中返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// query reservations user id, resource id, start, end, and status
#[derive(derive_builder::Builder)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationQuery {
    /// resource id, if empty, query all resources
    /// 预定资源查询ID, 如果ID为空则查询全部资源
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    /// user id, if empty, query all users
    /// 预定用户查询ID，如果ID为空，查询全部用户
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// use status to filter result. If UNKNOWN, return all reservations
    /// 如果用户状态未知，返回他的所有预定
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    pub status: i32,
    /// start time for the reservation query, if 0, use infinty start time
    /// 预定查询的起始时间, 如果为0，使用 infity start time
    #[prost(message, optional, tag = "4")]
    #[builder(setter(into, strip_option), default)]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    /// end time for the reservation query,if 0, use infinty end time
    /// 预定查询的结束时间,如 果为0，使用 infity end time
    #[prost(message, optional, tag = "5")]
    #[builder(setter(into, strip_option), default)]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
    /// sort direction` 排序方向
    #[prost(bool, tag = "6")]
    pub desc: bool,
}
/// To query a reservation, send a QueryRequest
/// 要查询一个预定，请发送QueryRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<ReservationQuery>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterResponse {
    #[prost(message, repeated, tag = "1")]
    pub reservations: ::prost::alloc::vec::Vec<Reservation>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<FilterPager>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueInt64 {
    #[prost(int64, tag = "1")]
    pub value: i64,
}
/// To query a reservation, order by reservation id
#[derive(derive_builder::Builder)]
#[builder(build_fn(name = "private_build"))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationFilter {
    /// resource id, if empty, query all resources
    /// 预定资源查询ID, 如果ID为空则查询全部资源
    #[prost(string, tag = "1")]
    #[builder(setter(into), default)]
    pub resource_id: ::prost::alloc::string::String,
    /// user id, if empty, query all users
    /// 预定用户查询ID，如果ID为空，查询全部用户
    #[prost(string, tag = "2")]
    #[builder(setter(into), default)]
    pub user_id: ::prost::alloc::string::String,
    /// use status to filter result. If UNKNOWN, return all reservations
    /// 如果用户状态未知，返回他的所有预定
    #[prost(enumeration = "ReservationStatus", tag = "3")]
    #[builder(setter(into), default)]
    pub status: i32,
    /// previous cursor
    #[prost(message, optional, tag = "4")]
    #[builder(setter(into), default)]
    pub cursor: ::core::option::Option<i64>,
    /// page size for the query
    #[prost(int64, tag = "5")]
    #[builder(setter(into), default = "10")]
    pub page_size: i64,
    /// sort direction` 排序方向
    #[prost(bool, tag = "6")]
    #[builder(setter(into), default)]
    pub desc: bool,
}
/// filter pager info
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterPager {
    #[prost(message, optional, tag = "1")]
    pub prev: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "2")]
    pub next: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "3")]
    pub total: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterRequest {
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<ReservationFilter>,
}
/// Client Can listen to reservation update by sending a ListenRequest
/// 客户端可以通过发送 ListenRequest 来监听预订更新
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenRequest {}
/// Server will send ListenResponse to client in streaming response
/// 服务器将在流式响应中向客户端发送 ListenResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenResponse {
    /// update type
    #[prost(enumeration = "ReservationUpdateType", tag = "1")]
    pub op: i32,
    /// id for updated reservation
    /// 更新预定的ID
    #[prost(message, optional, tag = "2")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// reservation status for a given time period -- 给定时间段的预订状态
#[derive(
    sqlx::Type, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
pub enum ReservationStatus {
    Unknown = 0,
    Pending = 1,
    /// 确认
    Confirmed = 2,
    Blocked = 3,
}
impl ReservationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReservationStatus::Unknown => "RESERVATION_STATUS_UNKNOWN",
            ReservationStatus::Pending => "RESERVATION_STATUS_PENDING",
            ReservationStatus::Confirmed => "RESERVATION_STATUS_CONFIRMED",
            ReservationStatus::Blocked => "RESERVATION_STATUS_BLOCKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVATION_STATUS_UNKNOWN" => Some(Self::Unknown),
            "RESERVATION_STATUS_PENDING" => Some(Self::Pending),
            "RESERVATION_STATUS_CONFIRMED" => Some(Self::Confirmed),
            "RESERVATION_STATUS_BLOCKED" => Some(Self::Blocked),
            _ => None,
        }
    }
}
/// when reservation is update, record the update type -- 预定更新时，记录更新的类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReservationUpdateType {
    Unknown = 0,
    Create = 1,
    Update = 2,
    Delete = 3,
}
impl ReservationUpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReservationUpdateType::Unknown => "RESERVATION_UPDATE_TYPE_UNKNOWN",
            ReservationUpdateType::Create => "RESERVATION_UPDATE_TYPE_CREATE",
            ReservationUpdateType::Update => "RESERVATION_UPDATE_TYPE_UPDATE",
            ReservationUpdateType::Delete => "RESERVATION_UPDATE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVATION_UPDATE_TYPE_UNKNOWN" => Some(Self::Unknown),
            "RESERVATION_UPDATE_TYPE_CREATE" => Some(Self::Create),
            "RESERVATION_UPDATE_TYPE_UPDATE" => Some(Self::Update),
            "RESERVATION_UPDATE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod reservation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Reservation service
    #[derive(Debug, Clone)]
    pub struct ReservationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReservationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReservationServiceClient<T>
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
        ) -> ReservationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ReservationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// make a reservation
        /// 构建一个预定
        pub async fn reserve(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveRequest>,
        ) -> Result<tonic::Response<super::ReserveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/reserve");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// confirm a pending reservation, if reservation is not pending, do nothing
        /// 确认待定的预定，如果预定不是待定的，什么都不做
        pub async fn confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmRequest>,
        ) -> Result<tonic::Response<super::ConfirmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/confirm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// update the reservation note
        /// 更新这个预定的注释`note`
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/update");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// cancle a reservation
        /// 取消预定
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/cancel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// get a reservation by id
        /// 根据ID获取预定
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/reservation.ReservationService/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query reservation by resource id, user id, status, start time, end time
        /// 通过资源id、用户id、状态、开始时间、结束时间 查询预约
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Reservation>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/Query");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// 查询时通过ID进行排序
        pub async fn filter(
            &mut self,
            request: impl tonic::IntoRequest<super::FilterRequest>,
        ) -> Result<tonic::Response<super::FilterResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/filter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// an other system could moniton newly added/confirmed/cancelled reservation
        /// 另一个系统可以监控新添加/确认/取消的预订
        pub async fn listen(
            &mut self,
            request: impl tonic::IntoRequest<super::ListenRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Reservation>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/reservation.ReservationService/listen");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod reservation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReservationServiceServer.
    #[async_trait]
    pub trait ReservationService: Send + Sync + 'static {
        /// make a reservation
        /// 构建一个预定
        async fn reserve(
            &self,
            request: tonic::Request<super::ReserveRequest>,
        ) -> Result<tonic::Response<super::ReserveResponse>, tonic::Status>;
        /// confirm a pending reservation, if reservation is not pending, do nothing
        /// 确认待定的预定，如果预定不是待定的，什么都不做
        async fn confirm(
            &self,
            request: tonic::Request<super::ConfirmRequest>,
        ) -> Result<tonic::Response<super::ConfirmResponse>, tonic::Status>;
        /// update the reservation note
        /// 更新这个预定的注释`note`
        async fn update(
            &self,
            request: tonic::Request<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
        /// cancle a reservation
        /// 取消预定
        async fn cancel(
            &self,
            request: tonic::Request<super::CancelRequest>,
        ) -> Result<tonic::Response<super::CancelResponse>, tonic::Status>;
        /// get a reservation by id
        /// 根据ID获取预定
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        /// Server streaming response type for the Query method.
        type QueryStream: futures_core::Stream<Item = Result<super::Reservation, tonic::Status>>
            + Send
            + 'static;
        /// query reservation by resource id, user id, status, start time, end time
        /// 通过资源id、用户id、状态、开始时间、结束时间 查询预约
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> Result<tonic::Response<Self::QueryStream>, tonic::Status>;
        /// 查询时通过ID进行排序
        async fn filter(
            &self,
            request: tonic::Request<super::FilterRequest>,
        ) -> Result<tonic::Response<super::FilterResponse>, tonic::Status>;
        /// Server streaming response type for the listen method.
        type listenStream: futures_core::Stream<Item = Result<super::Reservation, tonic::Status>>
            + Send
            + 'static;
        /// an other system could moniton newly added/confirmed/cancelled reservation
        /// 另一个系统可以监控新添加/确认/取消的预订
        async fn listen(
            &self,
            request: tonic::Request<super::ListenRequest>,
        ) -> Result<tonic::Response<Self::listenStream>, tonic::Status>;
    }
    /// Reservation service
    #[derive(Debug)]
    pub struct ReservationServiceServer<T: ReservationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ReservationService> ReservationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ReservationServiceServer<T>
    where
        T: ReservationService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/reservation.ReservationService/reserve" => {
                    #[allow(non_camel_case_types)]
                    struct reserveSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::ReserveRequest> for reserveSvc<T> {
                        type Response = super::ReserveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReserveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reserve(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = reserveSvc(inner);
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
                "/reservation.ReservationService/confirm" => {
                    #[allow(non_camel_case_types)]
                    struct confirmSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::ConfirmRequest> for confirmSvc<T> {
                        type Response = super::ConfirmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).confirm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = confirmSvc(inner);
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
                "/reservation.ReservationService/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::UpdateRequest> for updateSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
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
                "/reservation.ReservationService/cancel" => {
                    #[allow(non_camel_case_types)]
                    struct cancelSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::CancelRequest> for cancelSvc<T> {
                        type Response = super::CancelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = cancelSvc(inner);
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
                "/reservation.ReservationService/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::GetRequest> for getSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
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
                "/reservation.ReservationService/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService>
                        tonic::server::ServerStreamingService<super::QueryRequest> for QuerySvc<T>
                    {
                        type Response = super::Reservation;
                        type ResponseStream = T::QueryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reservation.ReservationService/filter" => {
                    #[allow(non_camel_case_types)]
                    struct filterSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService> tonic::server::UnaryService<super::FilterRequest> for filterSvc<T> {
                        type Response = super::FilterResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).filter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = filterSvc(inner);
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
                "/reservation.ReservationService/listen" => {
                    #[allow(non_camel_case_types)]
                    struct listenSvc<T: ReservationService>(pub Arc<T>);
                    impl<T: ReservationService>
                        tonic::server::ServerStreamingService<super::ListenRequest>
                        for listenSvc<T>
                    {
                        type Response = super::Reservation;
                        type ResponseStream = T::listenStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).listen(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: ReservationService> Clone for ReservationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ReservationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReservationService> tonic::server::NamedService for ReservationServiceServer<T> {
        const NAME: &'static str = "reservation.ReservationService";
    }
}
