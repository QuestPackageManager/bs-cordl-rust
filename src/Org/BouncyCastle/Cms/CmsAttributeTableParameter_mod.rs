#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableParameter")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmsAttributeTableParameter {
    ContentType = 0i32,
    Digest = 1i32,
    DigestAlgorithmIdentifier = 3i32,
    Signature = 2i32,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsAttributeTableParameter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsAttributeTableParameter => "Org.BouncyCastle.Cms"
    ."CmsAttributeTableParameter"
);
