#[cfg(feature = "ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtensionMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ExtensionMethods => ""."ExtensionMethods"
);
#[cfg(feature = "ExtensionMethods")]
impl std::ops::Deref for ExtensionMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl std::ops::DerefMut for ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl ExtensionMethods {}
#[cfg(feature = "ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
