#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+UnityWebRequestOperation"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::UnityEngine::Networking::UnityWebRequest,
    >,
    pub m_UWR: *mut crate::UnityEngine::Networking::UnityWebRequest,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+UnityWebRequestOperation"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::AsyncOperations::UnityWebRequestOperation =>
    "UnityEngine.ResourceManagement.AsyncOperations"."UnityWebRequestOperation"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+UnityWebRequestOperation"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::AsyncOperations::UnityWebRequestOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::UnityEngine::Networking::UnityWebRequest,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+UnityWebRequestOperation"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::AsyncOperations::UnityWebRequestOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+UnityWebRequestOperation"
)]
impl crate::UnityEngine::ResourceManagement::AsyncOperations::UnityWebRequestOperation {
    pub fn _ctor(
        &mut self,
        webRequest: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (webRequest))?;
        Ok(__cordl_ret)
    }
    pub fn _Execute_b__2_0(
        &mut self,
        request: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Execute>b__2_0", (request))?;
        Ok(__cordl_ret)
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        webRequest: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (webRequest))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+AsyncOperations+UnityWebRequestOperation"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::AsyncOperations::UnityWebRequestOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
