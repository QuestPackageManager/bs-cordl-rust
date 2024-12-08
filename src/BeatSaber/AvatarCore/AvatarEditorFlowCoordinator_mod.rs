#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarEditorFlowCoordinator {
    __cordl_parent: crate::HMUI::FlowCoordinator,
    pub didFinishEvent: *mut crate::System::Action_3<
        *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction,
    >,
    pub didSetupEvent: *mut crate::System::Action_1<
        crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
    >,
    pub randomizeAllButtonWasPressedEvent: *mut crate::System::Action,
    pub _selectedAvatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    pub _initialized: bool,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator => "BeatSaber.AvatarCore"
    ."AvatarEditorFlowCoordinator"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator {
    type Target = crate::HMUI::FlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator")]
impl crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator {
    #[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator+FinishAction")]
    pub type FinishAction = crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction;
    #[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator+EditMode")]
    pub type EditMode = crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode;
    pub fn Finish(
        &mut self,
        finishAction: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", (finishAction))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatAvatarEditorRandomizeAllButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeatAvatarEditorRandomizeAllButtonWasPressed", ())?;
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
    pub fn SetAvatarSystem(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAvatarSystem", (avatarSystem))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSetupEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSetupEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_randomizeAllButtonWasPressedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_randomizeAllButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSetupEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSetupEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_randomizeAllButtonWasPressedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_randomizeAllButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator+EditMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarEditorFlowCoordinator_EditMode {
    Create = 0i32,
    Edit = 1i32,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator+EditMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode =>
    "BeatSaber.AvatarCore"."AvatarEditorFlowCoordinator/EditMode"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator+FinishAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarEditorFlowCoordinator_FinishAction {
    Apply = 1i32,
    Cancel = 0i32,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarEditorFlowCoordinator+FinishAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction =>
    "BeatSaber.AvatarCore"."AvatarEditorFlowCoordinator/FinishAction"
);
