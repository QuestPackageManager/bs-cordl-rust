#[cfg(feature = "ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ExtensionMethods => ""
    ."ExtensionMethods"
);
#[cfg(feature = "ExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::ExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ExtensionMethods")]
impl crate::GlobalNamespace::ExtensionMethods {}
#[cfg(feature = "ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
