#[cfg(feature = "StandaloneGraphicSettingsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct StandaloneGraphicSettingsViewController {
    __cordl_parent: crate::GlobalNamespace::GraphicSettingsViewController,
    pub _antiAliasingLevel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedIntListSettingsController,
    >,
    pub _vrRenderingScale: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _fullscreen: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _shockwaveMaxParticles: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NamedIntListSettingsController,
    >,
    pub _smoke: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _screenDisplacementEffects: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _mainEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PresetsSettingsController,
    >,
    pub _mirror: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PresetsSettingsController,
    >,
}
#[cfg(feature = "StandaloneGraphicSettingsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandaloneGraphicSettingsViewController => ""
    ."StandaloneGraphicSettingsViewController"
);
#[cfg(feature = "StandaloneGraphicSettingsViewController")]
impl std::ops::Deref
for crate::GlobalNamespace::StandaloneGraphicSettingsViewController {
    type Target = crate::GlobalNamespace::GraphicSettingsViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandaloneGraphicSettingsViewController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandaloneGraphicSettingsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandaloneGraphicSettingsViewController")]
impl crate::GlobalNamespace::StandaloneGraphicSettingsViewController {
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAntiAliasingLevelChanged(
        &mut self,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAntiAliasingLevelChanged", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleFullscreenToggled(
        &mut self,
        newValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleFullscreenToggled", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMainEffectChanged(
        &mut self,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMainEffectChanged", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMirrorChanged(
        &mut self,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMirrorChanged", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleScreenDisplacementToggled(
        &mut self,
        newValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScreenDisplacementToggled", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleShockwaveMaxParticlesChanged(
        &mut self,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleShockwaveMaxParticlesChanged", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSmokeToggled(
        &mut self,
        newValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSmokeToggled", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleVrRenderingScaleChanged(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
        newValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVrRenderingScaleChanged", (_cordl__, newValue))?;
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
#[cfg(feature = "StandaloneGraphicSettingsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandaloneGraphicSettingsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
