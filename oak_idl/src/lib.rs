//
// Copyright 2022 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! A Rust-based IDL implemented via `macro_rules`. It is inspired by
//! <https://github.com/google/tarpc> and
//! <https://fuchsia.dev/fuchsia-src/reference/fidl/language/wire-format>.
//!
//! For each service definition, it generates both a client stub and a service trait, as well as a
//! method to dispatch invocations to a service implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ClientError {
    InvalidRequest,
    InvalidResponse,
    TransportError(TransportError),
}

impl From<TransportError> for ClientError {
    fn from(e: TransportError) -> Self {
        ClientError::TransportError(e)
    }
}

#[derive(Debug)]
pub enum TransportError {
    InvalidMessage,
    InvalidTransactionId,
    InvalidMethodId,
}

/// Unique identifier of a method within a service.
type MethodId = u32;

/// Header of a transactional message.
///
/// See <https://fuchsia.dev/fuchsia-src/reference/fidl/language/wire-format#transactional-messages>
#[derive(Serialize, Deserialize)]
pub struct Header {
    /// Identifies the transaction over the underlying transport. Each transaction (a.k.a. call or
    /// invocation) over the same transport must have a unique transaction id (regardless of the
    /// method_id value).
    ///
    /// The client picks a unique transaction id to put in the request header, and when the server
    /// replies to that message, it also uses the same transaction id in the response message.
    pub transaction_id: u32,

    /// Identifies the method to be invoked, as defined by the IDL.
    pub method_id: MethodId,
}

/// A transactional message.
///
/// The body field contains the serialized payload of the request or response.
///
/// Note that the serialization of the body does not have to match the serialization of the Message
/// itself. For instance, the Message may be serialized via bincode, but the payload may be
/// serialized via protobuf.
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub body: Vec<u8>,
}

/// A message-oriented transport that allows performing invocations.
///
/// Implementations of this trait are responsible for assigning a unique transaction id for each
/// invocation request, and checking whether the transaction id on the response matches that of the
/// request.
pub trait Transport {
    fn invoke(&mut self, request_message: Message) -> Result<Message, TransportError>;
}

/// A wrapper for a message-oriented channel handle, which implements the [`Transport`] trait.
struct Channel {
    handle: u32,
    next_transaction_id: u32,
}

impl Channel {
    #[allow(dead_code)]
    pub fn new(handle: u32) -> Self {
        Self {
            handle,
            next_transaction_id: 0,
        }
    }
}

impl Transport for Channel {
    fn invoke(&mut self, request_message: Message) -> Result<Message, TransportError> {
        let transaction_id = self.next_transaction_id;
        self.next_transaction_id += 1;
        let mut request_message = request_message;
        request_message.header.transaction_id = transaction_id;
        // The serialization of the outer message is fixed by the transport, since the header needs
        // to be deserializable by the other end of the transport, but the body of the
        // request / response is serialized at a higher level, and in potentially different format,
        // since that's handled by the application (client and server).
        let request_bytes =
            bincode::serialize(&request_message).map_err(|_| TransportError::InvalidMessage)?;
        let response_bytes = invoke(self.handle, &request_bytes)?;
        let response_message: Message =
            bincode::deserialize(&response_bytes).map_err(|_| TransportError::InvalidMessage)?;

        if response_message.header.transaction_id != transaction_id {
            return Err(TransportError::InvalidTransactionId);
        }

        Ok(response_message)
    }
}

/// A low-level message-oriented transport over channels. This is meant to emulate the low-level
/// Wasm ABI in Oak, and will eventually be replaced by that.
fn invoke(_channel_handle: u32, _request: &[u8]) -> Result<Vec<u8>, TransportError> {
    unimplemented!()
}

#[cfg(test)]
mod test {

    // service! {
    //     Oak {
    //         42 => fn lookup_data(key: Vec<u8>) -> Option<Vec<u8>>;
    //         10 => fn log(log: String) -> ();
    //     }
    // }

    // The (commented out) macro invocation above results in the following
    // implementation (generated via `cargo rustc --profile=check -- -Zunpretty=expanded`)

    /* START GENERATED BY MACRO */

    pub struct OakClient<T: crate::Transport> {
        transport: T,
    }

    impl<T: crate::Transport> OakClient<T> {
        pub fn new(transport: T) -> Self {
            Self { transport }
        }
        pub fn lookup_data(&mut self, key: Vec<u8>) -> Result<Option<Vec<u8>>, crate::ClientError> {
            let request_body =
                bincode::serialize(&key).map_err(|_| crate::ClientError::InvalidRequest)?;
            let request_message = crate::Message {
                header: crate::Header {
                    transaction_id: 0,
                    method_id: 42,
                },
                body: request_body,
            };
            let response_message = self.transport.invoke(request_message)?;
            let response = bincode::deserialize(&response_message.body)
                .map_err(|_| crate::ClientError::InvalidResponse)?;
            Ok(response)
        }
        pub fn log(&mut self, log: String) -> Result<(), crate::ClientError> {
            let request_body =
                bincode::serialize(&log).map_err(|_| crate::ClientError::InvalidRequest)?;
            let request_message = crate::Message {
                header: crate::Header {
                    transaction_id: 0,
                    method_id: 10,
                },
                body: request_body,
            };
            let response_message = self.transport.invoke(request_message)?;
            let response = bincode::deserialize(&response_message.body)
                .map_err(|_| crate::ClientError::InvalidResponse)?;
            Ok(response)
        }
    }
    pub struct OakServer<S> {
        service: S,
    }
    impl<S: Oak> crate::Transport for OakServer<S> {
        fn invoke(
            &mut self,
            request_message: crate::Message,
        ) -> Result<crate::Message, crate::TransportError> {
            let response_header = crate::Header {
                transaction_id: request_message.header.transaction_id,
                method_id: request_message.header.method_id,
            };
            match request_message.header.method_id {
                42 => {
                    let request: Vec<u8> = bincode::deserialize(&request_message.body).unwrap();
                    let response = self.service.lookup_data(request);
                    let response_body = bincode::serialize(&response).unwrap();
                    Ok(crate::Message {
                        header: response_header,
                        body: response_body,
                    })
                }
                10 => {
                    let request: String = bincode::deserialize(&request_message.body).unwrap();
                    let response = self.service.log(request);
                    let response_body = bincode::serialize(&response).unwrap();
                    Ok(crate::Message {
                        header: response_header,
                        body: response_body,
                    })
                }
                _ => Err(crate::TransportError::InvalidMethodId),
            }
        }
    }
    pub trait Oak: Sized {
        fn lookup_data(&self, key: Vec<u8>) -> Option<Vec<u8>>;
        fn log(&self, log: String) -> ();
        fn serve(self) -> OakServer<Self> {
            OakServer { service: self }
        }
    }

    /* END GENERATED BY MACRO */

    struct OakImpl;

    /// An implementation of the [`Oak`] service trait for testing.
    impl Oak for OakImpl {
        fn lookup_data(&self, key: Vec<u8>) -> Option<Vec<u8>> {
            let h = maplit::hashmap! {
                vec![14, 12] => vec![19, 88]
            };
            h.get(&key).cloned()
        }

        #[allow(clippy::unused_unit)]
        fn log(&self, log: String) -> () {
            eprintln!("log: {log}");
        }
    }

    #[test]
    fn test_lookup_data() {
        let s = OakImpl;
        let transport = s.serve();
        let mut c = OakClient::new(transport);
        let res = c.lookup_data(vec![14, 12]).unwrap();
        assert_eq!(Some(vec![19, 88]), res);
    }
}

// Macro Implementation for reference:

/// This macro generates the following objects (assuming it is invoked with the first argument set
/// to `Name`):
///
/// - a struct named `NameClient`, exposing a method for each method defined in the macro.
/// - a struct named `NameServer`, which implements the [`Transport`] trait, dispatching each
///   request to the appropriate method on the underlying service implementation.
/// - a trait named `Name`, with a method for each method defined in the macro, and an additional
///   default method named `serve` which returns an instance of `NameServer`; the developer of a
///   service would usually define a concrete struct and manually implement this trait for it.
#[macro_export]
macro_rules! service {
    (
        $name:ident {
            $( $method_id:literal => fn $method:ident ( $request_arg:ident : $request_type:ty ) -> $response_type:ty ; )*
        }
    ) => {
        // We use <https://docs.rs/paste/latest/paste/> in order to generate identifiers with
        // different suffixes.
        paste::paste! {
            pub struct [<$name "Client">]<T: $crate::Transport> {
                transport: T,
            }

            impl <T: $crate::Transport>[<$name "Client">]<T> {
                pub fn new(transport: T) -> Self {
                    Self {
                        transport
                    }
                }

                $(
                    pub fn $method(&mut self, $request_arg: $request_type) -> Result<$response_type, $crate::ClientError> {
                        let request_body = bincode::serialize(&$request_arg).map_err(|_| $crate::ClientError::InvalidRequest)?;
                        let request_message = $crate::Message {
                            header: $crate::Header {
                                // An appropriate transaction id is assigned by the underlying transport as part of each invocation.
                                transaction_id: 0,
                                method_id: $method_id,
                            },
                            body: request_body,
                        };
                        let response_message = self.transport.invoke(request_message)?;
                        let response = bincode::deserialize(&response_message.body).map_err(|_| $crate::ClientError::InvalidResponse)?;
                        Ok(response)
                    }
                )*
            }

            pub struct [<$name "Server">]<S> {
                service: S,
            }

            impl <S: $name> $crate::Transport for [<$name "Server">]<S> {
                fn invoke(&mut self, request_message: $crate::Message) -> Result<$crate::Message, $crate::TransportError> {
                    let response_header = $crate::Header {
                        transaction_id: request_message.header.transaction_id,
                        method_id: request_message.header.method_id,
                    };
                    match request_message.header.method_id {
                        $(
                            $method_id => {
                                let request: $request_type = bincode::deserialize(&request_message.body).unwrap();
                                let response = self.service.$method(request);
                                let response_body = bincode::serialize(&response).unwrap();
                                Ok($crate::Message {
                                    header: response_header,
                                    body: response_body,
                                })
                            }
                        )*
                        _ => Err($crate::TransportError::InvalidMethodId)
                    }
                }
            }

            pub trait $name: Sized {

                $(
                    fn $method(&self, $request_arg: $request_type) -> $response_type;
                )*

                fn serve(self) -> [<$name "Server">]<Self> {
                    [<$name "Server">] { service : self }
                }
            }
        }
    };
}
