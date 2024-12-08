#[cfg(feature = "TMPro+TMP_InputField+CharacterValidation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_InputField_CharacterValidation {
    Alphanumeric = 4i32,
    CustomValidator = 8i32,
    Decimal = 3i32,
    Digit = 1i32,
    EmailAddress = 7i32,
    Integer = 2i32,
    Name = 5i32,
    None = 0i32,
    Regex = 6i32,
}
#[cfg(feature = "TMPro+TMP_InputField+CharacterValidation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_CharacterValidation =>
    "TMPro"."TMP_InputField/CharacterValidation"
);
#[cfg(feature = "TMPro+TMP_InputField+ContentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_InputField_ContentType {
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
#[cfg(feature = "TMPro+TMP_InputField+ContentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_ContentType => "TMPro"
    ."TMP_InputField/ContentType"
);
#[cfg(feature = "TMPro+TMP_InputField+EditState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_InputField_EditState {
    Continue = 0i32,
    Finish = 1i32,
}
#[cfg(feature = "TMPro+TMP_InputField+EditState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_EditState => "TMPro"
    ."TMP_InputField/EditState"
);
#[cfg(feature = "TMPro+TMP_InputField+InputType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_InputField_InputType {
    AutoCorrect = 1i32,
    Password = 2i32,
    Standard = 0i32,
}
#[cfg(feature = "TMPro+TMP_InputField+InputType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_InputType => "TMPro"
    ."TMP_InputField/InputType"
);
#[cfg(feature = "TMPro+TMP_InputField+LineType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TMP_InputField_LineType {
    MultiLineNewline = 2i32,
    MultiLineSubmit = 1i32,
    SingleLine = 0i32,
}
#[cfg(feature = "TMPro+TMP_InputField+LineType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_LineType => "TMPro"
    ."TMP_InputField/LineType"
);
#[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField_OnChangeEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>,
}
#[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_OnChangeEvent => "TMPro"
    ."TMP_InputField/OnChangeEvent"
);
#[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
impl std::ops::Deref for crate::TMPro::TMP_InputField_OnChangeEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField_OnChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
impl crate::TMPro::TMP_InputField_OnChangeEvent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_InputField_OnChangeEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField_OnValidateInput {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_OnValidateInput => "TMPro"
    ."TMP_InputField/OnValidateInput"
);
#[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
impl std::ops::Deref for crate::TMPro::TMP_InputField_OnValidateInput {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField_OnValidateInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
impl crate::TMPro::TMP_InputField_OnValidateInput {
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
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_InputField_OnValidateInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField_SelectionEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>,
}
#[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_SelectionEvent => "TMPro"
    ."TMP_InputField/SelectionEvent"
);
#[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
impl std::ops::Deref for crate::TMPro::TMP_InputField_SelectionEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField_SelectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
impl crate::TMPro::TMP_InputField_SelectionEvent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_InputField_SelectionEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField_SubmitEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>,
}
#[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_SubmitEvent => "TMPro"
    ."TMP_InputField/SubmitEvent"
);
#[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
impl std::ops::Deref for crate::TMPro::TMP_InputField_SubmitEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<*mut crate::System::String>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField_SubmitEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
impl crate::TMPro::TMP_InputField_SubmitEvent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_InputField_SubmitEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_InputField")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField {
    __cordl_parent: crate::UnityEngine::UI::Selectable,
    pub m_SoftKeyboard: *mut crate::UnityEngine::TouchScreenKeyboard,
    pub m_RectTransform: *mut crate::UnityEngine::RectTransform,
    pub m_TextViewport: *mut crate::UnityEngine::RectTransform,
    pub m_TextComponentRectMask: *mut crate::UnityEngine::UI::RectMask2D,
    pub m_TextViewportRectMask: *mut crate::UnityEngine::UI::RectMask2D,
    pub m_CachedViewportRect: crate::UnityEngine::Rect,
    pub m_TextComponent: *mut crate::TMPro::TMP_Text,
    pub m_TextComponentRectTransform: *mut crate::UnityEngine::RectTransform,
    pub m_Placeholder: *mut crate::UnityEngine::UI::Graphic,
    pub m_VerticalScrollbar: *mut crate::UnityEngine::UI::Scrollbar,
    pub m_VerticalScrollbarEventHandler: *mut crate::TMPro::TMP_ScrollbarEventHandler,
    pub m_IsDrivenByLayoutComponents: bool,
    pub m_LayoutGroup: *mut crate::UnityEngine::UI::LayoutGroup,
    pub m_IScrollHandlerParent: *mut crate::UnityEngine::EventSystems::IScrollHandler,
    pub m_ScrollPosition: f32,
    pub m_ScrollSensitivity: f32,
    pub m_ContentType: crate::TMPro::TMP_InputField_ContentType,
    pub m_InputType: crate::TMPro::TMP_InputField_InputType,
    pub m_AsteriskChar: char,
    pub m_KeyboardType: crate::UnityEngine::TouchScreenKeyboardType,
    pub m_LineType: crate::TMPro::TMP_InputField_LineType,
    pub m_HideMobileInput: bool,
    pub m_HideSoftKeyboard: bool,
    pub m_CharacterValidation: crate::TMPro::TMP_InputField_CharacterValidation,
    pub m_RegexValue: *mut crate::System::String,
    pub m_GlobalPointSize: f32,
    pub m_CharacterLimit: i32,
    pub m_OnEndEdit: *mut crate::TMPro::TMP_InputField_SubmitEvent,
    pub m_OnSubmit: *mut crate::TMPro::TMP_InputField_SubmitEvent,
    pub m_OnSelect: *mut crate::TMPro::TMP_InputField_SelectionEvent,
    pub m_OnDeselect: *mut crate::TMPro::TMP_InputField_SelectionEvent,
    pub m_OnTextSelection: *mut crate::TMPro::TMP_InputField_TextSelectionEvent,
    pub m_OnEndTextSelection: *mut crate::TMPro::TMP_InputField_TextSelectionEvent,
    pub m_OnValueChanged: *mut crate::TMPro::TMP_InputField_OnChangeEvent,
    pub m_OnTouchScreenKeyboardStatusChanged: *mut crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent,
    pub m_OnValidateInput: *mut crate::TMPro::TMP_InputField_OnValidateInput,
    pub m_CaretColor: crate::UnityEngine::Color,
    pub m_CustomCaretColor: bool,
    pub m_SelectionColor: crate::UnityEngine::Color,
    pub m_Text: *mut crate::System::String,
    pub m_CaretBlinkRate: f32,
    pub m_CaretWidth: i32,
    pub m_ReadOnly: bool,
    pub m_RichText: bool,
    pub m_StringPosition: i32,
    pub m_StringSelectPosition: i32,
    pub m_CaretPosition: i32,
    pub m_CaretSelectPosition: i32,
    pub caretRectTrans: *mut crate::UnityEngine::RectTransform,
    pub m_CursorVerts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::UIVertex,
    >,
    pub m_CachedInputRenderer: *mut crate::UnityEngine::CanvasRenderer,
    pub m_LastPosition: crate::UnityEngine::Vector2,
    pub m_Mesh: *mut crate::UnityEngine::Mesh,
    pub m_AllowInput: bool,
    pub m_ShouldActivateNextUpdate: bool,
    pub m_UpdateDrag: bool,
    pub m_DragPositionOutOfBounds: bool,
    pub m_CaretVisible: bool,
    pub m_BlinkCoroutine: *mut crate::UnityEngine::Coroutine,
    pub m_BlinkStartTime: f32,
    pub m_DragCoroutine: *mut crate::UnityEngine::Coroutine,
    pub m_OriginalText: *mut crate::System::String,
    pub m_WasCanceled: bool,
    pub m_HasDoneFocusTransition: bool,
    pub m_WaitForSecondsRealtime: *mut crate::UnityEngine::WaitForSecondsRealtime,
    pub m_PreventCallback: bool,
    pub m_TouchKeyboardAllowsInPlaceEditing: bool,
    pub m_IsTextComponentUpdateRequired: bool,
    pub m_isLastKeyBackspace: bool,
    pub m_PointerDownClickStartTime: f32,
    pub m_KeyDownStartTime: f32,
    pub m_DoubleClickDelay: f32,
    pub m_IsCompositionActive: bool,
    pub m_ShouldUpdateIMEWindowPosition: bool,
    pub m_PreviousIMEInsertionLine: i32,
    pub m_GlobalFontAsset: *mut crate::TMPro::TMP_FontAsset,
    pub m_OnFocusSelectAll: bool,
    pub m_isSelectAll: bool,
    pub m_ResetOnDeActivation: bool,
    pub m_SelectionStillActive: bool,
    pub m_ReleaseSelection: bool,
    pub m_PreviouslySelectedObject: *mut crate::UnityEngine::GameObject,
    pub m_RestoreOriginalTextOnEscape: bool,
    pub m_isRichTextEditingAllowed: bool,
    pub m_LineLimit: i32,
    pub m_InputValidator: *mut crate::TMPro::TMP_InputValidator,
    pub m_isSelected: bool,
    pub m_IsStringPositionDirty: bool,
    pub m_IsCaretPositionDirty: bool,
    pub m_forceRectTransformAdjustment: bool,
    pub m_ProcessingEvent: *mut crate::UnityEngine::Event,
}
#[cfg(feature = "TMPro+TMP_InputField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField => "TMPro"
    ."TMP_InputField"
);
#[cfg(feature = "TMPro+TMP_InputField")]
impl std::ops::Deref for crate::TMPro::TMP_InputField {
    type Target = crate::UnityEngine::UI::Selectable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField")]
impl crate::TMPro::TMP_InputField {
    pub const kEmailSpecialCharacters: &'static str = "!#$%&\\'*+-/=?^_`{|}~";
    pub const kHScrollSpeed: f32 = 0.05f32;
    pub const kVScrollSpeed: f32 = 0.1f32;
    #[cfg(feature = "TMPro+TMP_InputField+InputType")]
    pub type InputType = crate::TMPro::TMP_InputField_InputType;
    #[cfg(feature = "TMPro+TMP_InputField+OnChangeEvent")]
    pub type OnChangeEvent = crate::TMPro::TMP_InputField_OnChangeEvent;
    #[cfg(feature = "TMPro+TMP_InputField+ContentType")]
    pub type ContentType = crate::TMPro::TMP_InputField_ContentType;
    #[cfg(feature = "TMPro+TMP_InputField+CharacterValidation")]
    pub type CharacterValidation = crate::TMPro::TMP_InputField_CharacterValidation;
    #[cfg(feature = "TMPro+TMP_InputField+SubmitEvent")]
    pub type SubmitEvent = crate::TMPro::TMP_InputField_SubmitEvent;
    #[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
    pub type TouchScreenKeyboardEvent = crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent;
    #[cfg(feature = "TMPro+TMP_InputField+LineType")]
    pub type LineType = crate::TMPro::TMP_InputField_LineType;
    #[cfg(feature = "TMPro+TMP_InputField+SelectionEvent")]
    pub type SelectionEvent = crate::TMPro::TMP_InputField_SelectionEvent;
    #[cfg(feature = "TMPro+TMP_InputField+_MouseDragOutsideRect_d__294")]
    pub type _MouseDragOutsideRect_d__294 = crate::TMPro::TMP_InputField__MouseDragOutsideRect_d__294;
    #[cfg(feature = "TMPro+TMP_InputField+OnValidateInput")]
    pub type OnValidateInput = crate::TMPro::TMP_InputField_OnValidateInput;
    #[cfg(feature = "TMPro+TMP_InputField+EditState")]
    pub type EditState = crate::TMPro::TMP_InputField_EditState;
    #[cfg(feature = "TMPro+TMP_InputField+_CaretBlink_d__276")]
    pub type _CaretBlink_d__276 = crate::TMPro::TMP_InputField__CaretBlink_d__276;
    #[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
    pub type TextSelectionEvent = crate::TMPro::TMP_InputField_TextSelectionEvent;
    pub fn MoveToStartOfLine(
        &mut self,
        shift: bool,
        ctrl: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToStartOfLine", (shift, ctrl))?;
        Ok(__cordl_ret)
    }
    pub fn InPlaceEditing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InPlaceEditing", ())?;
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
    pub fn get_textComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_Text> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_Text = __cordl_object
            .invoke("get_textComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onEndTextSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::TMPro::TMP_InputField_TextSelectionEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_TextSelectionEvent = __cordl_object
            .invoke("get_onEndTextSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionAnchorPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectionAnchorPosition", ())?;
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
    pub fn set_contentType(
        &mut self,
        value: crate::TMPro::TMP_InputField_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_contentType", (value))?;
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
    pub fn set_onSubmit(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_SubmitEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onSubmit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetTextComponentWrapMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextComponentWrapMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretBlinkRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_caretBlinkRate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasSelection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTextComponentRichTextMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextComponentRichTextMode", ())?;
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
    pub fn get_onEndEdit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_InputField_SubmitEvent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_SubmitEvent = __cordl_object
            .invoke("get_onEndEdit", ())?;
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
    pub fn get_onFocusSelectAll(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_onFocusSelectAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onDeselect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::TMPro::TMP_InputField_SelectionEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_SelectionEvent = __cordl_object
            .invoke("get_onDeselect", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_onValidateInput(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_OnValidateInput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValidateInput", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_onSubmit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_InputField_SubmitEvent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_SubmitEvent = __cordl_object
            .invoke("get_onSubmit", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClampCaretPos(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClampCaretPos", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn set_onEndEdit(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_SubmitEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onEndEdit", (value))?;
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
    pub fn MovePageUp__cordl_bool0(
        &mut self,
        shift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MovePageUp", (shift))?;
        Ok(__cordl_ret)
    }
    pub fn MovePageUp__cordl_bool1(
        &mut self,
        shift: bool,
        goToFirstChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MovePageUp", (shift, goToFirstChar))?;
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
    pub fn get_inputValidator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_InputValidator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputValidator = __cordl_object
            .invoke("get_inputValidator", ())?;
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
    pub fn SetGlobalFontAsset(
        &mut self,
        fontAsset: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalFontAsset", (fontAsset))?;
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
    pub fn set_onEndTextSelection(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_TextSelectionEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onEndTextSelection", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_characterValidation(
        &mut self,
        value: crate::TMPro::TMP_InputField_CharacterValidation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_characterValidation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_textViewport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_textViewport", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_stringPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stringPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_resetOnDeActivation(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_resetOnDeActivation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStringPositionFromKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStringPositionFromKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnScrollbarValueChange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScrollbarValueChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetMinCaretPositionFromStringIndex(
        &mut self,
        stringIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMinCaretPositionFromStringIndex", (stringIndex))?;
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
    pub fn get_preferredWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredWidth", ())?;
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
    pub fn get_flexibleHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn KeyPressed(
        &mut self,
        evt: *mut crate::UnityEngine::Event,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_InputField_EditState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_InputField_EditState = __cordl_object
            .invoke("KeyPressed", (evt))?;
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
    pub fn get_isFocused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFocused", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_restoreOriginalTextOnEscape(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_restoreOriginalTextOnEscape", (value))?;
        Ok(__cordl_ret)
    }
    pub fn PageDownCharacterPosition(
        &mut self,
        originalPos: i32,
        goToLastChar: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("PageDownCharacterPosition", (originalPos, goToLastChar))?;
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
    pub fn SetToCustomIfContentTypeIsNot(
        &mut self,
        allowedContentTypes: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::TMPro::TMP_InputField_ContentType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToCustomIfContentTypeIsNot", (allowedContentTypes))?;
        Ok(__cordl_ret)
    }
    pub fn get_lineType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_InputField_LineType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_InputField_LineType = __cordl_object
            .invoke("get_lineType", ())?;
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
    pub fn get_preferredHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_preferredHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_textComponent(
        &mut self,
        value: *mut crate::TMPro::TMP_Text,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textComponent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_inputSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::BaseInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::BaseInput = __cordl_object
            .invoke("get_inputSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_shouldHideSoftKeyboard(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shouldHideSoftKeyboard", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionStringAnchorPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_selectionStringAnchorPosition", ())?;
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
    pub fn get_onTouchScreenKeyboardStatusChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent = __cordl_object
            .invoke("get_onTouchScreenKeyboardStatusChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetScrollPositionRelativeToViewport(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetScrollPositionRelativeToViewport", ())?;
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
    pub fn get_selectionFocusPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectionFocusPosition", ())?;
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
    pub fn DeleteKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendTouchScreenKeyboardStatusChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendTouchScreenKeyboardStatusChanged", ())?;
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
    pub fn get_layoutPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layoutPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindNextWordBegin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindNextWordBegin", ())?;
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
    pub fn MovePageDown__cordl_bool0(
        &mut self,
        shift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MovePageDown", (shift))?;
        Ok(__cordl_ret)
    }
    pub fn MovePageDown__cordl_bool1(
        &mut self,
        shift: bool,
        goToLastChar: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MovePageDown", (shift, goToLastChar))?;
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
    pub fn get_stringSelectPositionInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_stringSelectPositionInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn isKeyboardUsingEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("isKeyboardUsingEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isRichTextEditingAllowed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isRichTextEditingAllowed", (value))?;
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
    pub fn set_textViewport(
        &mut self,
        value: *mut crate::UnityEngine::RectTransform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textViewport", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetStringIndexFromCaretPosition(
        &mut self,
        caretPosition: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetStringIndexFromCaretPosition", (caretPosition))?;
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
    pub fn ReleaseSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_resetOnDeActivation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_resetOnDeActivation", ())?;
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
    pub fn set_lineType(
        &mut self,
        value: crate::TMPro::TMP_InputField_LineType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateHightlight(
        &mut self,
        vbo: *mut crate::UnityEngine::UI::VertexHelper,
        roundingOffset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateHightlight", (vbo, roundingOffset))?;
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
    pub fn set_inputType(
        &mut self,
        value: crate::TMPro::TMP_InputField_InputType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inputType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_inputType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_InputField_InputType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_InputField_InputType = __cordl_object
            .invoke("get_inputType", ())?;
        Ok(__cordl_ret)
    }
    pub fn AdjustRectTransformRelativeToViewport(
        &mut self,
        startPosition: crate::UnityEngine::Vector2,
        height: f32,
        isCharVisible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AdjustRectTransformRelativeToViewport",
                (startPosition, height, isCharVisible),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_minWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_characterLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_characterLimit", ())?;
        Ok(__cordl_ret)
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
    pub fn SetToCustom_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToCustom", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetToCustom_TMP_InputField_CharacterValidation1(
        &mut self,
        characterValidation: crate::TMPro::TMP_InputField_CharacterValidation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetToCustom", (characterValidation))?;
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
    pub fn get_caretPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_caretPosition", ())?;
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
    pub fn get_shouldHideMobileInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldHideMobileInput", ())?;
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
    pub fn set_verticalScrollbar(
        &mut self,
        value: *mut crate::UnityEngine::UI::Scrollbar,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalScrollbar", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_onValidateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::TMPro::TMP_InputField_OnValidateInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_OnValidateInput = __cordl_object
            .invoke("get_onValidateInput", ())?;
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
    pub fn GetMaxCaretPositionFromStringIndex(
        &mut self,
        stringIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMaxCaretPositionFromStringIndex", (stringIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_multiLine(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_multiLine", ())?;
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
    pub fn SetGlobalPointSize(
        &mut self,
        pointSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalPointSize", (pointSize))?;
        Ok(__cordl_ret)
    }
    pub fn get_stringPositionInternal(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_stringPositionInternal", ())?;
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
    pub fn get_contentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_InputField_ContentType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_InputField_ContentType = __cordl_object
            .invoke("get_contentType", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScrollbar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScrollbar", ())?;
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
    pub fn ON_TEXT_CHANGED(
        &mut self,
        obj: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ON_TEXT_CHANGED", (obj))?;
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
    pub fn FindPrevWordBegin(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindPrevWordBegin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineLimit", ())?;
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
    pub fn get_asteriskChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_asteriskChar", ())?;
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
    pub fn get_shouldHideSoftKeyboard(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldHideSoftKeyboard", ())?;
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
    pub fn DeactivateInputField(
        &mut self,
        clearSelection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateInputField", (clearSelection))?;
        Ok(__cordl_ret)
    }
    pub fn get_isRichTextEditingAllowed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isRichTextEditingAllowed", ())?;
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
    pub fn set_lineLimit(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineLimit", (value))?;
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
    pub fn get_stringPosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_stringPosition", ())?;
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
    pub fn get_compositionLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_compositionLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onSelect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::TMPro::TMP_InputField_SelectionEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_SelectionEvent = __cordl_object
            .invoke("get_onSelect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionStringFocusPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_selectionStringFocusPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_onFocusSelectAll(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onFocusSelectAll", (value))?;
        Ok(__cordl_ret)
    }
    pub fn MoveToEndOfLine(
        &mut self,
        shift: bool,
        ctrl: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToEndOfLine", (shift, ctrl))?;
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
    pub fn set_onValueChanged(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_OnChangeEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onValueChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendOnFocus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnFocus", ())?;
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
    pub fn get_onValueChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_InputField_OnChangeEvent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_OnChangeEvent = __cordl_object
            .invoke("get_onValueChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fontAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::TMPro::TMP_FontAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_FontAsset = __cordl_object
            .invoke("get_fontAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_inputValidator(
        &mut self,
        value: *mut crate::TMPro::TMP_InputValidator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inputValidator", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_selectionStringAnchorPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionStringAnchorPosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AdjustTextPositionRelativeToViewport(
        &mut self,
        relativePosition: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustTextPositionRelativeToViewport", (relativePosition))?;
        Ok(__cordl_ret)
    }
    pub fn get_customCaretColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_customCaretColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCaretPositionFromStringIndex(
        &mut self,
        stringIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCaretPositionFromStringIndex", (stringIndex))?;
        Ok(__cordl_ret)
    }
    pub fn get_pointSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pointSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnControlClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnControlClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexibleWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexibleWidth", ())?;
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
    pub fn set_selectionStringFocusPosition(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectionStringFocusPosition", (value))?;
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
    pub fn set_scrollSensitivity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollSensitivity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onDeselect(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_SelectionEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onDeselect", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_wasCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasCanceled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalScrollbar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Scrollbar> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Scrollbar = __cordl_object
            .invoke("get_verticalScrollbar", ())?;
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
    pub fn set_stringSelectPositionInternal(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stringSelectPositionInternal", (value))?;
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
    pub fn OnScroll(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScroll", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn SendOnFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnFocusLost", ())?;
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
    pub fn SendOnTextSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnTextSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_caretPositionInternal(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_caretPositionInternal", ())?;
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
    pub fn PageUpCharacterPosition(
        &mut self,
        originalPos: i32,
        goToFirstChar: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("PageUpCharacterPosition", (originalPos, goToFirstChar))?;
        Ok(__cordl_ret)
    }
    pub fn IsValidChar(&mut self, c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValidChar", (c))?;
        Ok(__cordl_ret)
    }
    pub fn set_stringPositionInternal(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stringPositionInternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_restoreOriginalTextOnEscape(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_restoreOriginalTextOnEscape", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_onTextSelection(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_TextSelectionEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onTextSelection", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_richText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_richText", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClampStringPos(
        &mut self,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClampStringPos", (pos))?;
        Ok(__cordl_ret)
    }
    pub fn get_caretWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_caretWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_pointSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pointSize", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendOnEndTextSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendOnEndTextSelection", ())?;
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
    pub fn get_scrollSensitivity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollSensitivity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_readOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_readOnly", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMaskRegions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMaskRegions", ())?;
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
    pub fn set_fontAsset(
        &mut self,
        value: *mut crate::TMPro::TMP_FontAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_onTextSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::TMPro::TMP_InputField_TextSelectionEvent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::TMPro::TMP_InputField_TextSelectionEvent = __cordl_object
            .invoke("get_onTextSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_richText(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_richText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_characterValidation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::TMPro::TMP_InputField_CharacterValidation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::TMPro::TMP_InputField_CharacterValidation = __cordl_object
            .invoke("get_characterValidation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_onTouchScreenKeyboardStatusChanged(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onTouchScreenKeyboardStatusChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onSelect(
        &mut self,
        value: *mut crate::TMPro::TMP_InputField_SelectionEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onSelect", (value))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_InputField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField_TextSelectionEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_3<
        *mut crate::System::String,
        i32,
        i32,
    >,
}
#[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_TextSelectionEvent =>
    "TMPro"."TMP_InputField/TextSelectionEvent"
);
#[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
impl std::ops::Deref for crate::TMPro::TMP_InputField_TextSelectionEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_3<
        *mut crate::System::String,
        i32,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField_TextSelectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
impl crate::TMPro::TMP_InputField_TextSelectionEvent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TextSelectionEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::TMPro::TMP_InputField_TextSelectionEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_InputField_TouchScreenKeyboardEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        crate::UnityEngine::TouchScreenKeyboard_Status,
    >,
}
#[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_InputField_TouchScreenKeyboardEvent
    => "TMPro"."TMP_InputField/TouchScreenKeyboardEvent"
);
#[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
impl std::ops::Deref for crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        crate::UnityEngine::TouchScreenKeyboard_Status,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
impl std::ops::DerefMut for crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
impl crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_InputField+TouchScreenKeyboardEvent")]
impl quest_hook::libil2cpp::ObjectType
for crate::TMPro::TMP_InputField_TouchScreenKeyboardEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
