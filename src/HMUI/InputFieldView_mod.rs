#[cfg(feature = "HMUI+InputFieldView")]
#[repr(C)]
#[derive(Debug)]
pub struct InputFieldView {
    __cordl_parent: crate::UnityEngine::UI::Selectable,
    pub _textView: *mut crate::TMPro::TextMeshProUGUI,
    pub _textViewCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _blinkingCaret: *mut crate::HMUI::ImageView,
    pub _placeholderText: *mut crate::UnityEngine::GameObject,
    pub _clearSearchButton: *mut crate::UnityEngine::UI::Button,
    pub _useGlobalKeyboard: bool,
    pub _keyboardPositionOffset: crate::UnityEngine::Vector3,
    pub _useUppercase: bool,
    pub _textLengthLimit: i32,
    pub _caretOffset: f32,
    pub selectionStateDidChangeEvent: *mut crate::System::Action_1<
        crate::HMUI::InputFieldView_SelectionState,
    >,
    pub _selectionState: crate::HMUI::InputFieldView_SelectionState,
    pub _text: *mut crate::System::String,
    pub _hasKeyboardAssigned: bool,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
    pub _onValueChanged: *mut crate::HMUI::InputFieldView_InputFieldChanged,
    pub _blinkWaitYieldInstruction: *mut crate::UnityEngine::YieldInstruction,
}
#[cfg(feature = "HMUI+InputFieldView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::InputFieldView => "HMUI"."InputFieldView"
);
#[cfg(feature = "HMUI+InputFieldView")]
impl std::ops::Deref for crate::HMUI::InputFieldView {
    type Target = crate::UnityEngine::UI::Selectable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InputFieldView")]
impl std::ops::DerefMut for crate::HMUI::InputFieldView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InputFieldView")]
impl crate::HMUI::InputFieldView {
    pub const kBlinkingRate: f32 = 0.4f32;
    #[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
    pub type InputFieldChanged = crate::HMUI::InputFieldView_InputFieldChanged;
    #[cfg(feature = "HMUI+InputFieldView+SelectionState")]
    pub type SelectionState = crate::HMUI::InputFieldView_SelectionState;
    #[cfg(feature = "HMUI+InputFieldView+_BlinkingCaretCoroutine_d__43")]
    pub type _BlinkingCaretCoroutine_d__43 = crate::HMUI::InputFieldView__BlinkingCaretCoroutine_d__43;
    pub fn ActivateKeyboard(
        &mut self,
        keyboard: *mut crate::HMUI::UIKeyboard,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateKeyboard", (keyboard))?;
        Ok(__cordl_ret)
    }
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
    pub fn BlinkingCaretCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("BlinkingCaretCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeactivateKeyboard(
        &mut self,
        keyboard: *mut crate::HMUI::UIKeyboard,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateKeyboard", (keyboard))?;
        Ok(__cordl_ret)
    }
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
    pub fn KeyboardDeletePressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KeyboardDeletePressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn KeyboardKeyPressed(
        &mut self,
        letter: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("KeyboardKeyPressed", (letter))?;
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
    pub fn SetText(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCaretPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCaretPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateClearButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateClearButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePlaceholder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePlaceholder", ())?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__34_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__34_0", ())?;
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
    pub fn add_selectionStateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::HMUI::InputFieldView_SelectionState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectionStateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_keyboardPositionOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_keyboardPositionOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onValueChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HMUI::InputFieldView_InputFieldChanged,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::InputFieldView_InputFieldChanged = __cordl_object
            .invoke("get_onValueChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::InputFieldView_SelectionState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::InputFieldView_SelectionState = __cordl_object
            .invoke("get_selectionState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_text", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useGlobalKeyboard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useGlobalKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectionStateDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::HMUI::InputFieldView_SelectionState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectionStateDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onValueChanged(
        &mut self,
        value: *mut crate::HMUI::InputFieldView_InputFieldChanged,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValueChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+InputFieldView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::InputFieldView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
#[repr(C)]
#[derive(Debug)]
pub struct InputFieldView_InputFieldChanged {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::HMUI::InputFieldView,
    >,
}
#[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::InputFieldView_InputFieldChanged => "HMUI"
    ."InputFieldView/InputFieldChanged"
);
#[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
impl std::ops::Deref for crate::HMUI::InputFieldView_InputFieldChanged {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::HMUI::InputFieldView,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
impl std::ops::DerefMut for crate::HMUI::InputFieldView_InputFieldChanged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
impl crate::HMUI::InputFieldView_InputFieldChanged {
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
}
#[cfg(feature = "HMUI+InputFieldView+InputFieldChanged")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::InputFieldView_InputFieldChanged {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+InputFieldView+SelectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFieldView_SelectionState {
    Disabled = 3i32,
    Highlighted = 1i32,
    Normal = 0i32,
    Pressed = 2i32,
    Selected = 4i32,
}
#[cfg(feature = "HMUI+InputFieldView+SelectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::InputFieldView_SelectionState => "HMUI"
    ."InputFieldView/SelectionState"
);
