#[cfg(feature = "System+Security+SecurityManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SecurityManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Security+SecurityManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::SecurityManager =>
    "System.Security"."SecurityManager"
);
#[cfg(feature = "System+Security+SecurityManager")]
impl std::ops::Deref for crate::System::Security::SecurityManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+SecurityManager")]
impl std::ops::DerefMut for crate::System::Security::SecurityManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+SecurityManager")]
impl crate::System::Security::SecurityManager {}
#[cfg(feature = "System+Security+SecurityManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::SecurityManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}