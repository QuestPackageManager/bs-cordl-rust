#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
#[repr(C)]
#[derive(Debug)]
pub struct MirrorRendererGraphicsSettingsPresets {
    __cordl_parent: crate::GlobalNamespace::NamedPresetsSO,
    pub _presets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
        >,
    >,
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MirrorRendererGraphicsSettingsPresets => ""
    ."MirrorRendererGraphicsSettingsPresets"
);
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl std::ops::Deref for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    type Target = crate::GlobalNamespace::NamedPresetsSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    #[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
    pub type Preset = crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset;
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::NamedPreset>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::NamedPreset>,
        > = __cordl_object.invoke("get_namedPresets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_presets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset,
            >,
        > = __cordl_object.invoke("get_presets", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
#[repr(C)]
#[derive(Debug)]
pub struct MirrorRendererGraphicsSettingsPresets_Preset {
    __cordl_parent: crate::GlobalNamespace::NamedPreset,
    pub mirrorType: crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType,
    pub reflectLayers: crate::UnityEngine::LayerMask,
    pub stereoTextureWidth: i32,
    pub stereoTextureHeight: i32,
    pub monoTextureWidth: i32,
    pub monoTextureHeight: i32,
    pub maxAntiAliasing: i32,
    pub enableBloomPrePassFog: bool,
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset => ""
    ."MirrorRendererGraphicsSettingsPresets/Preset"
);
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl std::ops::Deref
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    type Target = crate::GlobalNamespace::NamedPreset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    #[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
    pub type MirrorType = crate::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType;
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
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MirrorRendererGraphicsSettingsPresets_Preset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preset_MirrorRendererGraphicsSettingsPresets_MirrorType {
    #[default]
    FakeMirror = 1i32,
    None = 0i32,
    RenderedMirror = 2i32,
}
#[cfg(feature = "MirrorRendererGraphicsSettingsPresets+Preset+MirrorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::Preset_MirrorRendererGraphicsSettingsPresets_MirrorType => ""
    ."MirrorRendererGraphicsSettingsPresets/Preset/MirrorType"
);
