#[cfg(feature = "Oculus+Platform+Request")]
#[repr(C)]
#[derive(Debug)]
pub struct Request {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub callback_: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_Callback>,
    pub _RequestID_k__BackingField: u64,
}
#[cfg(feature = "Oculus+Platform+Request")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Request {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Request";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Oculus+Platform+Request")]
impl std::ops::Deref for crate::Oculus::Platform::Request {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMessage", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        requestID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (requestID))?;
        Ok(__cordl_object.into())
    }
    pub fn OnComplete(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message_Callback>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = __cordl_object
            .invoke("OnComplete", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunCallbacks(
        limit: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunCallbacks", (limit))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_RequestID(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_RequestID", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
