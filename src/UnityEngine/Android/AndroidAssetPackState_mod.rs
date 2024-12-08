#[cfg(feature = "UnityEngine+Android+AndroidAssetPackState")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidAssetPackState {
    __cordl_parent: crate::System::Object,
    pub _name_k__BackingField: *mut crate::System::String,
    pub _status_k__BackingField: crate::UnityEngine::Android::AndroidAssetPackStatus,
    pub _error_k__BackingField: crate::UnityEngine::Android::AndroidAssetPackError,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidAssetPackState =>
    "UnityEngine.Android"."AndroidAssetPackState"
);
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackState")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidAssetPackState {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackState")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidAssetPackState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackState")]
impl crate::UnityEngine::Android::AndroidAssetPackState {
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        status: crate::UnityEngine::Android::AndroidAssetPackStatus,
        error: crate::UnityEngine::Android::AndroidAssetPackError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, status, error))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        status: crate::UnityEngine::Android::AndroidAssetPackStatus,
        error: crate::UnityEngine::Android::AndroidAssetPackError,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, status, error))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::AndroidAssetPackState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
