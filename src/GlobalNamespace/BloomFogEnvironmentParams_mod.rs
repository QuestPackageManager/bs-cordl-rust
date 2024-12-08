#[cfg(feature = "BloomFogEnvironmentParams")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFogEnvironmentParams {
    __cordl_parent: PersistentScriptableObject,
    pub attenuation: f32,
    pub offset: f32,
    pub heightFogStartY: f32,
    pub heightFogHeight: f32,
    pub autoExposureLimit: f32,
    pub legacyAutoExposure: bool,
    pub noteSpawnIntensity: f32,
}
#[cfg(feature = "BloomFogEnvironmentParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomFogEnvironmentParams => ""
    ."BloomFogEnvironmentParams"
);
#[cfg(feature = "BloomFogEnvironmentParams")]
impl std::ops::Deref for BloomFogEnvironmentParams {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogEnvironmentParams")]
impl std::ops::DerefMut for BloomFogEnvironmentParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogEnvironmentParams")]
impl BloomFogEnvironmentParams {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BloomFogEnvironmentParams")]
impl quest_hook::libil2cpp::ObjectType for BloomFogEnvironmentParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
