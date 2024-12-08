#[cfg(feature = "AvatarSystemSelectionFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSystemSelectionFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub _selectAvatarSystemViewController: *mut SelectAvatarSystemViewController,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _container: *mut crate::Zenject::DiContainer,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut AvatarSystemSelectionFlowCoordinator,
        crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator_FinishAction,
    >,
    pub _avatarFlowCoordinators: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
        *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
    >,
    pub _continueButtonVisible: bool,
}
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AvatarSystemSelectionFlowCoordinator => ""
    ."AvatarSystemSelectionFlowCoordinator"
);
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator")]
impl std::ops::Deref for AvatarSystemSelectionFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator")]
impl std::ops::DerefMut for AvatarSystemSelectionFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator")]
impl AvatarSystemSelectionFlowCoordinator {
    #[cfg(feature = "AvatarSystemSelectionFlowCoordinator+FinishAction")]
    pub type FinishAction = crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator_FinishAction;
    #[cfg(feature = "AvatarSystemSelectionFlowCoordinator+_Initialize_d__10")]
    pub type _Initialize_d__10 = crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator__Initialize_d__10;
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret)
    }
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
        Ok(__cordl_ret)
    }
    pub fn HandleAvatarEditorFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        finishAction: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAvatarEditorFlowCoordinatorDidFinish",
                (flowCoordinator, avatarSystem, finishAction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectAvatarSystemViewControllerDidPressContinueButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectAvatarSystemViewControllerDidPressContinueButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectAvatarSystemViewControllerDidRequestCreationOfAvatar(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSelectAvatarSystemViewControllerDidRequestCreationOfAvatar",
                (avatarSystem),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectAvatarSystemViewControllerDidRequestEditOfAvatar(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSelectAvatarSystemViewControllerDidRequestEditOfAvatar",
                (avatarSystem),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectAvatarSystemViewControllerDidSetAvatarSystemPreferred(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSelectAvatarSystemViewControllerDidSetAvatarSystemPreferred",
                (avatarSystem),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        avatarSystemsMetadata: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (avatarSystemsMetadata))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetAvatarSystemPreferred(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAvatarSystemPreferred", (avatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        continueButtonVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (continueButtonVisible))?;
        Ok(__cordl_ret)
    }
    pub fn TopViewControllerWillChange(
        &mut self,
        oldViewController: *mut crate::HMUI::ViewController,
        newViewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _Initialize_b__10_0(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<Initialize>b__10_0", (avatarSystem))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut AvatarSystemSelectionFlowCoordinator,
            crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut AvatarSystemSelectionFlowCoordinator,
            crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for AvatarSystemSelectionFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator+FinishAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSystemSelectionFlowCoordinator_FinishAction {
    Back = 1i32,
    Continue = 0i32,
}
#[cfg(feature = "AvatarSystemSelectionFlowCoordinator+FinishAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AvatarSystemSelectionFlowCoordinator_FinishAction => ""
    ."AvatarSystemSelectionFlowCoordinator/FinishAction"
);
