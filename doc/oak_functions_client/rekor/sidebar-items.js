initSidebarItems({"fn":[["get_sha256","Computes a SHA-256 digest of `input` and returns it in a form of raw bytes. Returns the hash as a 32-bytes array."],["unmarshal_pem_to_p256_public_key","Parses a PEM-encoded x509/PKIX public key into a `p256::ecdsa::VerifyingKey`."],["verify_rekor_body","Verifies the signature in the `body` over the `contents_bytes`, using the public key in `pem_encoded_public_key_bytes`."],["verify_rekor_log_entry","Verifies a Rekor LogEntry."],["verify_rekor_signature","Parses `log_entry_bytes` into a Rekor LogEntry, and verifies the signature in signedEntryTimestamp using the public key in `pem_encoded_public_key_bytes`."],["verify_signature","Verifies the given base64-encoded signature over the given data bytes, using the given PEM-encoded public key."]],"struct":[["Body","Struct representing the body in a Rekor LogEntry."],["Data","Struct representing the hashed data in the body of a Rekor LogEntry. Based on https://github.com/sigstore/rekor/blob/2978cdc26fdf8f5bfede8459afd9735f0f231a2a/pkg/generated/models/rekord_v001_schema.go#L179."],["GenericSignature","Struct representing a signature in the body of a Rekor LogEntry. Based on https://github.com/sigstore/rekor/blob/2978cdc26fdf8f5bfede8459afd9735f0f231a2a/pkg/generated/models/rekord_v001_schema.go#L383"],["Hash","Struct representing a hash digest. Based on https://github.com/sigstore/rekor/blob/2978cdc26fdf8f5bfede8459afd9735f0f231a2a/pkg/generated/models/rekord_v001_schema.go#L273."],["LogEntry","Struct representing a Rekor LogEntry. Based on https://github.com/sigstore/rekor/blob/2978cdc26fdf8f5bfede8459afd9735f0f231a2a/pkg/generated/models/log_entry.go#L89."],["LogEntryVerification","Struct representing a verification object in a Rekor LogEntry. The verification object in Rekor also contains an inclusion proof. Since we currently don’t verify the inclusion proof in the client, it is omitted from this struct."],["PublicKey","Struct representing a public key included in the body of a Rekor LogEntry. Based on https://github.com/sigstore/rekor/blob/2978cdc26fdf8f5bfede8459afd9735f0f231a2a/pkg/generated/models/rekord_v001_schema.go#L551."],["RekorSignatureBundle","Convenient struct for verifying the `signedEntryTimestamp` in a Rekor LogEntry."],["Spec","Struct representing the `spec` in the body of a Rekor LogEntry. Based on https://github.com/sigstore/rekor/blob/2978cdc26fdf8f5bfede8459afd9735f0f231a2a/pkg/generated/models/rekord_v001_schema.go#L39."]]});