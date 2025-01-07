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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EditAvatarFlowCoordinatorHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FinishAction";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::EditAvatarFlowCoordinatorHelper_FinishAction {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
