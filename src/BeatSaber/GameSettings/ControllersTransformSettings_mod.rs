#[cfg(feature = "BeatSaber+GameSettings+ControllersTransformSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllersTransformSettings {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _isLeft: bool,
    pub _posXSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _posYSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _posZSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _rotXSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _rotYSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _rotZSlider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _selectedControllerProfile: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::GameSettings::ControllerProfile,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllersTransformSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::GameSettings::ControllersTransformSettings => "BeatSaber.GameSettings"
    ."ControllersTransformSettings"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllersTransformSettings")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllersTransformSettings {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllersTransformSettings")]
impl std::ops::DerefMut
for crate::BeatSaber::GameSettings::ControllersTransformSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllersTransformSettings")]
impl crate::BeatSaber::GameSettings::ControllersTransformSettings {
    pub const kPositionMul: f32 = 100f32;
    pub const kPositionStep: f32 = 0.1f32;
    pub const kRotationStep: f32 = 1f32;
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
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
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshView(
        &mut self,
        controllerProfile: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshView", (controllerProfile))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInteractable(
        &mut self,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInteractable", (interactable))?;
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
    pub fn get_selectedControllerProfile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = __cordl_object.invoke("get_selectedControllerProfile", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllersTransformSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllersTransformSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
