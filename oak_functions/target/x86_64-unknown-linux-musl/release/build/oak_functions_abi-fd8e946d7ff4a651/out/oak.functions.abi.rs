/// The inference from a TensorFlow model, containing an inference vector of floats, and a shape
/// vector specifying the dimensions of the inference vector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inference {
    #[prost(uint64, repeated, tag="1")]
    pub shape: ::prost::alloc::vec::Vec<u64>,
    #[prost(float, repeated, tag="2")]
    pub inference_vec: ::prost::alloc::vec::Vec<f32>,
}
/// Configuration information that should be provided to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigurationInfo {
    /// Hash of the loaded Wasm module.
    #[prost(bytes="vec", tag="1")]
    pub wasm_hash: ::prost::alloc::vec::Vec<u8>,
    /// The validated server-side policy.
    #[prost(message, optional, tag="2")]
    pub policy: ::core::option::Option<ServerPolicy>,
    /// Whether machine learning inference using a TensorFlow model is enabled.
    #[prost(bool, tag="3")]
    pub ml_inference: bool,
    /// Server-side configuration for differentially private metrics. This field is optional. If not
    /// provided, differentially private metrics are not enabled on the server.
    #[prost(message, optional, tag="4")]
    pub metrics: ::core::option::Option<PrivateMetricsConfig>,
}
//// Server-side policy describing limits on the size of the response and response processing time to
//// avoid side-channel leaks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPolicy {
    /// A fixed size for responses returned by the trusted runtime.
    ///
    /// This size only applies to the body of the Oak Functions response. If the response body
    /// computed by the Wasm module is smaller than this amount, it is padded with additional
    /// data before serialization and inclusion in the HTTP response to the client. If the body is
    /// larger than this amount, the trusted runtime discards the response and instead uses a
    /// response with a body of exactly this size, containing an error message indicating the
    /// policy violation. The body included in the HTTP response sent to the client is the binary
    /// protobuf encoding of the Oak Functions response, and will have a size larger than
    /// `constant_response_size_bytes`. However, this size is still guaranteed to be a constant.
    #[prost(uint32, tag="1")]
    pub constant_response_size_bytes: u32,
    /// A fixed response time, in milliseconds.
    ///
    /// Similar to the previous one, but controls the amount of time the function is allowed to run
    /// for. If the function finishes before this time, the response is not sent back until the
    /// time is elapsed. If the function does not finish within this deadline, the trusted runtime
    /// sends a response to the client containing an error message indicating the failure. The size
    /// of this response is equal to the size specified by the previous parameter.
    #[prost(uint32, tag="2")]
    pub constant_processing_time_ms: u32,
}
/// Configuration for differentially private metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateMetricsConfig {
    /// The privacy budget.
    #[prost(double, tag="1")]
    pub epsilon: f64,
    /// The number of requests that will be aggregated into each batch.
    #[prost(uint32, tag="2")]
    pub batch_size: u32,
}
/// Status values exchanged as i32 values across the Node Wasm interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OakStatus {
    Unspecified = 0,
    /// Success.
    Ok = 1,
    /// Arguments invalid.
    ErrInvalidArgs = 2,
    /// Internal error.
    ErrInternal = 3,
    /// Lookup storage item not found.
    ErrStorageItemNotFound = 4,
    /// TensorFlow model not found.
    ErrTensorFlowModelNotFound = 5,
    /// Error when running the TensorFlow model, due to bad input tensor.
    ErrBadTensorFlowModelInput = 6,
}
