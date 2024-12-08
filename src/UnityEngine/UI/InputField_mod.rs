#[cfg(feature = "UnityEngine+UI+InputField+CharacterValidation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField_CharacterValidation {
    Alphanumeric = 3i32,
    Decimal = 2i32,
    EmailAddress = 5i32,
    Integer = 1i32,
    Name = 4i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+UI+InputField+CharacterValidation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_CharacterValidation
    => "UnityEngine.UI"."InputField/CharacterValidation"
);
#[cfg(feature = "UnityEngine+UI+InputField+ContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField_ContentType {
    Alphanumeric = 4i32,
    Autocorrected = 1i32,
    Custom = 9i32,
    DecimalNumber = 3i32,
    EmailAddress = 6i32,
    IntegerNumber = 2i32,
    Name = 5i32,
    Password = 7i32,
    Pin = 8i32,
    Standard = 0i32,
}
#[cfg(feature = "UnityEngine+UI+InputField+ContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_ContentType =>
    "UnityEngine.UI"."InputField/ContentType"
);
#[cfg(feature = "UnityEngine+UI+InputField+EditState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField_EditState {
    Continue = 0i32,
    Finish = 1i32,
}
#[cfg(feature = "UnityEngine+UI+InputField+EditState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_EditState =>
    "UnityEngine.UI"."InputField/EditState"
);
#[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct InputField_EndEditEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>,
}
#[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_EndEditEvent =>
    "UnityEngine.UI"."InputField/EndEditEvent"
);
#[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::InputField_EndEditEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UI::InputField_EndEditEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
impl crate::UnityEngine::UI::InputField_EndEditEvent {
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
#[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::InputField_EndEditEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+InputField")]
#[repr(C)]
#[derive(Debug)]
pub struct InputField {
    __cordl_parent: crate::UnityEngine::UI::Selectable,
    pub m_Keyboard: *mut crate::UnityEngine::TouchScreenKeyboard,
    pub m_TextComponent: *mut crate::UnityEngine::UI::Text,
    pub m_Placeholder: *mut crate::UnityEngine::UI::Graphic,
    pub m_ContentType: crate::UnityEngine::UI::InputField_ContentType,
    pub m_InputType: crate::UnityEngine::UI::InputField_InputType,
    pub m_AsteriskChar: char,
    pub m_KeyboardType: crate::UnityEngine::TouchScreenKeyboardType,
    pub m_LineType: crate::UnityEngine::UI::InputField_LineType,
    pub m_HideMobileInput: bool,
    pub m_CharacterValidation: crate::UnityEngine::UI::InputField_CharacterValidation,
    pub m_CharacterLimit: i32,
    pub m_OnSubmit: *mut crate::UnityEngine::UI::InputField_SubmitEvent,
    pub m_OnDidEndEdit: *mut crate::UnityEngine::UI::InputField_EndEditEvent,
    pub m_OnValueChanged: *mut crate::UnityEngine::UI::InputField_OnChangeEvent,
    pub m_OnValidateInput: *mut crate::UnityEngine::UI::InputField_OnValidateInput,
    pub m_CaretColor: crate::UnityEngine::Color,
    pub m_CustomCaretColor: bool,
    pub m_SelectionColor: crate::UnityEngine::Color,
    pub m_Text: *mut crate::System::String,
    pub m_CaretBlinkRate: f32,
    pub m_CaretWidth: i32,
    pub m_ReadOnly: bool,
    pub m_ShouldActivateOnSelect: bool,
    pub m_CaretPosition: i32,
    pub m_CaretSelectPosition: i32,
    pub caretRectTrans: *mut crate::UnityEngine::RectTransform,
    pub m_CursorVerts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIVertex,
    >,
    pub m_InputTextCache: *mut crate::UnityEngine::TextGenerator,
    pub m_CachedInputRenderer: *mut crate::UnityEngine::CanvasRenderer,
    pub m_PreventFontCallback: bool,
    pub m_Mesh: *mut crate::UnityEngine::Mesh,
    pub m_AllowInput: bool,
    pub m_ShouldActivateNextUpdate: bool,
    pub m_UpdateDrag: bool,
    pub m_DragPositionOutOfBounds: bool,
    pub m_CaretVisible: bool,
    pub m_BlinkCoroutine: *mut crate::UnityEngine::Coroutine,
    pub m_BlinkStartTime: f32,
    pub m_DrawStart: i32,
    pub m_DrawEnd: i32,
    pub m_DragCoroutine: *mut crate::UnityEngine::Coroutine,
    pub m_OriginalText: *mut crate::System::String,
    pub m_WasCanceled: bool,
    pub m_HasDoneFocusTransition: bool,
    pub m_WaitForSecondsRealtime: *mut crate::UnityEngine::WaitForSecondsRealtime,
    pub m_TouchKeyboardAllowsInPlaceEditing: bool,
    pub m_IsCompositionActive: bool,
    pub m_ProcessingEvent: *mut crate::UnityEngine::Event,
}
#[cfg(feature = "UnityEngine+UI+InputField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField => "UnityEngine.UI"
    ."InputField"
);
#[cfg(feature = "UnityEngine+UI+InputField")]
impl std::ops::Deref for crate::UnityEngine::UI::InputField {
    type Target = crate::UnityEngine::UI::Selectable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField")]
impl std::ops::DerefMut for crate::UnityEngine::UI::InputField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField")]
impl crate::UnityEngine::UI::InputField {
    pub const kEmailSpecialCharacters: &'static str = "!#$%&\\'*+-/=?^_`{|}~";
    pub const kHScrollSpeed: f32 = 0.05f32;
    pub const kOculusQuestDeviceModel: &'static str = "Oculus Quest";
    pub const kVScrollSpeed: f32 = 0.1f32;
    pub const k_MaxTextLength: i32 = 16382i32;
    #[cfg(feature = "UnityEngine+UI+InputField+CharacterValidation")]
    pub type CharacterValidation = crate::UnityEngine::UI::InputField_CharacterValidation;
    #[cfg(feature = "UnityEngine+UI+InputField+ContentType")]
    pub type ContentType = crate::UnityEngine::UI::InputField_ContentType;
    #[cfg(feature = "UnityEngine+UI+InputField+EditState")]
    pub type EditState = crate::UnityEngine::UI::InputField_EditState;
    #[cfg(feature = "UnityEngine+UI+InputField+EndEditEvent")]
    pub type EndEditEvent = crate::UnityEngine::UI::InputField_EndEditEvent;
    #[cfg(feature = "UnityEngine+UI+InputField+InputType")]
    pub type InputType = crate::UnityEngine::UI::InputField_InputType;
    #[cfg(feature = "UnityEngine+UI+InputField+LineType")]
    pub type LineType = crate::UnityEngine::UI::InputField_LineType;
    #[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
    pub type OnChangeEvent = crate::UnityEngine::UI::InputField_OnChangeEvent;
    #[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
    pub type OnValidateInput = crate::UnityEngine::UI::InputField_OnValidateInput;
    #[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
    pub type SubmitEvent = crate::UnityEngine::UI::InputField_SubmitEvent;
    #[cfg(feature = "UnityEngine+UI+InputField+_CaretBlink_d__172")]
    pub type _CaretBlink_d__172 = crate::UnityEngine::UI::InputField__CaretBlink_d__172;
    #[cfg(feature = "UnityEngine+UI+InputField+_MouseDragOutsideRect_d__196")]
    pub type _MouseDragOutsideRect_d__196 = crate::UnityEngine::UI::InputField__MouseDragOutsideRect_d__196;
    pub fn ActivateInputField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateInputField", ())?;
        Ok(__cordl_ret)
    }
    pub fn ActivateInputFieldInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateInputFieldInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn Append_String0(
        &mut self,
        input: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Append", (input))?;
        Ok(__cordl_ret)
    }
    pub fn Append__cordl_char1(
        &mut self,
        input: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Append", (input))?;
        Ok(__cordl_ret)
    }
    pub fn AssignPositioningIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignPositioningIfNeeded", ())?;
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
    pub fn Backspace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Backspace", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateLayoutInputHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputHorizontal", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateLayoutInputVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateLayoutInputVertical", ())?;
        Ok(__cordl_ret)
    }
    pub fn CaretBlink(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("CaretBlink", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClampPos(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClampPos", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCursorVerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCursorVerts", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeactivateInputField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateInputField", ())?;
        Ok(__cordl_ret)
    }
    pub fn Delete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Delete", ())?;
        Ok(__cordl_ret)
    }
    pub fn DetermineCharacterLine(
        &mut self,
        charPos: i32,
        generator: *mut crate::UnityEngine::TextGenerator,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("DetermineCharacterLine", (charPos, generator))?;
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
    pub fn EnforceContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnforceContentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnforceTextHOverflow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnforceTextHOverflow", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindtNextWordBegin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindtNextWordBegin", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindtPrevWordBegin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindtPrevWordBegin", ())?;
        Ok(__cordl_ret)
    }
    pub fn ForceLabelUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceLabelUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ForwardSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForwardSpace", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCaret(
        &mut self,
        vbo: *mut crate::UnityEngine::UI::VertexHelper,
        roundingOffset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateCaret", (vbo, roundingOffset))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateHighlight(
        &mut self,
        vbo: *mut crate::UnityEngine::UI::VertexHelper,
        roundingOffset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateHighlight", (vbo, roundingOffset))?;
        Ok(__cordl_ret)
    }
    pub fn GetCharacterIndexFromPosition(
        &mut self,
        pos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharacterIndexFromPosition", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn GetInternalSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RangeInt> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RangeInt = __cordl_object
            .invoke("GetInternalSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetSelectedString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUnclampedCharacterLineFromPosition(
        &mut self,
        pos: crate::UnityEngine::Vector2,
        generator: *mut crate::UnityEngine::TextGenerator,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetUnclampedCharacterLineFromPosition", (pos, generator))?;
        Ok(__cordl_ret)
    }
    pub fn GraphicUpdateComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GraphicUpdateComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn InPlaceEditing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InPlaceEditing", ())?;
        Ok(__cordl_ret)
    }
    pub fn InPlaceEditingChanged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InPlaceEditingChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn Insert(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (c))?;
        Ok(__cordl_ret)
    }
    pub fn IsSelectionVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSelectionVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsValidChar(&mut self, c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidChar", (c))?;
        Ok(__cordl_ret)
    }
    pub fn KeyPressed(
        &mut self,
        evt: *mut crate::UnityEngine::Event,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::InputField_EditState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::InputField_EditState = __cordl_object
            .invoke("KeyPressed", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn LayoutComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LayoutComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn LineDownCharacterPosition(
        &mut self,
        originalPos: i32,
        goToLastChar: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LineDownCharacterPosition", (originalPos, goToLastChar))?;
        Ok(__cordl_ret)
    }
    pub fn LineUpCharacterPosition(
        &mut self,
        originalPos: i32,
        goToFirstChar: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LineUpCharacterPosition", (originalPos, goToFirstChar))?;
        Ok(__cordl_ret)
    }
    pub fn MarkGeometryAsDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkGeometryAsDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn MayDrag(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MayDrag", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn MouseDragOutsideRect(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("MouseDragOutsideRect", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn MoveDown__cordl_bool0(
        &mut self,
        shift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveDown", (shift))?;
        Ok(__cordl_ret)
    }
    pub fn MoveDown__cordl_bool1(
        &mut self,
        shift: bool,
        goToLastChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveDown", (shift, goToLastChar))?;
        Ok(__cordl_ret)
    }
    pub fn MoveLeft(
        &mut self,
        shift: bool,
        ctrl: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveLeft", (shift, ctrl))?;
        Ok(__cordl_ret)
    }
    pub fn MoveRight(
        &mut self,
        shift: bool,
        ctrl: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveRight", (shift, ctrl))?;
        Ok(__cordl_ret)
    }
    pub fn MoveTextEnd(
        &mut self,
        shift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveTextEnd", (shift))?;
        Ok(__cordl_ret)
    }
    pub fn MoveTextStart(
        &mut self,
        shift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveTextStart", (shift))?;
        Ok(__cordl_ret)
    }
    pub fn MoveUp__cordl_bool0(
        &mut self,
        shift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveUp", (shift))?;
        Ok(__cordl_ret)
    }
    pub fn MoveUp__cordl_bool1(
        &mut self,
        shift: bool,
        goToFirstChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveUp", (shift, goToFirstChar))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnBeginDrag(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBeginDrag", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnDeselect(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeselect", (eventData))?;
        Ok(__cordl_ret)
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
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDrag(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrag", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEndDrag(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEndDrag", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnFillVBO(
        &mut self,
        vbo: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFillVBO", (vbo))?;
        Ok(__cordl_ret)
    }
    pub fn OnFocus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocus", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerClick(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerClick", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnSelect(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSelect", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnSubmit(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSubmit", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnUpdateSelected(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdateSelected", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessEvent(
        &mut self,
        e: *mut crate::UnityEngine::Event,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEvent", (e))?;
        Ok(__cordl_ret)
    }
    pub fn Rebuild(
        &mut self,
        update: crate::UnityEngine::UI::CanvasUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Rebuild", (update))?;
        Ok(__cordl_ret)
    }
    pub fn ScreenToLocal(
        &mut self,
        screen: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ScreenToLocal", (screen))?;
        Ok(__cordl_ret)
    }
    pub fn SelectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendOnEndEdit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnEndEdit", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendOnSubmit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnSubmit", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendOnValueChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnValueChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendOnValueChangedAndUpdateLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnValueChangedAndUpdateLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCaretActive(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCaretActive", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCaretVisible(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCaretVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetDrawRangeToContainCaretPosition(
        &mut self,
        caretPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDrawRangeToContainCaretPosition", (caretPos))?;
        Ok(__cordl_ret)
    }
    pub fn SetText(
        &mut self,
        value: *mut crate::System::String,
        sendCallback: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetText", (value, sendCallback))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextWithoutNotify(
        &mut self,
        input: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextWithoutNotify", (input))?;
        Ok(__cordl_ret)
    }
    pub fn SetToCustom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToCustom", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetToCustomIfContentTypeIsNot(
        &mut self,
        allowedContentTypes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UI::InputField_ContentType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToCustomIfContentTypeIsNot", (allowedContentTypes))?;
        Ok(__cordl_ret)
    }
    pub fn TouchScreenKeyboardShouldBeUsed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TouchScreenKeyboardShouldBeUsed", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnityEngine_UI_ICanvasElement_get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("UnityEngine.UI.ICanvasElement.get_transform", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCaretFromKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCaretFromKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCaretMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCaretMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGeometry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGeometry", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateKeyboardCaret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateKeyboardCaret", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLabel", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTouchKeyboardFromEditChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTouchKeyboardFromEditChanges", ())?;
        Ok(__cordl_ret)
    }
    pub fn Validate(
        &mut self,
        text: *mut crate::System::String,
        pos: i32,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("Validate", (text, pos, ch))?;
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
    pub fn get_asteriskChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_asteriskChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cachedInputTextGenerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextGenerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextGenerator = __cordl_object
            .invoke("get_cachedInputTextGenerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretBlinkRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_caretBlinkRate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_caretColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_caretPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretPositionInternal(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_caretPositionInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretSelectPositionInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_caretSelectPositionInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_caretWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_characterLimit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::InputField_CharacterValidation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::InputField_CharacterValidation = __cordl_object
            .invoke("get_characterValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_compositionString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_compositionString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_contentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::InputField_ContentType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::InputField_ContentType = __cordl_object
            .invoke("get_contentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_customCaretColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_customCaretColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::BaseInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::BaseInput = __cordl_object
            .invoke("get_input", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inputType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::InputField_InputType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::InputField_InputType = __cordl_object
            .invoke("get_inputType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isFocused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFocused", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_keyboardType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TouchScreenKeyboardType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TouchScreenKeyboardType = __cordl_object
            .invoke("get_keyboardType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_layoutPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layoutPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UI::InputField_LineType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::InputField_LineType = __cordl_object
            .invoke("get_lineType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Mesh = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiLine(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_multiLine", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onEndEdit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::InputField_EndEditEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::InputField_EndEditEvent = __cordl_object
            .invoke("get_onEndEdit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onSubmit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::InputField_SubmitEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::InputField_SubmitEvent = __cordl_object
            .invoke("get_onSubmit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onValidateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::InputField_OnValidateInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::InputField_OnValidateInput = __cordl_object
            .invoke("get_onValidateInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onValueChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::InputField_OnChangeEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::InputField_OnChangeEvent = __cordl_object
            .invoke("get_onValueChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onValueChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UI::InputField_OnChangeEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::InputField_OnChangeEvent = __cordl_object
            .invoke("get_onValueChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_placeholder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Graphic> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Graphic = __cordl_object
            .invoke("get_placeholder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_preferredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_readOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_readOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionAnchorPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectionAnchorPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_selectionColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionFocusPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectionFocusPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldActivateOnSelect(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldActivateOnSelect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldHideMobileInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldHideMobileInput", ())?;
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
    pub fn get_textComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Text = __cordl_object
            .invoke("get_textComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_touchScreenKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TouchScreenKeyboard> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TouchScreenKeyboard = __cordl_object
            .invoke("get_touchScreenKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wasCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasCanceled", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_asteriskChar(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_asteriskChar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_caretBlinkRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_caretBlinkRate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_caretColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_caretColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_caretPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_caretPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_caretPositionInternal(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_caretPositionInternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_caretSelectPositionInternal(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_caretSelectPositionInternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_caretWidth(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_caretWidth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_characterLimit(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterLimit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_characterValidation(
        &mut self,
        value: crate::UnityEngine::UI::InputField_CharacterValidation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterValidation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_contentType(
        &mut self,
        value: crate::UnityEngine::UI::InputField_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_customCaretColor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customCaretColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_inputType(
        &mut self,
        value: crate::UnityEngine::UI::InputField_InputType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inputType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_keyboardType(
        &mut self,
        value: crate::UnityEngine::TouchScreenKeyboardType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_keyboardType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lineType(
        &mut self,
        value: crate::UnityEngine::UI::InputField_LineType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onEndEdit(
        &mut self,
        value: *mut crate::UnityEngine::UI::InputField_EndEditEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onEndEdit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onSubmit(
        &mut self,
        value: *mut crate::UnityEngine::UI::InputField_SubmitEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onSubmit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onValidateInput(
        &mut self,
        value: *mut crate::UnityEngine::UI::InputField_OnValidateInput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValidateInput", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onValueChange(
        &mut self,
        value: *mut crate::UnityEngine::UI::InputField_OnChangeEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValueChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onValueChanged(
        &mut self,
        value: *mut crate::UnityEngine::UI::InputField_OnChangeEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValueChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_placeholder(
        &mut self,
        value: *mut crate::UnityEngine::UI::Graphic,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_placeholder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_readOnly(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_readOnly", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectionAnchorPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionAnchorPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectionColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectionFocusPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionFocusPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_shouldActivateOnSelect(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shouldActivateOnSelect", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_shouldHideMobileInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shouldHideMobileInput", (value))?;
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
    pub fn set_textComponent(
        &mut self,
        value: *mut crate::UnityEngine::UI::Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textComponent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+InputField")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::InputField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+InputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField_InputType {
    AutoCorrect = 1i32,
    Password = 2i32,
    Standard = 0i32,
}
#[cfg(feature = "UnityEngine+UI+InputField+InputType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_InputType =>
    "UnityEngine.UI"."InputField/InputType"
);
#[cfg(feature = "UnityEngine+UI+InputField+LineType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputField_LineType {
    MultiLineNewline = 2i32,
    MultiLineSubmit = 1i32,
    SingleLine = 0i32,
}
#[cfg(feature = "UnityEngine+UI+InputField+LineType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_LineType =>
    "UnityEngine.UI"."InputField/LineType"
);
#[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct InputField_OnChangeEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>,
}
#[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_OnChangeEvent =>
    "UnityEngine.UI"."InputField/OnChangeEvent"
);
#[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::InputField_OnChangeEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UI::InputField_OnChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
impl crate::UnityEngine::UI::InputField_OnChangeEvent {
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
#[cfg(feature = "UnityEngine+UI+InputField+OnChangeEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::InputField_OnChangeEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
#[repr(C)]
#[derive(Debug)]
pub struct InputField_OnValidateInput {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_OnValidateInput =>
    "UnityEngine.UI"."InputField/OnValidateInput"
);
#[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
impl std::ops::Deref for crate::UnityEngine::UI::InputField_OnValidateInput {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
impl std::ops::DerefMut for crate::UnityEngine::UI::InputField_OnValidateInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
impl crate::UnityEngine::UI::InputField_OnValidateInput {
    pub fn BeginInvoke(
        &mut self,
        text: *mut crate::System::String,
        charIndex: i32,
        addedChar: char,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (text, charIndex, addedChar, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        text: *mut crate::System::String,
        charIndex: i32,
        addedChar: char,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object
            .invoke("Invoke", (text, charIndex, addedChar))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+OnValidateInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::InputField_OnValidateInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct InputField_SubmitEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>,
}
#[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::InputField_SubmitEvent =>
    "UnityEngine.UI"."InputField/SubmitEvent"
);
#[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
impl std::ops::Deref for crate::UnityEngine::UI::InputField_SubmitEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
impl std::ops::DerefMut for crate::UnityEngine::UI::InputField_SubmitEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
impl crate::UnityEngine::UI::InputField_SubmitEvent {
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
#[cfg(feature = "UnityEngine+UI+InputField+SubmitEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::InputField_SubmitEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
