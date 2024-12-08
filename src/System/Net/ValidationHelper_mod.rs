#[cfg(feature = "System+Net+ValidationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+ValidationHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ValidationHelper => "System.Net"
    ."ValidationHelper"
);
#[cfg(feature = "System+Net+ValidationHelper")]
impl std::ops::Deref for crate::System::Net::ValidationHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ValidationHelper")]
impl std::ops::DerefMut for crate::System::Net::ValidationHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ValidationHelper")]
impl crate::System::Net::ValidationHelper {}
#[cfg(feature = "System+Net+ValidationHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ValidationHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
