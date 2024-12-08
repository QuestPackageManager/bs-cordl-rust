#[cfg(feature = "System+StringExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct StringExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+StringExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::StringExtensions => "System"
    ."StringExtensions"
);
#[cfg(feature = "System+StringExtensions")]
impl std::ops::Deref for crate::System::StringExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+StringExtensions")]
impl std::ops::DerefMut for crate::System::StringExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+StringExtensions")]
impl crate::System::StringExtensions {}
#[cfg(feature = "System+StringExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::StringExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
