#[cfg(feature = "BeatAvatarEditorFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarEditorFlowCoordinator {
    __cordl_parent: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
    pub _avatarTweenController: *mut crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::AvatarTweenController,
    pub _avatarContainerGameObject: *mut crate::UnityEngine::GameObject,
    pub _avatarVisualController: *mut crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController,
    pub _beatAvatarEditorViewController: *mut crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::BeatAvatarEditorViewController,
    pub _editAvatarColorViewController: *mut crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarColorViewController,
    pub _avatarDataModel: *mut crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
    pub _parameterChangedAnimationCallbacks: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::BeatSaber::BeatAvatarSDK::AvatarPart,
        *mut crate::System::Action,
    >,
    pub _coloredAvatarPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    pub _originalColorOfColoredPart: crate::UnityEngine::Color,
}
#[cfg(feature = "BeatAvatarEditorFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatAvatarEditorFlowCoordinator
    => ""."BeatAvatarEditorFlowCoordinator"
);
#[cfg(feature = "BeatAvatarEditorFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::BeatAvatarEditorFlowCoordinator {
    type Target = crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatAvatarEditorFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatAvatarEditorFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatAvatarEditorFlowCoordinator")]
impl crate::GlobalNamespace::BeatAvatarEditorFlowCoordinator {
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
    pub fn HandleBeatAvatarEditorViewCancelButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatAvatarEditorViewCancelButtonWasPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatAvatarEditorViewOkButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatAvatarEditorViewOkButtonWasPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatAvatarViewControllerChangedAvatarPart(
        &mut self,
        avatarPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatAvatarViewControllerChangedAvatarPart", (avatarPart))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatAvatarViewControllerDidRequestColorChange(
        &mut self,
        colorCallback: *mut crate::System::Action_1<crate::UnityEngine::Color>,
        currentColor: crate::UnityEngine::Color,
        editPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
        uvSegment: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatAvatarViewControllerDidRequestColorChange",
                (colorCallback, currentColor, editPart, uvSegment),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleEditColorViewControllerControllerDidFinish(
        &mut self,
        apply: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEditColorViewControllerControllerDidFinish", (apply))?;
        Ok(__cordl_ret)
    }
    pub fn HandleEditColorViewControllerDidChangedColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleEditColorViewControllerDidChangedColor", (color))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OneTimeInitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OneTimeInitialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        editMode: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (editMode))?;
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
#[cfg(feature = "BeatAvatarEditorFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatAvatarEditorFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
