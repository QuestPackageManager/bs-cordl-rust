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
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.IO.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "AssetLoadingSubsystem";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AssetLoadingSubsystem")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AssetLoadingSubsystem")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AssetLoadingSubsystem")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+AssetLoadingSubsystem")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::IO::LowLevel::Unsafe::AssetLoadingSubsystem {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
