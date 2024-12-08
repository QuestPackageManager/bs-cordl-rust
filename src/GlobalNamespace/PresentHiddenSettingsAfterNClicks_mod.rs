#[cfg(feature = "PresentHiddenSettingsAfterNClicks")]
#[repr(C)]
#[derive(Debug)]
pub struct PresentHiddenSettingsAfterNClicks {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _hiddenSettingsButton: *mut crate::UnityEngine::UI::Button,
    pub _hiddenSettingsViewController: *mut crate::HMUI::ViewController,
    pub _numberOfClicksRequired: i32,
    pub _settingsFlowCoordinator: *mut crate::GlobalNamespace::SettingsFlowCoordinator,
    pub _currentNumberOfClicks: i32,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
}
#[cfg(feature = "PresentHiddenSettingsAfterNClicks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PresentHiddenSettingsAfterNClicks => ""
    ."PresentHiddenSettingsAfterNClicks"
);
#[cfg(feature = "PresentHiddenSettingsAfterNClicks")]
impl std::ops::Deref for crate::GlobalNamespace::PresentHiddenSettingsAfterNClicks {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PresentHiddenSettingsAfterNClicks")]
impl std::ops::DerefMut for crate::GlobalNamespace::PresentHiddenSettingsAfterNClicks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PresentHiddenSettingsAfterNClicks")]
impl crate::GlobalNamespace::PresentHiddenSettingsAfterNClicks {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__6_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__6_0", ())?;
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
#[cfg(feature = "PresentHiddenSettingsAfterNClicks")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PresentHiddenSettingsAfterNClicks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
