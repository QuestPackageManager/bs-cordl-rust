#[cfg(feature = "BakedLightTexturePacking")]
#[repr(C)]
#[derive(Debug)]
pub struct BakedLightTexturePacking {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BakedLightTexturePacking")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BakedLightTexturePacking => ""
    ."BakedLightTexturePacking"
);
#[cfg(feature = "BakedLightTexturePacking")]
impl std::ops::Deref for crate::GlobalNamespace::BakedLightTexturePacking {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl std::ops::DerefMut for crate::GlobalNamespace::BakedLightTexturePacking {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl crate::GlobalNamespace::BakedLightTexturePacking {
    pub const kBakedLightTexturePackingShaderName: &'static str = "Hidden/BakedLightTexturePacking";
    pub fn PackTextures(
        textures: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::UnityEngine::RenderTexture,
            >,
        >,
        descriptor: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PackTextures", (textures, descriptor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BakedLightTexturePacking")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BakedLightTexturePacking {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
