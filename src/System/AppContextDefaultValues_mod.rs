#[cfg(feature = "System+AppContextDefaultValues")]
#[repr(C)]
#[derive(Debug)]
pub struct AppContextDefaultValues {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+AppContextDefaultValues")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppContextDefaultValues => "System"
    ."AppContextDefaultValues"
);
#[cfg(feature = "System+AppContextDefaultValues")]
impl std::ops::Deref for crate::System::AppContextDefaultValues {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl std::ops::DerefMut for crate::System::AppContextDefaultValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextDefaultValues")]
impl crate::System::AppContextDefaultValues {}
#[cfg(feature = "System+AppContextDefaultValues")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppContextDefaultValues {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
