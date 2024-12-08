#[cfg(feature = "ControllersTransformSettingsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllersTransformSettingsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _posXSlider: *mut crate::HMUI::RangeValuesTextSlider,
    pub _posYSlider: *mut crate::HMUI::RangeValuesTextSlider,
    pub _posZSlider: *mut crate::HMUI::RangeValuesTextSlider,
    pub _rotXSlider: *mut crate::HMUI::RangeValuesTextSlider,
    pub _rotYSlider: *mut crate::HMUI::RangeValuesTextSlider,
    pub _rotZSlider: *mut crate::HMUI::RangeValuesTextSlider,
    pub _vrPlatformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
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
        Ok(__cordl_ret)
    }
    pub fn HandlePositionSliderValueDidChange(
        &mut self,
        slider: *mut crate::HMUI::RangeValuesTextSlider,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePositionSliderValueDidChange", (slider, value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleRotationSliderValueDidChange(
        &mut self,
        slider: *mut crate::HMUI::RangeValuesTextSlider,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRotationSliderValueDidChange", (slider, value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
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
