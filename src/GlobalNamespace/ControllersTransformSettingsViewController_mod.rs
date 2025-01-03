#[cfg(feature = "ControllersTransformSettingsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllersTransformSettingsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _posXSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _posYSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _posZSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _rotXSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _rotYSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _rotZSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
}
#[cfg(feature = "ControllersTransformSettingsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ControllersTransformSettingsViewController => ""
    ."ControllersTransformSettingsViewController"
);
#[cfg(feature = "ControllersTransformSettingsViewController")]
impl std::ops::Deref
for crate::GlobalNamespace::ControllersTransformSettingsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ControllersTransformSettingsViewController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ControllersTransformSettingsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ControllersTransformSettingsViewController")]
impl crate::GlobalNamespace::ControllersTransformSettingsViewController {
    pub const kPositionMul: f32 = 100f32;
    pub const kPositionStep: f32 = 0.1f32;
    pub const kRotationStep: f32 = 1f32;
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
    pub fn HandlePositionSliderValueDidChange(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePositionSliderValueDidChange", (slider, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleRotationSliderValueDidChange(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRotationSliderValueDidChange", (slider, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
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
}
#[cfg(feature = "ControllersTransformSettingsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ControllersTransformSettingsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
