#[cfg(feature = "Oculus+Platform+Request")]
#[repr(C)]
#[derive(Debug)]
pub struct Request {
    __cordl_parent: crate::System::Object,
    pub callback_: *mut crate::Oculus::Platform::Message_Callback,
    pub _RequestID_k__BackingField: u64,
}
#[cfg(feature = "Oculus+Platform+Request")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Request => "Oculus.Platform"
    ."Request"
);
#[cfg(feature = "Oculus+Platform+Request")]
impl std::ops::Deref for crate::Oculus::Platform::Request {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Request")]
impl std::ops::DerefMut for crate::Oculus::Platform::Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Request")]
impl crate::Oculus::Platform::Request {
    pub fn HandleMessage(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMessage", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn New(requestID: u64) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (requestID))?;
        Ok(__cordl_object)
    }
    pub fn OnComplete(
        &mut self,
        callback: *mut crate::Oculus::Platform::Message_Callback,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Request> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Request = __cordl_object
            .invoke("OnComplete", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        requestID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (requestID))?;
        Ok(__cordl_ret)
    }
    pub fn get_RequestID(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_RequestID", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_RequestID(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RequestID", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+Request")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Request {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
