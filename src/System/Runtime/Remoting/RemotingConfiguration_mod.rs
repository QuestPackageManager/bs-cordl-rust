#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
#[repr(C)]
#[derive(Debug)]
pub struct RemotingConfiguration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::RemotingConfiguration
    => "System.Runtime.Remoting"."RemotingConfiguration"
);
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl std::ops::Deref for crate::System::Runtime::Remoting::RemotingConfiguration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::RemotingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl crate::System::Runtime::Remoting::RemotingConfiguration {}
#[cfg(feature = "System+Runtime+Remoting+RemotingConfiguration")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::RemotingConfiguration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
