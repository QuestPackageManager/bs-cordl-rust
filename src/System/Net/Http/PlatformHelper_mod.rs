#[cfg(feature = "System+Net+Http+PlatformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::PlatformHelper =>
    "System.Net.Http"."PlatformHelper"
);
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl std::ops::Deref for crate::System::Net::Http::PlatformHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl std::ops::DerefMut for crate::System::Net::Http::PlatformHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl crate::System::Net::Http::PlatformHelper {}
#[cfg(feature = "System+Net+Http+PlatformHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::PlatformHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
