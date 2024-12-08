#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
#[repr(C)]
#[derive(Debug)]
pub struct EditAvatarAnalytics {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _avatarEditorFlowCoordinator: *mut crate::GlobalNamespace::BeatAvatarEditorFlowCoordinator,
    pub _analyticsModel: *mut crate::GlobalNamespace::IAnalyticsModel,
    pub _avatarDataModel: *mut crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
    pub _lastEditMode: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarAnalytics =>
    "BeatSaber.BeatAvatarAdapter.AvatarEditor"."EditAvatarAnalytics"
);
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarAnalytics {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarAnalytics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarAnalytics {
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
    pub fn CreateEditAvatarEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("CreateEditAvatarEventData", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleAvatarEditorFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        avatarSystemMetadata: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        finishAction: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAvatarEditorFlowCoordinatorDidFinish",
                (flowCoordinator, avatarSystemMetadata, finishAction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleAvatarEditorFlowCoordinatorDidSetup(
        &mut self,
        editMode: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAvatarEditorFlowCoordinatorDidSetup", (editMode))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAvatarEditorFlowCoordinatorRandomizeAllButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAvatarEditorFlowCoordinatorRandomizeAllButtonWasPressed",
                (),
            )?;
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
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::AvatarEditor::EditAvatarAnalytics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
