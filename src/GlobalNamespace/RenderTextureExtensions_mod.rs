#[cfg(feature = "RenderTextureExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderTextureExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RenderTextureExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RenderTextureExtensions => ""
    ."RenderTextureExtensions"
);
#[cfg(feature = "RenderTextureExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::RenderTextureExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::GlobalNamespace::RenderTextureExtensions {
    pub fn GetTexture2D(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTexture2D", (rt))?;
        Ok(__cordl_ret.into())
    }
}
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
