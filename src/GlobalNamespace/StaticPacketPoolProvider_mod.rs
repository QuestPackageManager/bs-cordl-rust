#[cfg(feature = "StaticPacketPoolProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticPacketPoolProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "StaticPacketPoolProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StaticPacketPoolProvider => ""
    ."StaticPacketPoolProvider"
);
#[cfg(feature = "StaticPacketPoolProvider")]
impl std::ops::Deref for crate::GlobalNamespace::StaticPacketPoolProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StaticPacketPoolProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::StaticPacketPoolProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StaticPacketPoolProvider")]
impl crate::GlobalNamespace::StaticPacketPoolProvider {}
#[cfg(feature = "StaticPacketPoolProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StaticPacketPoolProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
