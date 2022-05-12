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

#![no_std]
#![feature(never_type)]

extern crate alloc;

use alloc::{vec, vec::Vec};
use anyhow::{bail, Context};
use oak_remote_attestation_sessions::{SessionId, SESSION_ID_LENGTH};

pub mod framing;
mod logger;
mod remote_attestation;
mod wasm;

/// Basic hardware abstraction layer for sending data.
pub trait Channel {
    fn send(&mut self, data: &[u8]) -> anyhow::Result<()>;
    fn recv(&mut self, data: &mut [u8]) -> anyhow::Result<()>;
}

/// Length of the entire frame, including the header.
pub type FrameLength = u32;
const FRAME_LENGTH_ENCODED_SIZE: usize = 4;

/// Links multiple frames that make up a single message. Also links request
/// messages with response messages.
pub type InvocationId = u16;
const INVOCATION_ID_ENCODED_SIZE: usize = 2;

/// 0x1 indicates the start of a stream of frames that constitute a message.
/// 0x2 indicates its end.
/// 0x3 - 0x8 must be unset and ignored.
pub type Flags = u8;
const FLAGS_ENCODED_SIZE: usize = 1;

type MethodStatus = u8;
const METHOD_STATUS_ENCODED_SIZE: usize = 1;

/// Encoded the FrameHeader is 8 bytes.
pub const FRAME_HEADER_SIZE: usize = 8;

pub struct Frame {
    pub length: FrameLength,
    pub invocation_id: InvocationId,
    pub flags: Flags,
    pub method_status: MethodStatus,
    pub body: Vec<u8>,
}

pub struct Framed<T: Channel> {
    inner: T,
}

impl<T: Channel> Framed<T> {
    pub fn new(channel: T) -> Self {
        Self { inner: channel }
    }

    pub fn read_frame(&mut self) -> anyhow::Result<Frame> {
        let length = {
            let mut length_bytes = [0; FRAME_LENGTH_ENCODED_SIZE];
            self.inner.recv(&mut length_bytes)?;
            FrameLength::from_le_bytes(length_bytes)
        };
        let invocation_id = {
            let mut invocation_bytes = [0; INVOCATION_ID_ENCODED_SIZE];
            self.inner.recv(&mut invocation_bytes)?;
            InvocationId::from_le_bytes(invocation_bytes)
        };
        let flags = {
            let mut flags_bytes = [0; FLAGS_ENCODED_SIZE];
            self.inner.recv(&mut flags_bytes)?;
            Flags::from_le_bytes(flags_bytes)
        };
        let method_status = {
            let mut method_bytes = [0; METHOD_STATUS_ENCODED_SIZE];
            self.inner.recv(&mut method_bytes)?;
            MethodStatus::from_le_bytes(method_bytes)
        };

        let body_length: u32 = length - FRAME_HEADER_SIZE as u32;
        let mut body: Vec<u8> = vec![
            0;
            usize::try_from(body_length).expect(
                "the supported pointer size is smaller than the frame length"
            )
        ];
        self.inner.recv(&mut body)?;
        Ok(Frame {
            length,
            invocation_id,
            flags,
            method_status,
            body,
        })
    }

    pub fn write_frame(&mut self, frame: Frame) -> anyhow::Result<()> {
        let length: FrameLength = FrameLength::try_from(frame.body.len())
            .map_err(anyhow::Error::msg)
            .context("the frame body is too large")?;
        let encoded_length = length.to_be_bytes();
        self.inner.send(&encoded_length)?;
        self.inner
            .send(&[0; FRAME_HEADER_SIZE - FRAME_LENGTH_ENCODED_SIZE])?;
        self.inner.send(&frame.body)?;
        Ok(())
    }
}

pub struct SerializeableRequest {
    pub session_id: SessionId,
    pub body: Vec<u8>,
}

impl From<SerializeableRequest> for Vec<u8> {
    fn from(serializeable_request: SerializeableRequest) -> Vec<u8> {
        // The payload is the request's body prepended with the 8 byte session_id.
        // This takes adavantage of the session_id's fixed size to avoid needing
        // to use a key/value pair binary serialization protocol.
        let mut serialized_request: Vec<u8> = Vec::with_capacity(
            serializeable_request.session_id.len() + serializeable_request.body.len(),
        );

        serialized_request.extend(serializeable_request.session_id);
        serialized_request.extend(serializeable_request.body);

        serialized_request
    }
}

impl TryFrom<&[u8]> for SerializeableRequest {
    type Error = anyhow::Error;

    fn try_from(serialized_request: &[u8]) -> Result<Self, Self::Error> {
        if serialized_request.len() < SESSION_ID_LENGTH {
            bail!(
                "Message too short to contain a SessionId. The length of a SessionId
                is {} bytes, the message received contained only {} bytes",
                SESSION_ID_LENGTH,
                serialized_request.len()
            );
        }

        let (session_id_slice, request_body_slice) = serialized_request.split_at(SESSION_ID_LENGTH);

        let mut session_id: SessionId = [0; SESSION_ID_LENGTH];
        session_id.copy_from_slice(session_id_slice);
        let body = request_body_slice.to_vec();

        Ok(Self { session_id, body })
    }
}
