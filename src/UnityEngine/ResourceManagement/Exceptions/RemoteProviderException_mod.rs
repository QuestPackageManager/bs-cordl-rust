#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+RemoteProviderException")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteProviderException {
    __cordl_parent: crate::UnityEngine::ResourceManagement::Exceptions::ProviderException,
    pub _WebRequestResult_k__BackingField: *mut crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+RemoteProviderException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Exceptions::RemoteProviderException =>
    "UnityEngine.ResourceManagement.Exceptions"."RemoteProviderException"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+RemoteProviderException")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Exceptions::RemoteProviderException {
    type Target = crate::UnityEngine::ResourceManagement::Exceptions::ProviderException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+RemoteProviderException")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Exceptions::RemoteProviderException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+RemoteProviderException")]
impl crate::UnityEngine::ResourceManagement::Exceptions::RemoteProviderException {
    pub fn New(
        message: *mut crate::System::String,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        uwrResult: *mut crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, location, uwrResult, innerException))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        message: *mut crate::System::String,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        uwrResult: *mut crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, location, uwrResult, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn get_Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Message", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WebRequestResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult = __cordl_object
            .invoke("get_WebRequestResult", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Exceptions+RemoteProviderException")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Exceptions::RemoteProviderException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
