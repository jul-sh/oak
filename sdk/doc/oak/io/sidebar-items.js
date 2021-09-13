initSidebarItems({"fn":[["channel_create","Creates a new channel for transmission of [`Encodable`] and [`Decodable`] types."],["channel_create_with_downgrade","Uses the current node’s label-downgrading privilege to create a new channel for transmission of [`Encodable`] and [`Decodable`] types."],["entrypoint_node_create","Creates a node and corresponding inbound channel of the same type, sends an init message to it, and returns a [`Sender`] of command messages; for nodes that are instantiated via the [`crate::entrypoint_command_handler_init`] macro."],["error_from_nonok_status","Map a non-OK [`OakStatus`] value to the nearest available [`std::io::Error`]."],["forward_invocation","Sends [`crate::grpc::Invocation`] (containing [`oak_io::Receiver`] and [`oak_io::Sender`]) through invocation sender if [`oak_io::Receiver`] label flows to invocation sender’s label. If failed - sends error back through [`oak_io::Sender`]."],["node_create","Creates a node and corresponding inbound channel of the same type, and returns [`Sender`] for such channel."],["node_create_with_downgrade","Uses the current node’s label-downgrading privilege to create a node and corresponding inbound channel of the same type, and returns a [`Sender`] for the channel."]],"struct":[["Message","A simple holder for bytes + handles, using internally owned buffers."],["Receiver","Wrapper for a handle to the read half of a channel, allowing to receive data that can be decoded as bytes + handles via the `Decodable` trait."],["Sender","Wrapper for a handle to the send half of a channel, allowing to send data that can be encoded as bytes + handles via the `Encodable` trait."]],"trait":[["Decodable","A trait for objects that can be decoded from bytes + handles."],["Encodable","A trait for objects that can be encoded as bytes + handles."],["ReceiverExt","SDK-specific functionality provided by a receiver."],["SenderExt","Trait for context-dependent functionality on a `Sender`."]]});