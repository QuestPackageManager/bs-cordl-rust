#[cfg(feature = "LightBakingPersistentSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LightBakingPersistentSettingsSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub _settingsApplicator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsApplicatorSO,
    >,
    pub _reflectionsCount: i32,
    pub _colorFromSchemeAlpha: f32,
    pub _bakedLightEditorColors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    >,
    pub _bakedLightDataLoaderPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BakedLightDataLoader,
    >,
    pub _bakedReflectionProbePrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BakedReflectionProbe,
    >,
    pub _lightmapLightsWithIds: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LightmapLightsWithIds,
    >,
    pub _fakeMirrorObjectsInstallerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FakeMirrorObjectsInstaller,
    >,
    pub _fakeMirrorSettingsPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FakeMirrorSettings,
    >,
    pub _defaultDepthOnlyWriteMaterialForFakeMirror: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
}
#[cfg(feature = "LightBakingPersistentSettingsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightBakingPersistentSettingsSO
    => ""."LightBakingPersistentSettingsSO"
);
#[cfg(feature = "LightBakingPersistentSettingsSO")]
impl std::ops::Deref for crate::GlobalNamespace::LightBakingPersistentSettingsSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightBakingPersistentSettingsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightBakingPersistentSettingsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightBakingPersistentSettingsSO")]
impl crate::GlobalNamespace::LightBakingPersistentSettingsSO {
    pub fn ApplySettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplySettings", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_bakedLightDataLoaderPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BakedLightDataLoader>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BakedLightDataLoader,
        > = __cordl_object.invoke("get_bakedLightDataLoaderPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bakedLightEditorColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = __cordl_object.invoke("get_bakedLightEditorColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bakedReflectionProbePrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BakedReflectionProbe>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BakedReflectionProbe,
        > = __cordl_object.invoke("get_bakedReflectionProbePrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorFromSchemeAlpha(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_colorFromSchemeAlpha", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultDepthOnlyWriteMaterialForFakeMirror(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_defaultDepthOnlyWriteMaterialForFakeMirror", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fakeMirrorObjectsInstallerPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FakeMirrorObjectsInstaller>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FakeMirrorObjectsInstaller,
        > = __cordl_object.invoke("get_fakeMirrorObjectsInstallerPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fakeMirrorSettingsPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FakeMirrorSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FakeMirrorSettings,
        > = __cordl_object.invoke("get_fakeMirrorSettingsPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapLightsWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightmapLightsWithIds>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightmapLightsWithIds,
        > = __cordl_object.invoke("get_lightmapLightsWithIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reflectionsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_reflectionsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorFromSchemeAlpha(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorFromSchemeAlpha", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_reflectionsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_reflectionsCount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightBakingPersistentSettingsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightBakingPersistentSettingsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
