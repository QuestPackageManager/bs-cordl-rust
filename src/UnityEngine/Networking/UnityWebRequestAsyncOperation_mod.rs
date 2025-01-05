#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestAsyncOperation {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    pub _webRequest_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::UnityWebRequest,
    >,
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::UnityWebRequestAsyncOperation => "UnityEngine.Networking"
    ."UnityWebRequestAsyncOperation"
);
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl std::ops::Deref for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_webRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = __cordl_object.invoke("get_webRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_webRequest(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_webRequest", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Networking+UnityWebRequestAsyncOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::UnityWebRequestAsyncOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
