#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MainEffectGraphicsSettingsPresetsSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPresetsSO>,
    pub _presets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset,
            >,
        >,
    >,
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO => ""
    ."MainEffectGraphicsSettingsPresetsSO"
);
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO")]
impl std::ops::Deref for crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPresetsSO>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO")]
impl crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO {
    #[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
    pub type Preset = crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset;
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
    pub fn get_namedPresets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPreset>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPreset>,
            >,
        > = __cordl_object.invoke("get_namedPresets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_presets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset,
                >,
            >,
        > = __cordl_object.invoke("get_presets", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
#[repr(C)]
#[derive(Debug)]
pub struct MainEffectGraphicsSettingsPresetsSO_Preset {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPreset>,
    pub mainEffect: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MainEffectSO>,
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset => ""
    ."MainEffectGraphicsSettingsPresetsSO/Preset"
);
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
impl std::ops::Deref
for crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::NamedPreset>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
impl crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset {
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
#[cfg(feature = "MainEffectGraphicsSettingsPresetsSO+Preset")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MainEffectGraphicsSettingsPresetsSO_Preset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
