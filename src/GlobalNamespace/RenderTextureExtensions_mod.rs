#[cfg(feature = "RenderTextureExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderTextureExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "RenderTextureExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RenderTextureExtensions => ""
    ."RenderTextureExtensions"
);
#[cfg(feature = "RenderTextureExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::RenderTextureExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RenderTextureExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::RenderTextureExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RenderTextureExtensions")]
impl crate::GlobalNamespace::RenderTextureExtensions {}
#[cfg(feature = "RenderTextureExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RenderTextureExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
