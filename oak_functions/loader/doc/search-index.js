var searchIndex = JSON.parse('{\
"oak_functions_loader":{"doc":"","i":[[0,"proto","oak_functions_loader","",null,null],[3,"Request","oak_functions_loader::proto","Wrapper around the encrypted payload. TODO(#2144): Extend…",null,null],[12,"encrypted_payload","","Payload encrypted using the session key.",0,null],[3,"AttestedInvokeRequest","","",null,null],[12,"request_type","","",1,null],[3,"AttestedInvokeResponse","","",null,null],[12,"response_type","","",2,null],[0,"attested_invoke_request","","Nested message and enum types in `AttestedInvokeRequest`.",null,null],[4,"RequestType","oak_functions_loader::proto::attested_invoke_request","",null,null],[13,"ClientHello","","Initialization message that starts the Remote Attestation…",3,null],[13,"ClientIdentity","","Client part of session key negotiation.",3,null],[13,"Request","","Request with payload encrypted using the session key.",3,null],[11,"encode","","",3,[[]]],[11,"merge","","",3,[[["decodecontext",3],["option",4],["wiretype",4]],[["decodeerror",3],["result",4]]]],[11,"encoded_len","","",3,[[]]],[0,"attested_invoke_response","oak_functions_loader::proto","Nested message and enum types in `AttestedInvokeResponse`.",null,null],[4,"ResponseType","oak_functions_loader::proto::attested_invoke_response","",null,null],[13,"ServerIdentity","","Server part of session key negotiation.",4,null],[13,"EncryptedPayload","","Payload encrypted using the session key.",4,null],[11,"encode","","",4,[[]]],[11,"merge","","",4,[[["decodecontext",3],["wiretype",4],["option",4]],[["decodeerror",3],["result",4]]]],[11,"encoded_len","","",4,[[]]],[0,"remote_attestation_server","oak_functions_loader::proto","Generated server implementations.",null,null],[3,"RemoteAttestationServer","oak_functions_loader::proto::remote_attestation_server","",null,null],[8,"RemoteAttestation","","Generated trait containing gRPC methods that should be…",null,null],[16,"AttestedInvokeStream","","Server streaming response type for the AttestedInvoke…",5,null],[10,"attested_invoke","","Creates a message stream for session key negotiation and…",5,[[["request",3],["streaming",3]],[["pin",3],["box",3]]]],[11,"new","","",6,[[]]],[11,"with_interceptor","","",6,[[]]],[0,"attestation","oak_functions_loader","",null,null],[3,"AttestationServer","oak_functions_loader::attestation","gRPC Attestation Service implementation.",null,null],[11,"create","","",7,[[["vec",3]],["result",6]]],[0,"grpc","oak_functions_loader","gRPC server for Oak Functions.",null,null],[5,"create_wasm_handler","oak_functions_loader::grpc","Creates a [`WasmHandler`] with the given Wasm module,…",null,[[["boxedextension",6],["logger",3],["arc",3],["arc",3],["lookupdata",3],["option",4],["vec",3]],[["result",6],["wasmhandler",3]]]],[5,"create_and_start_grpc_server","","Starts a gRPC server on the given address, serving the…",null,[[["vec",3],["logger",3],["future",8],["wasmhandler",3],["socketaddr",4],["policy",3]]]],[0,"logger","oak_functions_loader","",null,null],[3,"Logger","oak_functions_loader::logger","A simple logger that splits logging between writing logs…",null,null],[11,"new","","Creates a new logger with the specified maximum…",8,[[["levelfilter",4]]]],[11,"for_test","","Creates a new logger for testing using the debug…",8,[[]]],[11,"log_sensitive","","Logs the message at the specified `Level`, but only if the…",8,[[["level",4]]]],[11,"log_public","","Logs a message that contains only public, non-sensitive…",8,[[["level",4]]]],[0,"lookup","oak_functions_loader","",null,null],[3,"LookupData","oak_functions_loader::lookup","An in-memory lookup store instance that can refresh its…",null,null],[4,"LookupDataAuth","","",null,null],[13,"None","","",9,null],[13,"GcpMetadataToken","","",9,null],[5,"parse_lookup_entries","","",null,[[["buf",8]],[["result",6],["hashmap",3]]]],[11,"default","","",9,[[]]],[11,"new_empty","","Creates a new empty [`LookupData`] instance that can…",10,[[["logger",3],["lookupdataauth",4]],["lookupdata",3]]],[11,"refresh","","Refreshes the internal entries of this struct from the…",10,[[]]],[11,"for_test","","Creates an instance of LookupData populated with the given…",10,[[["hashmap",3],["vec",3]]]],[11,"get","","Convenience getter for an individual entry that reduces…",10,[[],[["vec",3],["option",4]]]],[11,"len","","",10,[[]]],[11,"is_empty","","",10,[[]]],[0,"metrics","oak_functions_loader","",null,null],[3,"PrivateMetricsConfig","oak_functions_loader::metrics","Configuration for differentially-private metrics reporting.",null,null],[12,"epsilon","","The privacy budget. See…",11,null],[12,"batch_size","","The number of requests that will be aggregated into each…",11,null],[12,"buckets","","The labels and configurations of buckets for which metrics…",11,null],[3,"PrivateMetricsAggregator","","Aggregator for count- and sum-based differentially private…",null,null],[3,"PrivateMetricsProxy","","Proxy for use by request handler instances to push…",null,null],[4,"BucketConfig","","Configuration for metrics buckets.",null,null],[13,"Count","","A bucket used for counting of events. This is equivalent…",12,null],[13,"Sum","","A bucket used for summing integer values in a range.…",12,null],[12,"min","oak_functions_loader::metrics::BucketConfig","",13,null],[12,"max","","",13,null],[5,"add_laplace_noise","oak_functions_loader::metrics","Adds Laplacian noise with parameter `beta` scaled by…",null,[[["stdrng",3]]]],[11,"validate","","",11,[[],["result",6]]],[11,"new","","",14,[[["privatemetricsconfig",3]],["result",6]]],[11,"report_metrics","","Reports new metrics for a single request that should be…",14,[[["string",3],["hashmap",3]],["option",4]]],[11,"new","","",15,[[["arc",3],["mutex",3]]]],[11,"report_metric","","Sets the value for the labeled metric. If a value was…",15,[[]]],[11,"publish","","Consumes the proxy and publishes the local…",15,[[],["option",4]]],[0,"server","oak_functions_loader","",null,null],[3,"Policy","oak_functions_loader::server","A policy describing limits on the size of the response and…",null,null],[12,"constant_response_size_bytes","","A fixed size for responses returned by the trusted runtime.",16,null],[12,"constant_processing_time","","A fixed response time.",16,null],[3,"ValidatedPolicy","","Similar to [`Policy`], but it is guaranteed to be valid.…",null,null],[3,"ExtensionResult","","",null,null],[12,"bytes","","",17,null],[12,"buf_ptr_ptr","","",17,null],[12,"buf_len_ptr","","",17,null],[3,"WasmState","","`WasmState` holds runtime values for a particular…",null,null],[3,"WasmHandler","","",null,null],[5,"apply_policy","","Runs the given function and applies the given security…",null,[[["validatedpolicy",3]]]],[5,"format_bytes","","Converts a binary sequence to a string if it is a valid…",null,[[],["string",3]]],[6,"AbiPointer","","",null,null],[6,"AbiPointerOffset","","",null,null],[6,"BoxedExtension","","",null,null],[17,"ABI_USIZE","","Wasm type identifier for position/offset values in linear…",null,null],[8,"OakApiNativeExtension","","Trait for implementing extensions, to implement new native…",null,null],[10,"invoke","","Similar to `invoke_index` in [`wasmi::Externals`], but may…",18,[[["wasmstate",3],["runtimeargs",3]],[["result",4],["result",4],["trap",3]]]],[10,"get_metadata","","Metadata about this Extension, including the exported host…",18,[[]]],[11,"validate","","",16,[[],["result",6]]],[11,"get_memory","","Helper function to get memory.",19,[[],["memoryref",3]]],[11,"read_request","","Corresponds to the host ABI function `read_request`.",19,[[["abipointer",6]],[["result",4],["oakstatus",4]]]],[11,"write_response","","Corresponds to the host ABI function `write_response`.",19,[[["abipointeroffset",6],["abipointer",6]],[["result",4],["oakstatus",4]]]],[11,"write_log_message","","Corresponds to the host ABI function `write_log_message`.",19,[[["abipointeroffset",6],["abipointer",6]],[["result",4],["oakstatus",4]]]],[11,"report_metric","","Corresponds to the host ABI function `report_metric`.",19,[[["abipointeroffset",6],["abipointer",6]],[["result",4],["oakstatus",4]]]],[11,"storage_get_item","","Corresponds to the host ABI function `storage_get_item`.",19,[[["abipointeroffset",6],["abipointer",6]],[["result",4],["oakstatus",4]]]],[11,"alloc","","",19,[[],["abipointer",6]]],[11,"publish_metrics","","",19,[[]]],[11,"create","","",20,[[["logger",3],["option",4],["arc",3],["arc",3],["lookupdata",3],["vec",3],["boxedextension",6]],["result",6]]],[11,"handle_invoke","","",20,[[["request",3]]]],[11,"from","oak_functions_loader::proto","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"into_request","","",0,[[],["request",3]]],[11,"vzip","","",0,[[]]],[11,"into_any_arc","","",0,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",0,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",0,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",0,[[],["any",8]]],[11,"as_any_mut","","",0,[[],["any",8]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"into_request","","",1,[[],["request",3]]],[11,"vzip","","",1,[[]]],[11,"into_any_arc","","",1,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",1,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",1,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",1,[[],["any",8]]],[11,"as_any_mut","","",1,[[],["any",8]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"into_request","","",2,[[],["request",3]]],[11,"vzip","","",2,[[]]],[11,"into_any_arc","","",2,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",2,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",2,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",2,[[],["any",8]]],[11,"as_any_mut","","",2,[[],["any",8]]],[11,"from","oak_functions_loader::proto::attested_invoke_request","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"into_request","","",3,[[],["request",3]]],[11,"vzip","","",3,[[]]],[11,"into_any_arc","","",3,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",3,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",3,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",3,[[],["any",8]]],[11,"as_any_mut","","",3,[[],["any",8]]],[11,"from","oak_functions_loader::proto::attested_invoke_response","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"into_request","","",4,[[],["request",3]]],[11,"vzip","","",4,[[]]],[11,"into_any_arc","","",4,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",4,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",4,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",4,[[],["any",8]]],[11,"as_any_mut","","",4,[[],["any",8]]],[11,"from","oak_functions_loader::proto::remote_attestation_server","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"poll_ready","","",6,[[["context",3]],[["result",4],["poll",4]]]],[11,"call","","",6,[[["request",3]]]],[11,"into_request","","",6,[[],["request",3]]],[11,"vzip","","",6,[[]]],[11,"into_any_arc","","",6,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",6,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",6,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",6,[[],["any",8]]],[11,"as_any_mut","","",6,[[],["any",8]]],[11,"from","oak_functions_loader::attestation","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"into_request","","",7,[[],["request",3]]],[11,"vzip","","",7,[[]]],[11,"into_any_arc","","",7,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",7,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",7,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",7,[[],["any",8]]],[11,"as_any_mut","","",7,[[],["any",8]]],[11,"from","oak_functions_loader::logger","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"into_request","","",8,[[],["request",3]]],[11,"vzip","","",8,[[]]],[11,"into_any_arc","","",8,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",8,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",8,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",8,[[],["any",8]]],[11,"as_any_mut","","",8,[[],["any",8]]],[11,"from","oak_functions_loader::lookup","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"into_request","","",10,[[],["request",3]]],[11,"vzip","","",10,[[]]],[11,"into_any_arc","","",10,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",10,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",10,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",10,[[],["any",8]]],[11,"as_any_mut","","",10,[[],["any",8]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"to_owned","","",9,[[]]],[11,"clone_into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"into_request","","",9,[[],["request",3]]],[11,"vzip","","",9,[[]]],[11,"into_any_arc","","",9,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",9,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",9,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",9,[[],["any",8]]],[11,"as_any_mut","","",9,[[],["any",8]]],[11,"from","oak_functions_loader::metrics","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"into_request","","",11,[[],["request",3]]],[11,"vzip","","",11,[[]]],[11,"into_any_arc","","",11,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",11,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",11,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",11,[[],["any",8]]],[11,"as_any_mut","","",11,[[],["any",8]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"into_request","","",14,[[],["request",3]]],[11,"vzip","","",14,[[]]],[11,"into_any_arc","","",14,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",14,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",14,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",14,[[],["any",8]]],[11,"as_any_mut","","",14,[[],["any",8]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"into_request","","",15,[[],["request",3]]],[11,"vzip","","",15,[[]]],[11,"into_any_arc","","",15,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",15,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",15,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",15,[[],["any",8]]],[11,"as_any_mut","","",15,[[],["any",8]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"into_request","","",12,[[],["request",3]]],[11,"vzip","","",12,[[]]],[11,"into_any_arc","","",12,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",12,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",12,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",12,[[],["any",8]]],[11,"as_any_mut","","",12,[[],["any",8]]],[11,"from","oak_functions_loader::server","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"into_request","","",16,[[],["request",3]]],[11,"vzip","","",16,[[]]],[11,"into_any_arc","","",16,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",16,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",16,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",16,[[],["any",8]]],[11,"as_any_mut","","",16,[[],["any",8]]],[11,"from","","",21,[[]]],[11,"into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"into_request","","",21,[[],["request",3]]],[11,"vzip","","",21,[[]]],[11,"into_any_arc","","",21,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",21,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",21,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",21,[[],["any",8]]],[11,"as_any_mut","","",21,[[],["any",8]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"into_request","","",17,[[],["request",3]]],[11,"vzip","","",17,[[]]],[11,"into_any_arc","","",17,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",17,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",17,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",17,[[],["any",8]]],[11,"as_any_mut","","",17,[[],["any",8]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"into_request","","",19,[[],["request",3]]],[11,"vzip","","",19,[[]]],[11,"into_any","","",19,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",19,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",19,[[],["any",8]]],[11,"as_any_mut","","",19,[[],["any",8]]],[11,"from","","",20,[[]]],[11,"into","","",20,[[]]],[11,"to_owned","","",20,[[]]],[11,"clone_into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"into_request","","",20,[[],["request",3]]],[11,"vzip","","",20,[[]]],[11,"into_any_arc","","",20,[[["arc",3]],[["arc",3],["any",8]]]],[11,"into_any","","",20,[[["box",3]],[["any",8],["box",3]]]],[11,"into_any_rc","","",20,[[["rc",3]],[["rc",3],["any",8]]]],[11,"as_any","","",20,[[],["any",8]]],[11,"as_any_mut","","",20,[[],["any",8]]],[11,"attested_invoke","oak_functions_loader::attestation","",7,[[["streaming",3],["request",3]],[["pin",3],["box",3]]]],[11,"clone","oak_functions_loader::proto","",0,[[],["request",3]]],[11,"clone","","",1,[[],["attestedinvokerequest",3]]],[11,"clone","oak_functions_loader::proto::attested_invoke_request","",3,[[],["requesttype",4]]],[11,"clone","oak_functions_loader::proto","",2,[[],["attestedinvokeresponse",3]]],[11,"clone","oak_functions_loader::proto::attested_invoke_response","",4,[[],["responsetype",4]]],[11,"clone","oak_functions_loader::proto::remote_attestation_server","",6,[[]]],[11,"clone","oak_functions_loader::logger","",8,[[],["logger",3]]],[11,"clone","oak_functions_loader::lookup","",9,[[],["lookupdataauth",4]]],[11,"clone","oak_functions_loader::metrics","",12,[[],["bucketconfig",4]]],[11,"clone","oak_functions_loader::server","",16,[[],["policy",3]]],[11,"clone","","",20,[[],["wasmhandler",3]]],[11,"default","oak_functions_loader::proto","",0,[[]]],[11,"default","","",1,[[]]],[11,"default","","",2,[[]]],[11,"default","oak_functions_loader::logger","",8,[[]]],[11,"eq","oak_functions_loader::proto","",0,[[["request",3]]]],[11,"ne","","",0,[[["request",3]]]],[11,"eq","","",1,[[["attestedinvokerequest",3]]]],[11,"ne","","",1,[[["attestedinvokerequest",3]]]],[11,"eq","oak_functions_loader::proto::attested_invoke_request","",3,[[["requesttype",4]]]],[11,"ne","","",3,[[["requesttype",4]]]],[11,"eq","oak_functions_loader::proto","",2,[[["attestedinvokeresponse",3]]]],[11,"ne","","",2,[[["attestedinvokeresponse",3]]]],[11,"eq","oak_functions_loader::proto::attested_invoke_response","",4,[[["responsetype",4]]]],[11,"ne","","",4,[[["responsetype",4]]]],[11,"fmt","oak_functions_loader::proto","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::proto::attested_invoke_request","",3,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::proto","",2,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::proto::attested_invoke_response","",4,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::proto::remote_attestation_server","",6,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::lookup","",9,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::metrics","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","oak_functions_loader::server","",16,[[["formatter",3]],["result",6]]],[11,"try_from","","",21,[[["policy",3]],["result",6]]],[11,"poll_ready","oak_functions_loader::proto::remote_attestation_server","",6,[[["context",3]],[["result",4],["poll",4]]]],[11,"call","","",6,[[["request",3]]]],[11,"encode_raw","oak_functions_loader::proto","",0,[[]]],[11,"merge_field","","",0,[[["wiretype",4],["decodecontext",3]],[["decodeerror",3],["result",4]]]],[11,"encoded_len","","",0,[[]]],[11,"clear","","",0,[[]]],[11,"encode_raw","","",1,[[]]],[11,"merge_field","","",1,[[["wiretype",4],["decodecontext",3]],[["decodeerror",3],["result",4]]]],[11,"encoded_len","","",1,[[]]],[11,"clear","","",1,[[]]],[11,"encode_raw","","",2,[[]]],[11,"merge_field","","",2,[[["wiretype",4],["decodecontext",3]],[["decodeerror",3],["result",4]]]],[11,"encoded_len","","",2,[[]]],[11,"clear","","",2,[[]]],[11,"deserialize","oak_functions_loader::lookup","",9,[[],["result",4]]],[11,"deserialize","oak_functions_loader::metrics","",11,[[],["result",4]]],[11,"deserialize","","",12,[[],["result",4]]],[11,"deserialize","oak_functions_loader::server","",16,[[],["result",4]]],[11,"invoke_index","","Invocation of a host function specified by its registered…",19,[[["runtimeargs",3]],[["option",4],["trap",3],["result",4]]]],[11,"resolve_func","","",19,[[["signature",3]],[["error",4],["funcref",3],["result",4]]]]],"p":[[3,"Request"],[3,"AttestedInvokeRequest"],[3,"AttestedInvokeResponse"],[4,"RequestType"],[4,"ResponseType"],[8,"RemoteAttestation"],[3,"RemoteAttestationServer"],[3,"AttestationServer"],[3,"Logger"],[4,"LookupDataAuth"],[3,"LookupData"],[3,"PrivateMetricsConfig"],[4,"BucketConfig"],[13,"Sum"],[3,"PrivateMetricsAggregator"],[3,"PrivateMetricsProxy"],[3,"Policy"],[3,"ExtensionResult"],[8,"OakApiNativeExtension"],[3,"WasmState"],[3,"WasmHandler"],[3,"ValidatedPolicy"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);