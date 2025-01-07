#[cfg(feature = "UnityEngine+UIElements+TextElement")]
#[repr(C)]
#[derive(Debug)]
pub struct TextElement {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement,
    pub _uitkTextHandle_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UITKTextHandle,
    >,
    pub m_Text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_EnableRichText: bool,
    pub m_ParseEscapeSequences: bool,
    pub m_DisplayTooltipWhenElided: bool,
    pub _isElided_k__BackingField: bool,
    pub elidedText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_WasElided: bool,
    pub editingManipulator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextEditingManipulator,
    >,
    pub m_Multiline: bool,
    pub m_TouchScreenKeyboard: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TouchScreenKeyboard,
    >,
    pub m_KeyboardType: crate::UnityEngine::TouchScreenKeyboardType,
    pub m_HideMobileInput: bool,
    pub m_IsReadOnly: bool,
    pub m_MaxLength: i32,
    pub _UnityEngine_UIElements_ITextEdition_isDelayed_k__BackingField: bool,
    pub _UnityEngine_UIElements_ITextEdition_AcceptCharacter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<char, bool>,
    >,
    pub _UnityEngine_UIElements_ITextEdition_UpdateScrollOffset_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<bool>,
    >,
    pub _UnityEngine_UIElements_ITextEdition_UpdateValueFromText_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _UnityEngine_UIElements_ITextEdition_UpdateTextFromValue_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _UnityEngine_UIElements_ITextEdition_MoveFocusToCompositeRoot_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub m_RenderedText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_OriginalText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_MaskChar: char,
    pub m_IsPassword: bool,
    pub m_AutoCorrection: bool,
    pub m_SelectingManipulator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextSelectingManipulator,
    >,
    pub m_IsSelectable: bool,
    pub _UnityEngine_UIElements_ITextSelection_doubleClickSelectsWord_k__BackingField: bool,
    pub _UnityEngine_UIElements_ITextSelection_tripleClickSelectsLine_k__BackingField: bool,
    pub _UnityEngine_UIElements_ITextSelection_selectAllOnFocus_k__BackingField: bool,
    pub _UnityEngine_UIElements_ITextSelection_selectAllOnMouseUp_k__BackingField: bool,
    pub m_SelectionColor: crate::UnityEngine::Color,
    pub m_CursorColor: crate::UnityEngine::Color,
    pub m_CursorWidth: f32,
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::TextElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TextElement";
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
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextElement {
    type Target = crate::UnityEngine::UIElements::BindableElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl crate::UnityEngine::UIElements::TextElement {
    #[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::TextElement_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::TextElement_UxmlTraits;
    pub fn BuildContextualMenu(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ContextualMenuPopulateEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildContextualMenu", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Copy", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyActionStatus(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DropdownMenuAction_Status = __cordl_object
            .invoke("CopyActionStatus", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cut(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cut", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn CutActionStatus(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DropdownMenuAction_Status = __cordl_object
            .invoke("CutActionStatus", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoMeasure(
        &mut self,
        desiredWidth: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        desiredHeight: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("DoMeasure", (desiredWidth, widthMode, desiredHeight, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawCaret(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawCaret", (mgc))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawHighlighting(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawHighlighting", (mgc))?;
        Ok(__cordl_ret.into())
    }
    pub fn ElideText(
        &mut self,
        drawText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ellipsisText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: f32,
        textOverflowPosition: crate::UnityEngine::UIElements::TextOverflowPosition,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("ElideText", (drawText, ellipsisText, width, textOverflowPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDefaultActionAtTarget(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultActionAtTarget", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastCharacterAt(
        &mut self,
        lineIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetLastCharacterAt", (lineIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn MeasureTextSize(
        &mut self,
        textToMeasure: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke(
                "MeasureTextSize",
                (textToMeasure, width, widthMode, height, heightMode),
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
    pub fn OnGenerateVisualContent(
        &mut self,
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGenerateVisualContent", (mgc))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnGeometryChanged(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGeometryChanged", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn Paste(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Paste", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn PasteActionStatus(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DropdownMenuAction>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DropdownMenuAction_Status,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DropdownMenuAction_Status = __cordl_object
            .invoke("PasteActionStatus", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMenuCommand(
        &mut self,
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMenuCommand", (command))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldElide(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldElide", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_INotifyValueChanged_System_String__SetValueWithoutNotify(
        &mut self,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.INotifyValueChanged<System.String>.SetValueWithoutNotify",
                (newValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_INotifyValueChanged_System_String__get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke(
                "UnityEngine.UIElements.INotifyValueChanged<System.String>.get_value",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_INotifyValueChanged_System_String__set_value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.INotifyValueChanged<System.String>.set_value",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_CullString(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.CullString", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_RestoreValueAndText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.RestoreValueAndText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_SaveValueAndText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.SaveValueAndText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_UpdateText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.UpdateText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_AcceptCharacter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_2<char, bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Func_2<char, bool>> = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_AcceptCharacter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_MoveFocusToCompositeRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextEdition.get_MoveFocusToCompositeRoot",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_UpdateScrollOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>> = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_UpdateScrollOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_UpdateTextFromValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_UpdateTextFromValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_UpdateValueFromText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_UpdateValueFromText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_autoCorrection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_autoCorrection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_hideMobileInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_hideMobileInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_isDelayed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_isDelayed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_isPassword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_isPassword", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_isReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_isReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_keyboardType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TouchScreenKeyboardType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TouchScreenKeyboardType = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_keyboardType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_maxLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_maxLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_get_multiline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.get_multiline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_AcceptCharacter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Func_2<char, bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_AcceptCharacter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_MoveFocusToCompositeRoot(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextEdition.set_MoveFocusToCompositeRoot",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_UpdateScrollOffset(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextEdition.set_UpdateScrollOffset",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_UpdateTextFromValue(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextEdition.set_UpdateTextFromValue",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_UpdateValueFromText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextEdition.set_UpdateValueFromText",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_autoCorrection(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_autoCorrection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_hideMobileInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_hideMobileInput", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_isDelayed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_isDelayed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_isPassword(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_isPassword", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_isReadOnly(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_isReadOnly", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_keyboardType(
        &mut self,
        value: crate::UnityEngine::TouchScreenKeyboardType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_keyboardType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_maskChar(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_maskChar", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_maxLength(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_maxLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextEdition_set_multiline(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextEdition.set_multiline", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_HasSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.HasSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_SelectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.SelectAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_SelectNone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.SelectNone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_cursorColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_cursorColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_cursorIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_cursorIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_cursorPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_cursorPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_cursorWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_cursorWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_doubleClickSelectsWord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextSelection.get_doubleClickSelectsWord",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_isSelectable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_isSelectable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_lineHeightAtCursorPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextSelection.get_lineHeightAtCursorPosition",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_selectAllOnFocus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_selectAllOnFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_selectAllOnMouseUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_selectAllOnMouseUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_selectIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_selectIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_selectionColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.get_selectionColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_get_tripleClickSelectsLine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextSelection.get_tripleClickSelectsLine",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_cursorColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.set_cursorColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_cursorIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.set_cursorIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_isSelectable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.set_isSelectable", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_selectAllOnFocus(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextSelection.set_selectAllOnFocus",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_selectAllOnMouseUp(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextSelection.set_selectAllOnMouseUp",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_selectIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.ITextSelection.set_selectIndex", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_ITextSelection_set_selectionColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.ITextSelection.set_selectionColor",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTooltip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTooltip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibleText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibleText", ())?;
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
    pub fn get_displayTooltipWhenElided(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_displayTooltipWhenElided", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_edition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ITextEdition>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ITextEdition,
        > = __cordl_object.invoke("get_edition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_effectiveMaskChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_effectiveMaskChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableRichText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableRichText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isElided(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isElided", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_originalText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_originalText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_parseEscapeSequences(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_parseEscapeSequences", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_renderedText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectingManipulator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextSelectingManipulator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TextSelectingManipulator,
        > = __cordl_object.invoke("get_selectingManipulator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ITextSelection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ITextSelection,
        > = __cordl_object.invoke("get_selection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_text", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uitkTextHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UITKTextHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UITKTextHandle,
        > = __cordl_object.invoke("get_uitkTextHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_displayTooltipWhenElided(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayTooltipWhenElided", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableRichText(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableRichText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isElided(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isElided", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_parseEscapeSequences(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parseEscapeSequences", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderedText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderedText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_text(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uitkTextHandle(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UITKTextHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uitkTextHandle", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::TextElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsRef<crate::UnityEngine::UIElements::IExperimentalFeatures>
for crate::UnityEngine::UIElements::TextElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IExperimentalFeatures {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsMut<crate::UnityEngine::UIElements::IExperimentalFeatures>
for crate::UnityEngine::UIElements::TextElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IExperimentalFeatures {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsRef<
    crate::UnityEngine::UIElements::INotifyValueChanged_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
> for crate::UnityEngine::UIElements::TextElement {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::UIElements::INotifyValueChanged_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsMut<
    crate::UnityEngine::UIElements::INotifyValueChanged_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
> for crate::UnityEngine::UIElements::TextElement {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::INotifyValueChanged_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsRef<crate::UnityEngine::UIElements::ITextEdition>
for crate::UnityEngine::UIElements::TextElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::ITextEdition {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsMut<crate::UnityEngine::UIElements::ITextEdition>
for crate::UnityEngine::UIElements::TextElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::ITextEdition {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsRef<crate::UnityEngine::UIElements::ITextSelection>
for crate::UnityEngine::UIElements::TextElement {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::ITextSelection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement")]
impl AsMut<crate::UnityEngine::UIElements::ITextSelection>
for crate::UnityEngine::UIElements::TextElement {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::ITextSelection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct TextElement_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement_UxmlTraits>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TextElement_UxmlFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UxmlFactory";
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
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextElement_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement_UxmlTraits>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextElement_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
impl crate::UnityEngine::UIElements::TextElement_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextElement_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct TextElement_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement_UxmlTraits,
    pub m_Text: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_EnableRichText: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_ParseEscapeSequences: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_DisplayTooltipWhenElided: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TextElement_UxmlTraits {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UxmlTraits";
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
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TextElement_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BindableElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TextElement_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
impl crate::UnityEngine::UIElements::TextElement_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "UnityEngine+UIElements+TextElement+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TextElement_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
