#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AssetLoadingSubsystem")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssetLoadingSubsystem {
    #[default]
    Audio = 4i32,
    ContentLoading = 9i32,
    EntitiesScene = 6i32,
    EntitiesStreamBinaryReader = 7i32,
    FileInfo = 8i32,
    Mesh = 3i32,
    Other = 0i32,
    Scripts = 5i32,
    Texture = 1i32,
    VirtualTexture = 2i32,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AssetLoadingSubsystem")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem => "Unity.IO.LowLevel.Unsafe"
    ."AssetLoadingSubsystem"
);
