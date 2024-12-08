#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueueOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestQueueOperation {
    __cordl_parent: crate::System::Object,
    pub m_Completed: bool,
    pub Result: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    pub OnComplete: *mut crate::System::Action_1<
        *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    >,
    pub m_WebRequest: *mut crate::UnityEngine::Networking::UnityWebRequest,
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueueOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::WebRequestQueueOperation =>
    "UnityEngine.ResourceManagement"."WebRequestQueueOperation"
);
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueueOperation")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::WebRequestQueueOperation {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueueOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::WebRequestQueueOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueueOperation")]
impl crate::UnityEngine::ResourceManagement::WebRequestQueueOperation {
    pub fn _ctor(
        &mut self,
        request: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request))?;
        Ok(__cordl_ret)
    }
    pub fn set_WebRequest(
        &mut self,
        value: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WebRequest", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDone(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDone", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WebRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::UnityWebRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::UnityWebRequest = __cordl_object
            .invoke("get_WebRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn Complete(
        &mut self,
        asyncOp: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Complete", (asyncOp))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        request: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueueOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::WebRequestQueueOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
