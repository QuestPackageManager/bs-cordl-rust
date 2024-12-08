#[cfg(feature = "UISupport")]
#[repr(C)]
#[derive(Debug)]
pub struct UISupport {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UISupport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UISupport => ""."UISupport"
);
#[cfg(feature = "UISupport")]
impl std::ops::Deref for crate::GlobalNamespace::UISupport {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UISupport")]
impl std::ops::DerefMut for crate::GlobalNamespace::UISupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UISupport")]
impl crate::GlobalNamespace::UISupport {}
#[cfg(feature = "UISupport")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UISupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
