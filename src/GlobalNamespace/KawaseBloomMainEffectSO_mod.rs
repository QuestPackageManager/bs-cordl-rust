#[cfg(feature = "KawaseBloomMainEffectSO")]
#[repr(C)]
#[derive(Debug)]
pub struct KawaseBloomMainEffectSO {
    __cordl_parent: crate::GlobalNamespace::MainEffectSO,
    pub _kawaseBlurRenderer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::KawaseBlurRendererSO,
    >,
    pub _mainEffectShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _bloomIntensity: f32,
    pub _bloomIterations: i32,
    pub _bloomBoost: f32,
    pub _bloomAlphaWeights: f32,
    pub _bloomTextureWidth: i32,
    pub _baseColorBoost: f32,
    pub _baseColorBoostThreshold: f32,
    pub _mainEffectMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "KawaseBloomMainEffectSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::KawaseBloomMainEffectSO => ""
    ."KawaseBloomMainEffectSO"
);
#[cfg(feature = "KawaseBloomMainEffectSO")]
impl std::ops::Deref for crate::GlobalNamespace::KawaseBloomMainEffectSO {
    type Target = crate::GlobalNamespace::MainEffectSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBloomMainEffectSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::KawaseBloomMainEffectSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBloomMainEffectSO")]
impl crate::GlobalNamespace::KawaseBloomMainEffectSO {
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
#[cfg(feature = "KawaseBloomMainEffectSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::KawaseBloomMainEffectSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
