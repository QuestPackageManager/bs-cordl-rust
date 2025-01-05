#[cfg(feature = "UnityEngine+RenderSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderSettings {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
}
#[cfg(feature = "UnityEngine+RenderSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderSettings => "UnityEngine"
    ."RenderSettings"
);
#[cfg(feature = "UnityEngine+RenderSettings")]
impl std::ops::Deref for crate::UnityEngine::RenderSettings {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RenderSettings")]
impl std::ops::DerefMut for crate::UnityEngine::RenderSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RenderSettings")]
impl crate::UnityEngine::RenderSettings {
    pub fn GetRenderSettings() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reset", ())?;
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
    pub fn get_ambientEquatorColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientEquatorColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientEquatorColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientEquatorColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientGroundColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientGroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientGroundColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientGroundColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientIntensity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientLight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientLight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientLight_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientLight_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::AmbientMode,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::AmbientMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientProbe() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SphericalHarmonicsL2,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::SphericalHarmonicsL2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientProbe", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientProbe_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientProbe_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientSkyColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientSkyColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientSkyColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientSkyColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ambientSkyboxAmount() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ambientSkyboxAmount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customReflection() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_customReflection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customReflectionTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_customReflectionTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultReflection() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultReflection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultReflectionMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::DefaultReflectionMode,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::DefaultReflectionMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultReflectionMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultReflectionResolution() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_defaultReflectionResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flareFadeSpeed() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flareFadeSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flareStrength() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flareStrength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fog() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fog", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fogColor() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fogColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fogColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fogColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fogDensity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fogDensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fogEndDistance() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fogEndDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fogMode() -> quest_hook::libil2cpp::Result<crate::UnityEngine::FogMode> {
        let __cordl_ret: crate::UnityEngine::FogMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fogMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fogStartDistance() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fogStartDistance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_haloStrength() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_haloStrength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reflectionBounces() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_reflectionBounces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reflectionIntensity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_reflectionIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_skybox() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_skybox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtractiveShadowColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_subtractiveShadowColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtractiveShadowColor_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_subtractiveShadowColor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sun() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_sun", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientEquatorColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientEquatorColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientEquatorColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientEquatorColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientGroundColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientGroundColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientGroundColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientGroundColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientIntensity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientIntensity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientLight(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientLight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientLight_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientLight_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientMode(
        value: crate::UnityEngine::Rendering::AmbientMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientProbe(
        value: crate::UnityEngine::Rendering::SphericalHarmonicsL2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientProbe", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientProbe_Injected(
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientProbe_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientSkyColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientSkyColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientSkyColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientSkyColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientSkyboxAmount(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ambientSkyboxAmount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_customReflection(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Cubemap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_customReflection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_customReflectionTexture(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_customReflectionTexture", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultReflectionMode(
        value: crate::UnityEngine::Rendering::DefaultReflectionMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultReflectionMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultReflectionResolution(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_defaultReflectionResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flareFadeSpeed(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_flareFadeSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flareStrength(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_flareStrength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fog(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fog", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fogColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fogColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fogColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fogColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fogDensity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fogDensity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fogEndDistance(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fogEndDistance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fogMode(
        value: crate::UnityEngine::FogMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fogMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fogStartDistance(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_fogStartDistance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_haloStrength(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_haloStrength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_reflectionBounces(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_reflectionBounces", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_reflectionIntensity(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_reflectionIntensity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_skybox(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_skybox", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subtractiveShadowColor(
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_subtractiveShadowColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subtractiveShadowColor_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_subtractiveShadowColor_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sun(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_sun", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+RenderSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RenderSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
