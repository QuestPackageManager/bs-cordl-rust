#[cfg(feature = "System+IPv4AddressHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct IPv4AddressHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IPv4AddressHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IPv4AddressHelper => "System"
    ."IPv4AddressHelper"
);
#[cfg(feature = "System+IPv4AddressHelper")]
impl std::ops::Deref for crate::System::IPv4AddressHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv4AddressHelper")]
impl std::ops::DerefMut for crate::System::IPv4AddressHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IPv4AddressHelper")]
impl crate::System::IPv4AddressHelper {}
#[cfg(feature = "System+IPv4AddressHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IPv4AddressHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
