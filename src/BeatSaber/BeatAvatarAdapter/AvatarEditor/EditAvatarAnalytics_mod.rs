#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarEditor+EditAvatarAnalytics")]
#[repr(C)]
#[derive(Debug)]
pub struct EditAvatarAnalytics {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _avatarEditorFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatAvatarEditorFlowCoordinator,
    >,
    pub _analyticsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAnalyticsModel,
    >,
    pub _avatarDataModel: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
    >,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateEditAvatarEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("CreateEditAvatarEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAvatarEditorFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        >,
        avatarSystemMetadata: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
