#[cfg(feature = "JetBrains+Annotations+CollectionAccessType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionAccessType {
    ModifyExistingContent = 2i32,
    None = 0i32,
    Read = 1i32,
    UpdatedContent = 6i32,
}
#[cfg(feature = "JetBrains+Annotations+CollectionAccessType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::JetBrains::Annotations::CollectionAccessType =>
    "JetBrains.Annotations"."CollectionAccessType"
);
