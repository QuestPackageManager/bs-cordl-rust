#[cfg(feature = "BloomPrePassBloomTextureEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBloomTextureEffectSO {
    __cordl_parent: BloomPrePassEffectSO,
    pub _radius: f32,
    pub _intensity: f32,
    pub _downBloomIntensityOffset: f32,
    pub _uniformPyramidWeights: bool,
    pub _pyramidWeightsParam: f32,
    pub _firstUpsampleBrightness: f32,
    pub _finalUpsampleBrightness: f32,
    pub _prefilterPass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _downsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _upsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _finalUpsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _bloomRenderer: *mut PyramidBloomRendererSO,
    pub _bloomFog: *mut BloomFogSO,
}
#[cfg(feature = "BloomPrePassBloomTextureEffectSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassBloomTextureEffectSO => ""
    ."BloomPrePassBloomTextureEffectSO"
);
#[cfg(feature = "BloomPrePassBloomTextureEffectSO")]
impl std::ops::Deref for BloomPrePassBloomTextureEffectSO {
    type Target = BloomPrePassEffectSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBloomTextureEffectSO")]
impl std::ops::DerefMut for BloomPrePassBloomTextureEffectSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBloomTextureEffectSO")]
impl BloomPrePassBloomTextureEffectSO {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Render(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
        dest: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (src, dest))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_toneMapping(&mut self) -> quest_hook::libil2cpp::Result<ToneMapping> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ToneMapping = __cordl_object.invoke("get_toneMapping", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BloomPrePassBloomTextureEffectSO")]
impl quest_hook::libil2cpp::ObjectType for BloomPrePassBloomTextureEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}