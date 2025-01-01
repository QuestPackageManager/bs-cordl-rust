#[cfg(feature = "RenderingParamsSetupApplicator")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderingParamsSetupApplicator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _settingsManager: *mut crate::GlobalNamespace::SettingsManager,
    pub _settingsApplicator: *mut crate::GlobalNamespace::SettingsApplicatorSO,
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RenderingParamsSetupApplicator
    => ""."RenderingParamsSetupApplicator"
);
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl std::ops::Deref for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl std::ops::DerefMut for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl crate::GlobalNamespace::RenderingParamsSetupApplicator {
    pub fn Apply(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (sceneType, optionalEnvironmentSerializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyGraphicsSettings(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ApplyGraphicsSettings",
                (sceneType, optionalEnvironmentSerializedName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyMainSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyMainSettings", ())?;
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
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl AsRef<crate::GlobalNamespace::IRenderingParamsApplicator>
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRenderingParamsApplicator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl AsMut<crate::GlobalNamespace::IRenderingParamsApplicator>
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRenderingParamsApplicator {
        unsafe { std::mem::transmute(self) }
    }
}
