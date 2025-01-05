#[cfg(feature = "BloomFogSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomFogSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub _bloomFogEnabled: bool,
    pub _legacyAutoExposureEnabled: bool,
    pub _transition: f32,
    pub _autoExposureLimit: f32,
    pub _noteSpawnIntensity: f32,
    pub _defaultFogParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomFogEnvironmentParams,
    >,
    pub _transitionFogParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomFogEnvironmentParams,
    >,
}
#[cfg(feature = "BloomFogSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomFogSO => ""."BloomFogSO"
);
#[cfg(feature = "BloomFogSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomFogSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomFogSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomFogSO")]
impl crate::GlobalNamespace::BloomFogSO {
    pub const kBloomFogEnabledKeyword: &'static str = "ENABLE_BLOOM_FOG";
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn SetParams(
        &mut self,
        attenuation: f32,
        offset: f32,
        heightFogStartY: f32,
        heightFogHeight: f32,
        autoExposureLimit: f32,
        noteSpawnIntensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetParams",
                (
                    attenuation,
                    offset,
                    heightFogStartY,
                    heightFogHeight,
                    autoExposureLimit,
                    noteSpawnIntensity,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        defaultFogParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (defaultFogParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateShaderParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateShaderParams", ())?;
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
    pub fn get_autoExposureLimit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_autoExposureLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bloomFogEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_bloomFogEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultForParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogEnvironmentParams>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        > = __cordl_object.invoke("get_defaultForParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_legacyAutoExposureEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_legacyAutoExposureEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteSpawnIntensity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteSpawnIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transition(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_transition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionFogParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomFogEnvironmentParams>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        > = __cordl_object.invoke("get_transitionFogParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bloomFogEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bloomFogEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_defaultForParams(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_defaultForParams", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_legacyAutoExposureEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_legacyAutoExposureEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transition(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transitionFogParams(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomFogEnvironmentParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_transitionFogParams", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomFogSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BloomFogSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
