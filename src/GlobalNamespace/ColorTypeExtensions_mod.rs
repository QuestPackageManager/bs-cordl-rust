#[cfg(feature = "ColorTypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorTypeExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ColorTypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ColorTypeExtensions => ""."ColorTypeExtensions"
);
#[cfg(feature = "ColorTypeExtensions")]
impl std::ops::Deref for ColorTypeExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorTypeExtensions")]
impl std::ops::DerefMut for ColorTypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorTypeExtensions")]
impl ColorTypeExtensions {}
#[cfg(feature = "ColorTypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for ColorTypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}