#[cfg(feature = "UnityEngine+GUIUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GUIUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+GUIUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIUtility => "UnityEngine"
    ."GUIUtility"
);
#[cfg(feature = "UnityEngine+GUIUtility")]
impl std::ops::Deref for crate::UnityEngine::GUIUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIUtility")]
impl std::ops::DerefMut for crate::UnityEngine::GUIUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIUtility")]
impl crate::UnityEngine::GUIUtility {
    pub fn AlignRectToDevice_ByRefMut_ByRefMut0(
        rect: crate::UnityEngine::Rect,
        widthInPixels: quest_hook::libil2cpp::ByRefMut<i32>,
        heightInPixels: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignRectToDevice", (rect, widthInPixels, heightInPixels))?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignRectToDevice_Injected(
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        widthInPixels: quest_hook::libil2cpp::ByRefMut<i32>,
        heightInPixels: quest_hook::libil2cpp::ByRefMut<i32>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AlignRectToDevice_Injected",
                (rect, widthInPixels, heightInPixels, ret),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignRectToDevice_Rect1(
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignRectToDevice", (rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginContainer(
        objectGUIState: quest_hook::libil2cpp::Gc<crate::UnityEngine::ObjectGUIState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginContainer", (objectGUIState))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginContainerFromOwner(
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginContainerFromOwner", (owner))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginGUI(
        skinMode: i32,
        instanceID: i32,
        useGUILayout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginGUI", (skinMode, instanceID, useGUILayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForTabEvent(
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckForTabEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckOnGUI() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckOnGUI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyGUI(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyGUI", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndContainer() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndContainerGUIFromException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndContainerGUIFromException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGUI(
        layoutType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndGUI", (layoutType))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndGUIFromException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndGUIFromException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExitGUI() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExitGUI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControlID_Rect0(
        hint: i32,
        focusType: crate::UnityEngine::FocusType,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControlID", (hint, focusType, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetControlID_i32_FocusType1(
        hint: i32,
        focus: crate::UnityEngine::FocusType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetControlID", (hint, focus))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultSkin() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUISkin> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultSkin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFocusableControls() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasFocusableControls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasKeyFocus(controlID: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasKeyFocus", (controlID))?;
        Ok(__cordl_ret.into())
    }
    pub fn HitTest__cordl_bool1(
        rect: crate::UnityEngine::Rect,
        point: crate::UnityEngine::Vector2,
        isDirectManipulationDevice: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HitTest", (rect, point, isDirectManipulationDevice))?;
        Ok(__cordl_ret.into())
    }
    pub fn HitTest_i32_0(
        rect: crate::UnityEngine::Rect,
        point: crate::UnityEngine::Vector2,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HitTest", (rect, point, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_EndContainer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_EndContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ExitGUI() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ExitGUI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetControlID(
        hint: i32,
        focusType: crate::UnityEngine::FocusType,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetControlID", (hint, focusType, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetControlID_Injected(
        hint: i32,
        focusType: crate::UnityEngine::FocusType,
        rect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetControlID_Injected", (hint, focusType, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetDefaultSkin(
        skinMode: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetDefaultSkin", (skinMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetHotControl() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetHotControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetKeyboardControl() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetKeyboardControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetHotControl(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_SetHotControl", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetKeyboardControl(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_SetKeyboardControl", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsExitGUIException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsExitGUIException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkGUIChanged() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MarkGUIChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OwnsId(id: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OwnsId", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessEvent(
        instanceID: i32,
        nativeEventPtr: crate::System::IntPtr,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProcessEvent", (instanceID, nativeEventPtr, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCapture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetGlobalState() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetGlobalState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundToPixelGrid(v: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundToPixelGrid", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardControlToFirstControlId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetKeyboardControlToFirstControlId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardControlToLastControlId() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetKeyboardControlToLastControlId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldRethrowException(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldRethrowException", (exception))?;
        Ok(__cordl_ret.into())
    }
    pub fn TakeCapture() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TakeCapture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_compositionString() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_compositionString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_guiDepth() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_guiDepth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hotControl() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_hotControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_keyboardControl() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_keyboardControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pixelsPerPoint() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pixelsPerPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_systemCopyBuffer() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_systemCopyBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textFieldInput() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_textFieldInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compositionCursorPos(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_compositionCursorPos", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compositionCursorPos_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_compositionCursorPos_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_guiIsExiting(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_guiIsExiting", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hotControl(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_hotControl", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_imeCompositionMode(
        value: crate::UnityEngine::IMECompositionMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_imeCompositionMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_keyboardControl(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_keyboardControl", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mouseUsed(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_mouseUsed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_systemCopyBuffer(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_systemCopyBuffer", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUIUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUIUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
