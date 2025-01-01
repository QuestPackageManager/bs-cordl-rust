#[cfg(feature = "SelectableStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectableStateController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub stateDidChangeEvent: *mut crate::System::Action_2<
        crate::GlobalNamespace::SelectableStateController_ViewState,
        bool,
    >,
    pub _currentViewState_k__BackingField: crate::GlobalNamespace::SelectableStateController_ViewState,
}
#[cfg(feature = "SelectableStateController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SelectableStateController => ""
    ."SelectableStateController"
);
#[cfg(feature = "SelectableStateController")]
impl std::ops::Deref for crate::GlobalNamespace::SelectableStateController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectableStateController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectableStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectableStateController")]
impl crate::GlobalNamespace::SelectableStateController {
    #[cfg(feature = "SelectableStateController+ViewState")]
    pub type ViewState = crate::GlobalNamespace::SelectableStateController_ViewState;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetState(
        &mut self,
        state: crate::GlobalNamespace::SelectableStateController_ViewState,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetState", (state, animated))?;
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
    pub fn add_stateDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::GlobalNamespace::SelectableStateController_ViewState,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_stateDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentViewState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SelectableStateController_ViewState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SelectableStateController_ViewState = __cordl_object
            .invoke("get_currentViewState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tweeningManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Tweening::TimeTweeningManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Tweening::TimeTweeningManager,
        > = __cordl_object.invoke("get_tweeningManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_stateDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::GlobalNamespace::SelectableStateController_ViewState,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_stateDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentViewState(
        &mut self,
        value: crate::GlobalNamespace::SelectableStateController_ViewState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentViewState", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SelectableStateController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectableStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SelectableStateController+ViewState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectableStateController_ViewState {
    Disabled = 3i32,
    Highlighted = 1i32,
    Normal = 0i32,
    Pressed = 2i32,
    Selected = 4i32,
    SelectedAndHighlighted = 5i32,
}
#[cfg(feature = "SelectableStateController+ViewState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectableStateController_ViewState => ""
    ."SelectableStateController/ViewState"
);
