#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct WebRequestQueue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ResourceManagement::WebRequestQueue
    => "UnityEngine.ResourceManagement"."WebRequestQueue"
);
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl std::ops::Deref for crate::UnityEngine::ResourceManagement::WebRequestQueue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl std::ops::DerefMut for crate::UnityEngine::ResourceManagement::WebRequestQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl crate::UnityEngine::ResourceManagement::WebRequestQueue {
    pub fn BeginWebRequest(
        queueOperation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginWebRequest", (queueOperation))?;
        Ok(__cordl_ret.into())
    }
    pub fn DequeueRequest(
        operation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DequeueRequest", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnWebAsyncOpComplete_AsyncOperation0(
        operation: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnWebAsyncOpComplete", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnWebAsyncOpComplete_UnityWebRequestAsyncOperation1(
        operation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnWebAsyncOpComplete", (operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueRequest(
        request: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QueueRequest", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMaxConcurrentRequests(
        maxRequests: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMaxConcurrentRequests", (maxRequests))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForRequestToBeActive(
        request: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        >,
        millisecondsTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitForRequestToBeActive", (request, millisecondsTimeout))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+WebRequestQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::WebRequestQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
