#[cfg(feature = "DebugConsoleExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugConsoleExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "DebugConsoleExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DebugConsoleExtensionMethods =>
    ""."DebugConsoleExtensionMethods"
);
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl std::ops::Deref for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl crate::GlobalNamespace::DebugConsoleExtensionMethods {}
#[cfg(feature = "DebugConsoleExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DebugConsoleExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
