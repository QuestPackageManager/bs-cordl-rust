#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct EditAvatarFlowCoordinatorHelper {
    __cordl_parent: crate::System::Object,
    pub _avatarSystemSelectionFlowCoordinator: *mut AvatarSystemSelectionFlowCoordinator,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _container: *mut crate::Zenject::DiContainer,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut crate::HMUI::FlowCoordinator,
        crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
    >,
    pub _parentFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
    pub _singleAvatarEditorFlowCoordinator: *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EditAvatarFlowCoordinatorHelper => ""
    ."EditAvatarFlowCoordinatorHelper"
);
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl std::ops::Deref for EditAvatarFlowCoordinatorHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl std::ops::DerefMut for EditAvatarFlowCoordinatorHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl EditAvatarFlowCoordinatorHelper {
    #[cfg(feature = "EditAvatarFlowCoordinatorHelper+_Initialize_d__13")]
    pub type _Initialize_d__13 = crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper__Initialize_d__13;
    #[cfg(feature = "EditAvatarFlowCoordinatorHelper+_Show_d__12")]
    pub type _Show_d__12 = crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper__Show_d__12;
    #[cfg(feature = "EditAvatarFlowCoordinatorHelper+FinishAction")]
    pub type FinishAction = crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction;
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::HMUI::FlowCoordinator,
            crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAvatarSystemSelectionFlowCoordinatorDidFinish(
        &mut self,
        flowCoordinator: *mut AvatarSystemSelectionFlowCoordinator,
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
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::HMUI::FlowCoordinator,
            crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Show(
        &mut self,
        parentFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
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
    pub fn get__hasOnlyOneAvatarSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get__hasOnlyOneAvatarSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn PresentAvatarEditorFlowCoordinator(
        &mut self,
        flowCoordinator: *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        editMode: crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator_EditMode,
        parentFlowCoordinator: *mut crate::HMUI::FlowCoordinator,
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
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
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
#[cfg(feature = "EditAvatarFlowCoordinatorHelper")]
impl quest_hook::libil2cpp::ObjectType for EditAvatarFlowCoordinatorHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper+FinishAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditAvatarFlowCoordinatorHelper_FinishAction {
    Back = 1i32,
    Continue = 0i32,
}
#[cfg(feature = "EditAvatarFlowCoordinatorHelper+FinishAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction => ""
    ."EditAvatarFlowCoordinatorHelper/FinishAction"
);
