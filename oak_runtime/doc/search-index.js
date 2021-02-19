var searchIndex = JSON.parse('{\
"oak_runtime":{"doc":"Oak Runtime implementation","i":[[3,"ChannelHalf","oak_runtime","A reference to one half of a `Channel`",null,null],[12,"direction","","",0,null],[3,"RuntimeProxy","","A proxy object that binds together a reference to the…",null,null],[12,"runtime","","",1,null],[12,"node_id","","",1,null],[12,"node_name","","",1,null],[3,"RuntimeConfiguration","","Configuration options that govern the behaviour of the…",null,null],[12,"metrics_port","","Port to run a metrics server on, if provided.",2,null],[12,"introspect_port","","Port to run an introspection server on, if provided.",2,null],[12,"kms_credentials","","Credentials filename for KMS integration, if provided.",2,null],[12,"secure_server_configuration","","Security options for server pseudo-nodes.",2,null],[12,"app_config","","Application configuration.",2,null],[12,"permissions_config","","Permissions configuration.",2,null],[12,"sign_table","","Table that contains signatures and public keys…",2,null],[12,"config_map","","Start-of-day configuration to feed to the running…",2,null],[3,"GrpcConfiguration","","Configuration options related to gRPC pseudo-Nodes.",null,null],[12,"grpc_server_tls_identity","","TLS identity to use for all gRPC Server Nodes.",3,null],[12,"oidc_client_info","","OpenID Connect Authentication client information.",3,null],[12,"grpc_client_root_tls_certificate","","PEM formatted root TLS certificate to use for all gRPC…",3,null],[3,"SignatureTable","","Configuration options table related to Wasm module…",null,null],[12,"values","","Map from Oak module hashes to corresponding signatures.",4,null],[3,"HttpConfiguration","","Configuration options related to HTTP pseudo-Nodes.",null,null],[12,"tls_config","","TLS identity to use for all HTTP Server Nodes.",5,null],[12,"http_client_root_tls_certificate","","PEM formatted root TLS certificate to use for all HTTP…",5,null],[3,"SecureServerConfiguration","","Configuration options for secure HTTP and gRPC pseudo-Nodes.",null,null],[12,"grpc_config","","",6,null],[12,"http_config","","",6,null],[3,"NodePrivilege","","The downgrading (declassification + endorsement) privilege…",null,null],[3,"NodeId","","A unique internal identifier for a Node or pseudo-Node…",null,null],[12,"0","","",7,null],[3,"AuxServer","","Information for managing an associated server.",null,null],[12,"name","","",8,null],[12,"join_handle","","",8,null],[12,"termination_notificiation_sender","","",8,null],[3,"Runtime","","Runtime structure for configuring and running a set of Oak…",null,null],[12,"metrics_data","","",9,null],[4,"ChannelHalfDirection","","The direction of a [`ChannelHalf`].",null,null],[13,"Read","","",10,null],[13,"Write","","",10,null],[4,"NodeReadStatus","","Helper types to indicate whether a channel read operation…",null,null],[13,"Success","","",11,null],[13,"NeedsCapacity","","",11,null],[4,"ReadStatus","","",null,null],[13,"Success","","",12,null],[13,"NeedsCapacity","","",12,null],[4,"LabelReadStatus","","Helper type to indicate whether retrieving a serialized…",null,null],[13,"Success","","",13,null],[13,"NeedsCapacity","","",13,null],[5,"construct_debug_id","","Returns a unique debug_id consisting out of the provided…",null,[[["nodeid",3]],["string",3]]],[0,"auth","","Authentication functionality.",null,null],[0,"oidc_utils","oak_runtime::auth","OpenID Connect utilities for exchanging authorisation…",null,null],[3,"Claims","oak_runtime::auth::oidc_utils","Claims encoded into the identity token. Reference:…",null,null],[3,"ClientInfo","","The OpenID Connect client ID and client secret contained…",null,null],[12,"client_id","","The client ID.",14,null],[12,"client_secret","","The client secret.",14,null],[3,"TokenError","","Represents a token validation error.",null,null],[5,"parse_client_info_json","","Parses the content of the downloaded OpenID Connect client…",null,[[],[["result",4],["clientinfo",3],["box",3]]]],[5,"exchange_code_for_token","","Exchanges an authorisation code for an ID token and…",null,[[]]],[11,"new","","",15,[[],["tokenerror",3]]],[11,"new","oak_runtime","Constructor for [`ChannelHalf`] keeps the underlying…",0,[[["channelhalfdirection",4],["arc",3],["channel",3]]]],[11,"get_channel_id","","Get the ID of the underlying channel. For…",0,[[]]],[11,"get_channel_debug_id","","Returns a unique debug_id used to identify the channel in…",0,[[],["string",3]]],[11,"get_messages","","Get read-only access to the channel\'s messages. For…",0,[[],[["rwlockreadguard",3],["vecdeque",3]]]],[11,"visit_halves","","Visit all channel halves that are reachable via this…",0,[[]]],[11,"wake_waiters","","Wake any threads waiting on the underlying channel.",0,[[]]],[0,"config","","Functionality covering configuration of a Runtime instance.",null,null],[5,"configure_and_run","oak_runtime::config","Configures a [`Runtime`] from the given…",null,[[["runtimeconfiguration",3]],[["result",4],["arc",3],["oakerror",4]]]],[11,"introspection_event","oak_runtime","no-op implementation, introspection events are a debugging…",9,[[["eventdetails",4]]]],[0,"permissions","","",null,null],[3,"PermissionsConfiguration","oak_runtime::permissions","Provides a declarative description of the features that…",null,null],[12,"allow_grpc_server_nodes","","Whether to enable the use of GrpcServerNode.",16,null],[12,"allow_http_server_nodes","","Whether to enable the use of HttpServerNode.",16,null],[12,"allow_log_nodes","","Whether to enable the use of LogNode.",16,null],[12,"allow_insecure_http_egress","","Whether to enable creating an HTTP client for insecure…",16,null],[12,"allow_egress_https_authorities","","List of the allowed authorities (of the form…",16,null],[11,"allowed_creation","","Check if this permissions configuration allows creating a…",16,[[["nodeconfiguration",3]],["result",6]]],[11,"allow_grpc_client","","Checks if creating a GrpcClient node with the given…",16,[[["grpcclientconfiguration",3]],["result",6]]],[11,"allow_http_client","","Checks if creating an HttpClient node with the given…",16,[[["httpclientconfiguration",3]]]],[11,"create_runtime","oak_runtime","Creates a [`Runtime`] instance with a single initial Node…",1,[[["permissionsconfiguration",3],["signaturetable",3],["pathbuf",3],["applicationconfiguration",3],["option",4],["secureserverconfiguration",3]],["runtimeproxy",3]]],[11,"start_runtime","","Configures and runs the protobuf specified…",1,[[["runtimeconfiguration",3]],[["result",4],["oakstatus",4],["handle",6]]]],[11,"is_terminating","","See [`Runtime::is_terminating`].",1,[[]]],[11,"node_create","","Calls [`Runtime::node_create_and_register`] without using…",1,[[["label",3],["nodeconfiguration",3],["handle",6]],[["oakstatus",4],["result",4]]]],[11,"node_create_with_downgrade","","Calls [`Runtime::node_create_and_register`] using the…",1,[[["label",3],["nodeconfiguration",3],["handle",6]],[["oakstatus",4],["result",4]]]],[11,"channel_create","","Calls [`Runtime::channel_create`] without using the Node\'s…",1,[[["label",3]],[["result",4],["oakstatus",4]]]],[11,"channel_create_with_downgrade","","Calls [`Runtime::channel_create`] using the Node\'s…",1,[[["label",3]],[["result",4],["oakstatus",4]]]],[11,"handle_clone","","Calls [`Runtime::handle_clone`].",1,[[["handle",6]],[["result",4],["oakstatus",4],["handle",6]]]],[11,"channel_close","","See [`Runtime::channel_close`].",1,[[["handle",6]],[["oakstatus",4],["result",4]]]],[11,"wait_on_channels","","See [`Runtime::wait_on_channels`].",1,[[],[["vec",3],["oakstatus",4],["result",4]]]],[11,"channel_write","","Calls [`Runtime::channel_write`] without using the Node\'s…",1,[[["nodemessage",3],["handle",6]],[["oakstatus",4],["result",4]]]],[11,"channel_write_with_downgrade","","Calls [`Runtime::channel_write`] using the Node\'s privilege.",1,[[["nodemessage",3],["handle",6]],[["oakstatus",4],["result",4]]]],[11,"channel_read","","Calls [`Runtime::channel_read`] without using the Node\'s…",1,[[["handle",6]],[["result",4],["oakstatus",4],["option",4]]]],[11,"channel_read_with_downgrade","","Calls [`Runtime::channel_read`] using the Node\'s privilege.",1,[[["handle",6]],[["result",4],["oakstatus",4],["option",4]]]],[11,"channel_try_read_message","","Calls [`Runtime::channel_try_read_message`] without the…",1,[[["handle",6]],[["option",4],["oakstatus",4],["result",4]]]],[11,"channel_try_read_message_with_downgrade","","Calls [`Runtime::channel_try_read_message`] using the…",1,[[["handle",6]],[["option",4],["oakstatus",4],["result",4]]]],[11,"get_serialized_channel_label","","See [`Runtime::get_serialized_channel_label`].",1,[[["handle",6]],[["oakstatus",4],["labelreadstatus",4],["result",4]]]],[11,"get_serialized_node_label","","See [`Runtime::get_serialized_node_label`].",1,[[],[["oakstatus",4],["labelreadstatus",4],["result",4]]]],[11,"get_serialized_node_privilege","","See [`Runtime::get_serialized_node_privilege`].",1,[[],[["oakstatus",4],["labelreadstatus",4],["result",4]]]],[11,"get_channel_label","","See [`Runtime::get_channel_label`].",1,[[["handle",6]],[["oakstatus",4],["label",3],["result",4]]]],[11,"channel_direction","","Return the direction of an ABI handle.",1,[[["handle",6]],[["result",4],["oakstatus",4],["channelhalfdirection",4]]]],[11,"metrics_data","","",1,[[],["metrics",3]]],[0,"time","","Roughtime client implementation to provide a source of…",null,null],[3,"RoughtimeServer","oak_runtime::time","Specifies the details of a Roughtime server.",null,null],[3,"RoughtimeClient","","A client for requesting Roughtime from multiple servers.",null,null],[4,"RoughtimeError","","Possible errors returned by the Roughtime client.",null,null],[13,"Base64Error","","",17,null],[13,"InvalidSignature","","",17,null],[13,"IoError","","",17,null],[13,"MidPointTooSmall","","",17,null],[13,"NotEnoughOverlappingIntervals","","",17,null],[12,"actual","oak_runtime::time::RoughtimeError","",18,null],[12,"expected","","",18,null],[13,"RadiusTooLarge","oak_runtime::time","",17,null],[13,"RoughenoughError","","",17,null],[13,"TimeoutError","","",17,null],[5,"get_default_servers","","Gets the default Roughtime servers in the ecosystem.",null,[[],[["vec",3],["roughtimeserver",3]]]],[6,"MicrosSinceEpoch","","Time is given as microseconds since the UNIX epoch…",null,null],[17,"DEFAULT_MIN_OVERLAPPING_INTERVALS","","",null,null],[17,"DEFAULT_MAX_RADIUS_MICROSECONDS","","",null,null],[17,"DEFAULT_TIMEOUT_SECONDS","","",null,null],[17,"DEFAULT_SERVER_RETRIES","","",null,null],[11,"default","","Creates a new Roughtime client with the default settings.",19,[[]]],[11,"new","","Creates a new Roughtime client.",19,[[["vec",3],["roughtimeserver",3]]]],[11,"get_roughtime","","Gets the Roughtime from multiple servers.",19,[[],[["microssinceepoch",6],["roughtimeerror",4],["result",4]]]],[11,"new","","Creates a new instance.",20,[[]]],[0,"tls","oak_runtime","",null,null],[3,"Certificate","oak_runtime::tls","Represents a PEM formatted certificate.",null,null],[3,"TlsConfig","","Represents TLS identity to use for HTTP server pseudo-nodes.",null,null],[11,"parse","","Checks that the input byte vector represents a valid PEM…",21,[[["vec",3]],[["result",6],["certificate",3]]]],[11,"new","","",22,[[],["option",4]]],[11,"new","oak_runtime","",23,[[["hashset",3],["tag",3]]]],[11,"gather_metrics","","Return the accumulated metrics for the `Runtime`.",9,[[],[["metricfamily",3],["vec",3]]]],[11,"is_terminating","","Return whether the [`Runtime`] is terminating.",9,[[]]],[11,"stop","","Signal termination to a [`Runtime`] and wait for its Node…",9,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"into_request","","",0,[[],["request",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"into_request","","",1,[[],["request",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"into_request","","",2,[[],["request",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"into_request","","",3,[[],["request",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"into_request","","",4,[[],["request",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_owned","","",5,[[]]],[11,"clone_into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"into_request","","",5,[[],["request",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"into_request","","",6,[[],["request",3]]],[11,"from","","",23,[[]]],[11,"into","","",23,[[]]],[11,"to_owned","","",23,[[]]],[11,"clone_into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"vzip","","",23,[[]]],[11,"into_request","","",23,[[],["request",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"into_request","","",7,[[],["request",3]]],[11,"equivalent","","",7,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"into_request","","",8,[[],["request",3]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"vzip","","",9,[[]]],[11,"into_request","","",9,[[],["request",3]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_owned","","",10,[[]]],[11,"clone_into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"into_request","","",10,[[],["request",3]]],[11,"equivalent","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"into_request","","",11,[[],["request",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"vzip","","",12,[[]]],[11,"into_request","","",12,[[],["request",3]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"vzip","","",13,[[]]],[11,"into_request","","",13,[[],["request",3]]],[11,"from","oak_runtime::auth::oidc_utils","",24,[[]]],[11,"into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"vzip","","",24,[[]]],[11,"into_request","","",24,[[],["request",3]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"vzip","","",14,[[]]],[11,"into_request","","",14,[[],["request",3]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"to_string","","",15,[[],["string",3]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"vzip","","",15,[[]]],[11,"into_request","","",15,[[],["request",3]]],[11,"from","oak_runtime::permissions","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"vzip","","",16,[[]]],[11,"into_request","","",16,[[],["request",3]]],[11,"from","oak_runtime::time","",20,[[]]],[11,"into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"vzip","","",20,[[]]],[11,"into_request","","",20,[[],["request",3]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"vzip","","",19,[[]]],[11,"into_request","","",19,[[],["request",3]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"to_string","","",17,[[],["string",3]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"vzip","","",17,[[]]],[11,"into_request","","",17,[[],["request",3]]],[11,"from","oak_runtime::tls","",21,[[]]],[11,"into","","",21,[[]]],[11,"to_owned","","",21,[[]]],[11,"clone_into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"encode_hex","","",21,[[]]],[11,"encode_hex_upper","","",21,[[]]],[11,"vzip","","",21,[[]]],[11,"into_request","","",21,[[],["request",3]]],[11,"as_bits","","",21,[[],["bitslice",3]]],[11,"from","","",22,[[]]],[11,"into","","",22,[[]]],[11,"to_owned","","",22,[[]]],[11,"clone_into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"vzip","","",22,[[]]],[11,"into_request","","",22,[[],["request",3]]],[11,"drop","oak_runtime","",0,[[]]],[11,"drop","","Dropping an auxiliary server involves notifying it that it…",8,[[]]],[11,"drop","","",9,[[]]],[11,"as_ref","oak_runtime::tls","",21,[[]]],[11,"from","oak_runtime::time","",17,[[["error",3]]]],[11,"from","","",17,[[["error",4]]]],[11,"from","","",17,[[["decodeerror",4]]]],[11,"clone","oak_runtime::auth::oidc_utils","",14,[[],["clientinfo",3]]],[11,"clone","","",15,[[],["tokenerror",3]]],[11,"clone","oak_runtime","",0,[[]]],[11,"clone","","",10,[[],["channelhalfdirection",4]]],[11,"clone","oak_runtime::permissions","",16,[[],["permissionsconfiguration",3]]],[11,"clone","oak_runtime","",1,[[],["runtimeproxy",3]]],[11,"clone","oak_runtime::tls","",21,[[],["certificate",3]]],[11,"clone","","",22,[[],["tlsconfig",3]]],[11,"clone","oak_runtime","",2,[[],["runtimeconfiguration",3]]],[11,"clone","","",3,[[],["grpcconfiguration",3]]],[11,"clone","","",4,[[],["signaturetable",3]]],[11,"clone","","",5,[[],["httpconfiguration",3]]],[11,"clone","","",6,[[],["secureserverconfiguration",3]]],[11,"clone","","",23,[[],["nodeprivilege",3]]],[11,"clone","","",7,[[],["nodeid",3]]],[11,"default","oak_runtime::permissions","",16,[[],["permissionsconfiguration",3]]],[11,"default","oak_runtime::tls","",22,[[],["tlsconfig",3]]],[11,"default","oak_runtime","",2,[[],["runtimeconfiguration",3]]],[11,"default","","",3,[[],["grpcconfiguration",3]]],[11,"default","","",4,[[],["signaturetable",3]]],[11,"default","","",5,[[],["httpconfiguration",3]]],[11,"default","","",6,[[],["secureserverconfiguration",3]]],[11,"default","","",23,[[],["nodeprivilege",3]]],[11,"cmp","","",7,[[["nodeid",3]],["ordering",4]]],[11,"eq","","",10,[[["channelhalfdirection",4]]]],[11,"eq","","",7,[[["nodeid",3]]]],[11,"ne","","",7,[[["nodeid",3]]]],[11,"partial_cmp","","",7,[[["nodeid",3]],[["ordering",4],["option",4]]]],[11,"lt","","",7,[[["nodeid",3]]]],[11,"le","","",7,[[["nodeid",3]]]],[11,"gt","","",7,[[["nodeid",3]]]],[11,"ge","","",7,[[["nodeid",3]]]],[11,"fmt","oak_runtime::auth::oidc_utils","",24,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","oak_runtime","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","oak_runtime::permissions","",16,[[["formatter",3]],["result",6]]],[11,"fmt","oak_runtime::time","",17,[[["formatter",3]],["result",6]]],[11,"fmt","oak_runtime","",23,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","oak_runtime::auth::oidc_utils","",15,[[["formatter",3]],["result",6]]],[11,"fmt","oak_runtime::time","",17,[[["formatter",3]],["result",6]]],[11,"hash","oak_runtime","",10,[[]]],[11,"hash","","",7,[[]]],[11,"source","oak_runtime::auth::oidc_utils","",15,[[],[["option",4],["error",8]]]],[11,"deserialize","","",24,[[],["result",4]]],[11,"deserialize","","",14,[[],["result",4]]],[11,"deserialize","oak_runtime::permissions","",16,[[],["result",4]]]],"p":[[3,"ChannelHalf"],[3,"RuntimeProxy"],[3,"RuntimeConfiguration"],[3,"GrpcConfiguration"],[3,"SignatureTable"],[3,"HttpConfiguration"],[3,"SecureServerConfiguration"],[3,"NodeId"],[3,"AuxServer"],[3,"Runtime"],[4,"ChannelHalfDirection"],[4,"NodeReadStatus"],[4,"ReadStatus"],[4,"LabelReadStatus"],[3,"ClientInfo"],[3,"TokenError"],[3,"PermissionsConfiguration"],[4,"RoughtimeError"],[13,"NotEnoughOverlappingIntervals"],[3,"RoughtimeClient"],[3,"RoughtimeServer"],[3,"Certificate"],[3,"TlsConfig"],[3,"NodePrivilege"],[3,"Claims"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);