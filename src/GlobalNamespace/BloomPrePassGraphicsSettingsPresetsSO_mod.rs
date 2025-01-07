#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassGraphicsSettingsPresetsSO {
    __cordl_parent: crate::GlobalNamespace::NamedPresetsSO,
    pub _presets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset,
            >,
        >,
    >,
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BloomPrePassGraphicsSettingsPresetsSO";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO {
    type Target = crate::GlobalNamespace::NamedPresetsSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO")]
impl crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO {
    #[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
    pub type Preset = crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset;
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
                    crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset,
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
                    crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset,
                >,
            >,
        > = __cordl_object.invoke("get_presets", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassGraphicsSettingsPresetsSO_Preset {
    __cordl_parent: crate::GlobalNamespace::NamedPreset,
    pub bloomPrePassEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassEffectSO,
    >,
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Preset";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset {
    type Target = crate::GlobalNamespace::NamedPreset;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
impl crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset {
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
#[cfg(feature = "BloomPrePassGraphicsSettingsPresetsSO+Preset")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassGraphicsSettingsPresetsSO_Preset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
