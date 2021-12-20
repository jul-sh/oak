/// A request to execute a benchmark action a specified number of times.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BenchmarkRequest {
    #[prost(uint32, tag="1")]
    pub iterations: u32,
    #[prost(oneof="benchmark_request::Action", tags="2")]
    pub action: ::core::option::Option<benchmark_request::Action>,
}
/// Nested message and enum types in `BenchmarkRequest`.
pub mod benchmark_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag="2")]
        Lookup(super::LookupTest),
    }
}
/// Container for the key to use when doing a benchmark lookup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupTest {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
