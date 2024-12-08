#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPackInfo {
    __cordl_parent: crate::System::Object,
    pub _name_k__BackingField: *mut crate::System::String,
    pub _status_k__BackingField: crate::UnityEngine::Android::AndroidAssetPackStatus,
    pub _size_k__BackingField: u64,
    pub _bytesDownloaded_k__BackingField: u64,
    pub _transferProgress_k__BackingField: f32,
    pub _error_k__BackingField: crate::UnityEngine::Android::AndroidAssetPackError,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidAssetPackInfo =>
    "UnityEngine.Android"."AndroidAssetPackInfo"
);
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidAssetPackInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidAssetPackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl crate::UnityEngine::Android::AndroidAssetPackInfo {
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        status: crate::UnityEngine::Android::AndroidAssetPackStatus,
        _cordl_size: u64,
        bytesDownloaded: u64,
        transferProgress: f32,
        error: crate::UnityEngine::Android::AndroidAssetPackError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (name, status, _cordl_size, bytesDownloaded, transferProgress, error),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        status: crate::UnityEngine::Android::AndroidAssetPackStatus,
        _cordl_size: u64,
        bytesDownloaded: u64,
        transferProgress: f32,
        error: crate::UnityEngine::Android::AndroidAssetPackError,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, status, _cordl_size, bytesDownloaded, transferProgress, error),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
