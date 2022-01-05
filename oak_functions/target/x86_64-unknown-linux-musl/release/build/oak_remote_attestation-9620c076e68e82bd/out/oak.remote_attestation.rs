//// Placeholder implementation of TEE report for remote attestation.
//// <https://www.amd.com/system/files/TechDocs/56860.pdf#page=39>
////
//// TODO(#1867): Add remote attestation support and use real TEE reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationReport {
    /// Version number of this attestation report.
    #[prost(int32, tag="1")]
    pub version: i32,
    /// Security version number of SNP firmware.
    #[prost(int32, tag="2")]
    pub svn: i32,
    /// The installed version of firmware.
    #[prost(int32, tag="3")]
    pub platform_version: i32,
    /// Arbitrary data to put into the TEE report.
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// TEE measurement, i.e. VM hash.
    #[prost(bytes="vec", tag="5")]
    pub measurement: ::prost::alloc::vec::Vec<u8>,
    /// Signature of this report.
    #[prost(bytes="vec", tag="6")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
//// Convenience struct for representing X.509 TEE extensions containing TEE reports and TEE
//// provider's certificates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationInfo {
    //// TEE report.
    #[prost(message, optional, tag="1")]
    pub report: ::core::option::Option<AttestationReport>,
    //// Provider's PEM encoded X.509 certificate that signs TEE firmware keys.
    //// <https://tools.ietf.org/html/rfc7468>
    #[prost(bytes="vec", tag="2")]
    pub certificate: ::prost::alloc::vec::Vec<u8>,
}
