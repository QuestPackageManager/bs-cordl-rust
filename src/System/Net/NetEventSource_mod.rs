#[cfg(feature = "System+Net+NetEventSource")]
#[repr(C)]
#[derive(Debug)]
pub struct NetEventSource {
    __cordl_parent: crate::System::Diagnostics::Tracing::EventSource,
}
#[cfg(feature = "System+Net+NetEventSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetEventSource => "System.Net"
    ."NetEventSource"
);
#[cfg(feature = "System+Net+NetEventSource")]
impl std::ops::Deref for crate::System::Net::NetEventSource {
    type Target = crate::System::Diagnostics::Tracing::EventSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource")]
impl std::ops::DerefMut for crate::System::Net::NetEventSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource")]
impl crate::System::Net::NetEventSource {
    #[cfg(feature = "System+Net+NetEventSource+Keywords")]
    pub type Keywords = crate::System::Net::NetEventSource_Keywords;
    pub fn Associate(
        &mut self,
        thisOrContextObject: *mut crate::System::String,
        memberName: *mut crate::System::String,
        first: *mut crate::System::String,
        second: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Associate", (thisOrContextObject, memberName, first, second))?;
        Ok(__cordl_ret)
    }
    pub fn CriticalFailure(
        &mut self,
        thisOrContextObject: *mut crate::System::String,
        memberName: *mut crate::System::String,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CriticalFailure", (thisOrContextObject, memberName, message))?;
        Ok(__cordl_ret)
    }
    pub fn Enter(
        &mut self,
        thisOrContextObject: *mut crate::System::String,
        memberName: *mut crate::System::String,
        parameters: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enter", (thisOrContextObject, memberName, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn ErrorMessage(
        &mut self,
        thisOrContextObject: *mut crate::System::String,
        memberName: *mut crate::System::String,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ErrorMessage", (thisOrContextObject, memberName, message))?;
        Ok(__cordl_ret)
    }
    pub fn Exit(
        &mut self,
        thisOrContextObject: *mut crate::System::String,
        memberName: *mut crate::System::String,
        result: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Exit", (thisOrContextObject, memberName, result))?;
        Ok(__cordl_ret)
    }
    pub fn Info(
        &mut self,
        thisOrContextObject: *mut crate::System::String,
        memberName: *mut crate::System::String,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Info", (thisOrContextObject, memberName, message))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn WriteEvent(
        &mut self,
        eventId: i32,
        arg1: *mut crate::System::String,
        arg2: *mut crate::System::String,
        arg3: *mut crate::System::String,
        arg4: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteEvent", (eventId, arg1, arg2, arg3, arg4))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "System+Net+NetEventSource")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NetEventSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
#[repr(C)]
#[derive(Debug)]
pub struct NetEventSource_Keywords {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::NetEventSource_Keywords =>
    "System.Net"."NetEventSource/Keywords"
);
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl std::ops::Deref for crate::System::Net::NetEventSource_Keywords {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl std::ops::DerefMut for crate::System::Net::NetEventSource_Keywords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl crate::System::Net::NetEventSource_Keywords {}
#[cfg(feature = "System+Net+NetEventSource+Keywords")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::NetEventSource_Keywords {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
