#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EditAvatarFlowCoordinatorHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _avatarSystemSelectionFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _avatarSystemCollection: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
            crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
        >,
    >,
    pub _parentFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
    pub _singleAvatarEditorFlowCoordinator: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
    >,
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EditAvatarFlowCoordinatorHelper
    => ""."EditAvatarFlowCoordinatorHelper"
);
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl std::ops::Deref for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper {
    #[cfg(feature = "EditAvatarFlowCoordinatorHelper+FinishAction")]
    pub type FinishAction = crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction;
    pub fn HandleAvatarEditorFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        >,
        avatarSystem: quest_hook::libil2cpp::Gc<
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
                (flowCoordinator, avatarSystem, finishAction),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAvatarSystemSelectionFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator,
        >,
        finishAction: crate::GlobalNamespace::AvatarSystemSelectionFlowCoordinator_FinishAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAvatarSystemSelectionFlowCoordinatorDidFinish",
                (flowCoordinator, finishAction),
            )?;
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
    pub fn PresentAvatarEditorFlowCoordinator(
        &mut self,
        flowCoordinator: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        >,
        editMode: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
        parentFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        immediately: bool,
        replaceTopViewController: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PresentAvatarEditorFlowCoordinator",
                (
                    flowCoordinator,
                    editMode,
                    parentFlowCoordinator,
                    immediately,
                    replaceTopViewController,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Show(
        &mut self,
        parentFlowCoordinator: quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
        backButtonVisible: bool,
        immediately: bool,
        replaceTopViewController: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Show",
                (
                    parentFlowCoordinator,
                    backButtonVisible,
                    immediately,
                    replaceTopViewController,
                ),
            )?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get__hasOnlyOneAvatarSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get__hasOnlyOneAvatarSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::HMUI::FlowCoordinator>,
                crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper+FinishAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditAvatarFlowCoordinatorHelper_FinishAction {
    #[default]
    Back = 1i32,
    Continue = 0i32,
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper+FinishAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction => ""
    ."EditAvatarFlowCoordinatorHelper/FinishAction"
);
