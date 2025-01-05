#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassDoubleKawaseBlurTextureEffectSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassEffectSO,
    >,
    pub _bloom1KernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub _bloom1Boost: f32,
    pub _bloom2KernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub _bloom2Boost: f32,
    pub _bloom2Alpha: f32,
    pub _downsample: i32,
    pub _gammaCorrection: bool,
    pub _kawaseBlurRenderer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::KawaseBlurRendererSO,
    >,
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO => ""
    ."BloomPrePassDoubleKawaseBlurTextureEffectSO"
);
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassEffectSO,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Render(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (src, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassDoubleKawaseBlurTextureEffectSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassDoubleKawaseBlurTextureEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
