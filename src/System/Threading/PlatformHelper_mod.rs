#[cfg(feature = "System+Threading+PlatformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+PlatformHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::PlatformHelper =>
    "System.Threading"."PlatformHelper"
);
#[cfg(feature = "System+Threading+PlatformHelper")]
impl std::ops::Deref for crate::System::Threading::PlatformHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+PlatformHelper")]
impl std::ops::DerefMut for crate::System::Threading::PlatformHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+PlatformHelper")]
impl crate::System::Threading::PlatformHelper {}
#[cfg(feature = "System+Threading+PlatformHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::PlatformHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
