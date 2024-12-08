#[cfg(feature = "System+Runtime+Remoting+Channels+ISecurableChannel")]
#[repr(C)]
#[derive(Debug)]
pub struct ISecurableChannel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ISecurableChannel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Channels::ISecurableChannel =>
    "System.Runtime.Remoting.Channels"."ISecurableChannel"
);
#[cfg(feature = "System+Runtime+Remoting+Channels+ISecurableChannel")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Channels::ISecurableChannel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ISecurableChannel")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Channels::ISecurableChannel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ISecurableChannel")]
impl crate::System::Runtime::Remoting::Channels::ISecurableChannel {
    pub fn set_IsSecured(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsSecured", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Channels+ISecurableChannel")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Channels::ISecurableChannel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
