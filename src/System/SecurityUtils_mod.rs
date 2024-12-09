#[cfg(feature = "System+SecurityUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct SecurityUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+SecurityUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::SecurityUtils => "System"
    ."SecurityUtils"
);
#[cfg(feature = "System+SecurityUtils")]
impl std::ops::Deref for crate::System::SecurityUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+SecurityUtils")]
impl std::ops::DerefMut for crate::System::SecurityUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+SecurityUtils")]
impl crate::System::SecurityUtils {}
#[cfg(feature = "System+SecurityUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::System::SecurityUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
