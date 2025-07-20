#[cfg(feature = "LevelDataAssetDownloadUpdate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelDataAssetDownloadUpdate {
    pub levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub bytesTotal: u32,
    pub bytesTransferred: u32,
    pub assetDownloadingState: crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState,
}
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelDataAssetDownloadUpdate";
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
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
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
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
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
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
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
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LevelDataAssetDownloadUpdate")]
impl crate::GlobalNamespace::LevelDataAssetDownloadUpdate {
    #[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
    pub type AssetDownloadingState = crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState;
    pub fn _ctor(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bytesTotal: u32,
        bytesTransferred: u32,
        assetDownloadingState: crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::LevelDataAssetDownloadUpdate as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    u32,
                    u32,
                    crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::LevelDataAssetDownloadUpdate as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (levelID, bytesTotal, bytesTransferred, assetDownloadingState),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LevelDataAssetDownloadUpdate_AssetDownloadingState {
    #[default]
    Completed = 2i32,
    Downloading = 1i32,
    PreparingToDownload = 0i32,
}
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelDataAssetDownloadUpdate/AssetDownloadingState";
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
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState {
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
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState {
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
#[cfg(feature = "LevelDataAssetDownloadUpdate+AssetDownloadingState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LevelDataAssetDownloadUpdate_AssetDownloadingState {
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
