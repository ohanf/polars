// magic numbers
pub mod magic {
    pub const GZIP: [u8; 2] = [31, 139];
    pub const ZLIB0: [u8; 2] = [0x78, 0x01];
    pub const ZLIB1: [u8; 2] = [0x78, 0x9C];
    pub const ZLIB2: [u8; 2] = [0x78, 0xDA];
    pub const ZSTD: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];
}

/// Represents the compression algorithms that we have decoders for
pub enum SupportedCompression {
    GZIP,
    ZLIB,
    ZSTD,
}

impl SupportedCompression {
    /// If the given byte slice starts with the "magic" bytes for a supported compression family, return
    /// that family, for unsupported/uncompressed slices, return None
    pub fn check(bytes: &[u8]) -> Option<Self> {
        use magic::*;
        if bytes.len() < 4 {
            // not enough bytes to perform prefix checks
            return None;
        }
        if bytes.starts_with(&ZLIB0) || bytes.starts_with(&ZLIB1) || bytes.starts_with(&ZLIB2) {
            return Some(Self::ZLIB);
        }
        if bytes.starts_with(&GZIP) {
            return Some(Self::GZIP);
        }
        if bytes.starts_with(&ZSTD) {
            return Some(Self::ZSTD);
        }
        None
    }
}

/// check if byte slice appears to be a compression we know how to process
pub fn is_compressed(bytes: &[u8]) -> bool {
    use magic::*;

    bytes.starts_with(&ZLIB0)
        || bytes.starts_with(&ZLIB1)
        || bytes.starts_with(&ZLIB2)
        || bytes.starts_with(&GZIP)
        || bytes.starts_with(&ZSTD)
}
