#[cfg(feature = "ENet+Library")]
#[repr(C)]
#[derive(Debug)]
pub struct Library {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ENet+Library")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::Library => "ENet"."Library"
);
#[cfg(feature = "ENet+Library")]
impl std::ops::Deref for crate::ENet::Library {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Library")]
impl std::ops::DerefMut for crate::ENet::Library {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+Library")]
impl crate::ENet::Library {
    pub const maxChannelCount: u32 = 4287627136u32;
    pub const maxPacketSize: u32 = 194u32;
    pub const maxPeers: u32 = 12779407u32;
    pub const throttleAcceleration: u32 = 2291335682u32;
    pub const throttleDeceleration: u32 = 545821442u32;
    pub const throttleInterval: u32 = 2468382867u32;
    pub const throttleScale: u32 = 2466382368u32;
    pub const throttleThreshold: u32 = 33693736u32;
    pub const timeoutLimit: u32 = 3230176032u32;
    pub const timeoutMaximum: u32 = 812974272u32;
    pub const timeoutMinimum: u32 = 12617875u32;
    pub const version: u32 = 117703360u32;
}
#[cfg(feature = "ENet+Library")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::Library {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
