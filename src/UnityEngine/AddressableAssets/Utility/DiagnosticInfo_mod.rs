#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticInfo {
    __cordl_parent: crate::System::Object,
    pub DisplayName: *mut crate::System::String,
    pub ObjectId: i32,
    pub Dependencies: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Utility::DiagnosticInfo =>
    "UnityEngine.AddressableAssets.Utility"."DiagnosticInfo"
);
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    pub fn CreateEvent(
        &mut self,
        category: *mut crate::System::String,
        eventType: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
        frame: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent = __cordl_object
            .invoke("CreateEvent", (category, eventType, frame, val))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Utility+DiagnosticInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Utility::DiagnosticInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}