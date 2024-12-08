#[cfg(feature = "UnityEngine+ResourceManagement+Util+UnityWebRequestResult")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestResult {
    __cordl_parent: crate::System::Object,
    pub _Error_k__BackingField: *mut crate::System::String,
    pub _ResponseCode_k__BackingField: i64,
    pub _Result_k__BackingField: crate::UnityEngine::Networking::UnityWebRequest_Result,
    pub _Method_k__BackingField: *mut crate::System::String,
    pub _Url_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+UnityWebRequestResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::UnityWebRequestResult =>
    "UnityEngine.ResourceManagement.Util"."UnityWebRequestResult"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+UnityWebRequestResult")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+UnityWebRequestResult")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+UnityWebRequestResult")]
impl crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult {
    pub fn New(
        request: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request))?;
        Ok(__cordl_object)
    }
    pub fn ShouldRetryDownloadError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldRetryDownloadError", ())?;
        Ok(__cordl_ret)
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
        request: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request))?;
        Ok(__cordl_ret)
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Error", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Method(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Method", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResponseCode(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ResponseCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Networking::UnityWebRequest_Result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Networking::UnityWebRequest_Result = __cordl_object
            .invoke("get_Result", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Url(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Url", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Error(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Error", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+UnityWebRequestResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::UnityWebRequestResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
