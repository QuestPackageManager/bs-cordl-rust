#[cfg(feature = "BloomFogEnvironmentParams")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFogEnvironmentParams {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
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
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomFogEnvironmentParams => ""
    ."BloomFogEnvironmentParams"
);
#[cfg(feature = "BloomFogEnvironmentParams")]
impl std::ops::Deref for crate::GlobalNamespace::BloomFogEnvironmentParams {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogEnvironmentParams")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomFogEnvironmentParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogEnvironmentParams")]
impl crate::GlobalNamespace::BloomFogEnvironmentParams {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "BloomFogEnvironmentParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomFogEnvironmentParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
