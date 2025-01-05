#[cfg(feature = "System+Runtime+Remoting+Messaging+OneWayAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct OneWayAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Attribute>,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+OneWayAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Messaging::OneWayAttribute =>
    "System.Runtime.Remoting.Messaging"."OneWayAttribute"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+OneWayAttribute")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::OneWayAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Attribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+OneWayAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Messaging::OneWayAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+OneWayAttribute")]
impl crate::System::Runtime::Remoting::Messaging::OneWayAttribute {}
#[cfg(feature = "System+Runtime+Remoting+Messaging+OneWayAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::OneWayAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
