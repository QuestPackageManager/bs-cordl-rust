#[cfg(feature = "PauseMenuManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct PauseMenuManager_InitData {
    __cordl_parent: crate::System::Object,
    pub backButtonText: *mut crate::System::String,
    pub beatmapKey: BeatmapKey,
    pub beatmapLevel: *mut BeatmapLevel,
    pub showRestartButton: bool,
    pub showLevelBar: bool,
}
#[cfg(feature = "PauseMenuManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PauseMenuManager_InitData => ""
    ."PauseMenuManager/InitData"
);
#[cfg(feature = "PauseMenuManager+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::PauseMenuManager_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PauseMenuManager+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::PauseMenuManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PauseMenuManager+InitData")]
impl crate::GlobalNamespace::PauseMenuManager_InitData {
    pub fn _ctor(
        &mut self,
        backButtonText: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        showRestartButton: bool,
        showLevelBar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    backButtonText,
                    beatmapKey,
                    beatmapLevel,
                    showRestartButton,
                    showLevelBar,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        backButtonText: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        showRestartButton: bool,
        showLevelBar: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    backButtonText,
                    beatmapKey,
                    beatmapLevel,
                    showRestartButton,
                    showLevelBar,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PauseMenuManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PauseMenuManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PauseMenuManager")]
#[repr(C)]
#[derive(Debug)]
pub struct PauseMenuManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _pauseAnimationController: *mut PauseAnimationController,
    pub _levelBar: *mut LevelBar,
    pub _continueButton: *mut crate::UnityEngine::UI::Button,
    pub _restartButton: *mut crate::UnityEngine::UI::Button,
    pub _backButton: *mut crate::UnityEngine::UI::Button,
    pub _backButtonText: *mut crate::TMPro::TextMeshProUGUI,
    pub _pauseContainerTransform: *mut crate::UnityEngine::Transform,
    pub _initData: *mut crate::GlobalNamespace::PauseMenuManager_InitData,
    pub _vrPlatformHelper: *mut IVRPlatformHelper,
    pub _environmentSpawnRotation: *mut EnvironmentSpawnRotation,
    pub didPressContinueButtonEvent: *mut crate::System::Action,
    pub didPressMenuButtonEvent: *mut crate::System::Action,
    pub didPressRestartButtonEvent: *mut crate::System::Action,
    pub didFinishResumeAnimationEvent: *mut crate::System::Action,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
    pub _disabledInteractionRemainingTime: f32,
}
#[cfg(feature = "PauseMenuManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PauseMenuManager => ""."PauseMenuManager"
);
#[cfg(feature = "PauseMenuManager")]
impl std::ops::Deref for PauseMenuManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PauseMenuManager")]
impl std::ops::DerefMut for PauseMenuManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PauseMenuManager")]
impl PauseMenuManager {
    pub const kDisabledInteractionDuration: f32 = 0.2f32;
    #[cfg(feature = "PauseMenuManager+InitData")]
    pub type InitData = crate::GlobalNamespace::PauseMenuManager_InitData;
    pub fn remove_didPressContinueButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MenuButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MenuButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didFinishResumeAnimationEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishResumeAnimationEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn RestartButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressRestartButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressRestartButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ContinueButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ContinueButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressMenuButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressMenuButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishResumeAnimationEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishResumeAnimationEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ShowMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowMenu", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressContinueButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressRestartButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressRestartButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleResumeFromPauseAnimationDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleResumeFromPauseAnimationDidFinish", ())?;
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
    pub fn StartResumeAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartResumeAnimation", ())?;
        Ok(__cordl_ret)
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
    pub fn add_didPressMenuButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressMenuButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PauseMenuManager")]
impl quest_hook::libil2cpp::ObjectType for PauseMenuManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
