#[cfg(feature = "HMUI+ToggleWithCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct ToggleWithCallbacks {
    __cordl_parent: crate::UnityEngine::UI::Toggle,
    pub stateDidChangeEvent: *mut crate::System::Action_1<
        crate::HMUI::ToggleWithCallbacks_SelectionState,
    >,
}
#[cfg(feature = "HMUI+ToggleWithCallbacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ToggleWithCallbacks => "HMUI"
    ."ToggleWithCallbacks"
);
#[cfg(feature = "HMUI+ToggleWithCallbacks")]
impl std::ops::Deref for crate::HMUI::ToggleWithCallbacks {
    type Target = crate::UnityEngine::UI::Toggle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ToggleWithCallbacks")]
impl std::ops::DerefMut for crate::HMUI::ToggleWithCallbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ToggleWithCallbacks")]
impl crate::HMUI::ToggleWithCallbacks {
    #[cfg(feature = "HMUI+ToggleWithCallbacks+SelectionState")]
    pub type SelectionState = crate::HMUI::ToggleWithCallbacks_SelectionState;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn add_stateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::HMUI::ToggleWithCallbacks_SelectionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_stateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::ToggleWithCallbacks_SelectionState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::ToggleWithCallbacks_SelectionState = __cordl_object
            .invoke("get_selectionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_stateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::HMUI::ToggleWithCallbacks_SelectionState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_stateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+ToggleWithCallbacks")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ToggleWithCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ToggleWithCallbacks+SelectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleWithCallbacks_SelectionState {
    Disabled = 4i32,
    Highlighted = 1i32,
    Normal = 0i32,
    Pressed = 2i32,
    Selected = 3i32,
}
#[cfg(feature = "HMUI+ToggleWithCallbacks+SelectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ToggleWithCallbacks_SelectionState =>
    "HMUI"."ToggleWithCallbacks/SelectionState"
);
