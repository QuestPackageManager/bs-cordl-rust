#[cfg(feature = "ConnectedPlayerHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ConnectedPlayerHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ConnectedPlayerHelpers => ""
    ."ConnectedPlayerHelpers"
);
#[cfg(feature = "ConnectedPlayerHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectedPlayerHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerHelpers")]
impl crate::GlobalNamespace::ConnectedPlayerHelpers {}
#[cfg(feature = "ConnectedPlayerHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
