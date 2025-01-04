#[cfg(feature = "Org+BouncyCastle+Bcpg+CompressionAlgorithmTag")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionAlgorithmTag {
    #[default]
    BZip2 = 3i32,
    Uncompressed = 0i32,
    ZLib = 2i32,
    Zip = 1i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+CompressionAlgorithmTag")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag
    => "Org.BouncyCastle.Bcpg"."CompressionAlgorithmTag"
);
