#[cfg(feature = "PyramidBloomMainEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PyramidBloomMainEffectSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainEffectSO>,
    pub _bloomRenderer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PyramidBloomRendererSO,
    >,
    pub _bloomFog: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogSO>,
    pub _fadeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _mainEffectShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _bloomBlendFactor: f32,
    pub _bloomRadius: f32,
    pub _bloomIntensity: f32,
    pub _downBloomIntensityOffset: f32,
    pub _pyramidWeightsParam: f32,
    pub _alphaWeights: f32,
    pub _preFilterPass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _downsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _upsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _finalUpsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
    pub _bloomTextureWidth: i32,
    pub _baseColorBoost: f32,
    pub _baseColorBoostThreshold: f32,
    pub _fadeMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _mainEffectMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "PyramidBloomMainEffectSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PyramidBloomMainEffectSO => ""
    ."PyramidBloomMainEffectSO"
);
#[cfg(feature = "PyramidBloomMainEffectSO")]
impl std::ops::Deref for crate::GlobalNamespace::PyramidBloomMainEffectSO {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainEffectSO>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PyramidBloomMainEffectSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PyramidBloomMainEffectSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PyramidBloomMainEffectSO")]
impl crate::GlobalNamespace::PyramidBloomMainEffectSO {
    pub fn LazyInitializeMaterials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LazyInitializeMaterials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PreRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PreRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        fade: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (src, dest, fade))?;
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
    pub fn get_hasPostProcessEffect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPostProcessEffect", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PyramidBloomMainEffectSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PyramidBloomMainEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
