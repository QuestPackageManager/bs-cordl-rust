#[cfg(feature = "HMUI+NoTransitionsButton")]
#[repr(C)]
#[derive(Debug)]
pub struct NoTransitionsButton {
    __cordl_parent: crate::UnityEngine::UI::Button,
    pub selectionStateDidChangeEvent: *mut crate::System::Action_1<
        crate::HMUI::NoTransitionsButton_SelectionState,
    >,
    pub _selectionState: crate::HMUI::NoTransitionsButton_SelectionState,
}
#[cfg(feature = "HMUI+NoTransitionsButton")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::NoTransitionsButton => "HMUI"
    ."NoTransitionsButton"
);
#[cfg(feature = "HMUI+NoTransitionsButton")]
impl std::ops::Deref for crate::HMUI::NoTransitionsButton {
    type Target = crate::UnityEngine::UI::Button;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+NoTransitionsButton")]
impl std::ops::DerefMut for crate::HMUI::NoTransitionsButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+NoTransitionsButton")]
impl crate::HMUI::NoTransitionsButton {
    #[cfg(feature = "HMUI+NoTransitionsButton+SelectionState")]
    pub type SelectionState = crate::HMUI::NoTransitionsButton_SelectionState;
    pub fn DoStateTransition(
        &mut self,
        state: crate::UnityEngine::UI::Selectable_SelectionState,
        instant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoStateTransition", (state, instant))?;
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
    pub fn remove_selectionStateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::HMUI::NoTransitionsButton_SelectionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectionStateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::NoTransitionsButton_SelectionState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::NoTransitionsButton_SelectionState = __cordl_object
            .invoke("get_selectionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_selectionStateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::HMUI::NoTransitionsButton_SelectionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectionStateDidChangeEvent", (value))?;
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
#[cfg(feature = "HMUI+NoTransitionsButton")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::NoTransitionsButton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+NoTransitionsButton+SelectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoTransitionsButton_SelectionState {
    Disabled = 3i32,
    Highlighted = 1i32,
    Normal = 0i32,
    Pressed = 2i32,
}
#[cfg(feature = "HMUI+NoTransitionsButton+SelectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::NoTransitionsButton_SelectionState =>
    "HMUI"."NoTransitionsButton/SelectionState"
);
