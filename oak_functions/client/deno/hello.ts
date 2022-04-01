import { UnaryRequest } from "./proto/unary_server.js";

const URI = "http://127.0.0.1:8080/oak.session.unary.v1.UnarySession/Message";

const CONTENT_TYPE = "application/grpc-web";
const SESSION_ID_SIZE = 8;

let unaryRequest = new UnaryRequest({
  session_id: new Uint8Array(SESSION_ID_SIZE),
  body: new Uint8Array(256),
});

let body = encodeBody(unaryRequest.toBytes());

const response = await fetch(URI, {
  method: "POST",
  headers: {
    "Content-Type": CONTENT_TYPE,
    "Accept": CONTENT_TYPE,
  },
  body,
});

console.log(await response);
const content = await response.blob();
console.log(response.headers.get("grpc-message"));
console.log(content.size);

// Construct the request body, conformant with https://grpc.github.io/grpc/core/md_doc__p_r_o_t_o_c_o_l-_h_t_t_p2.html
function encodeBody(message: Uint8Array): Uint8Array {
  let grpcHeaders = new ArrayBuffer(4);
  let grpcHeadersView = new DataView(grpcHeaders);
  // Set the compression flag to 0, indicating no compression
  grpcHeadersView.setUint8(0, 0);
  // Set the message length
  grpcHeadersView.setUint32(0, message.length, false);
  // Finally return the prefixed message
  return new Uint8Array([
    ...new Uint8Array(grpcHeaders),
    ...message,
  ]);
}
