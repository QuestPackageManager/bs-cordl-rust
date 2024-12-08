#[cfg(feature = "System+Runtime+Remoting+Messaging+IInternalMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct IInternalMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IInternalMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::IInternalMessage =>
    "System.Runtime.Remoting.Messaging"."IInternalMessage"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+IInternalMessage")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::IInternalMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IInternalMessage")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::IInternalMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IInternalMessage")]
impl crate::System::Runtime::Remoting::Messaging::IInternalMessage {
    pub fn set_Uri(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Uri", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TargetIdentity(
        &mut self,
        value: *mut crate::System::Runtime::Remoting::Identity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TargetIdentity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_TargetIdentity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Runtime::Remoting::Identity> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Remoting::Identity = __cordl_object
            .invoke("get_TargetIdentity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Uri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Uri", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+IInternalMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::IInternalMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
