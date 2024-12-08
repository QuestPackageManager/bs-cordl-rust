#[cfg(feature = "UnityEngine+GUIStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct GUIStyle {
    __cordl_parent: crate::System::Object,
    pub m_Ptr: crate::System::IntPtr,
    pub m_Normal: *mut crate::UnityEngine::GUIStyleState,
    pub m_Hover: *mut crate::UnityEngine::GUIStyleState,
    pub m_Active: *mut crate::UnityEngine::GUIStyleState,
    pub m_Focused: *mut crate::UnityEngine::GUIStyleState,
    pub m_OnNormal: *mut crate::UnityEngine::GUIStyleState,
    pub m_OnHover: *mut crate::UnityEngine::GUIStyleState,
    pub m_OnActive: *mut crate::UnityEngine::GUIStyleState,
    pub m_OnFocused: *mut crate::UnityEngine::GUIStyleState,
    pub m_Border: *mut crate::UnityEngine::RectOffset,
    pub m_Padding: *mut crate::UnityEngine::RectOffset,
    pub m_Margin: *mut crate::UnityEngine::RectOffset,
    pub m_Overflow: *mut crate::UnityEngine::RectOffset,
    pub m_Name: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+GUIStyle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIStyle => "UnityEngine"
    ."GUIStyle"
);
#[cfg(feature = "UnityEngine+GUIStyle")]
impl std::ops::Deref for crate::UnityEngine::GUIStyle {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIStyle")]
impl std::ops::DerefMut for crate::UnityEngine::GUIStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIStyle")]
impl crate::UnityEngine::GUIStyle {
    pub fn Draw__cordl_bool__cordl_bool0(
        &mut self,
        position: crate::UnityEngine::Rect,
        content: *mut crate::UnityEngine::GUIContent,
        isHover: bool,
        isActive: bool,
        on: bool,
        hasKeyboardFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Draw",
                (position, content, isHover, isActive, on, hasKeyboardFocus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Draw_i32_1(
        &mut self,
        position: crate::UnityEngine::Rect,
        content: *mut crate::UnityEngine::GUIContent,
        controlID: i32,
        on: bool,
        hover: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Draw", (position, content, controlID, on, hover))?;
        Ok(__cordl_ret)
    }
    pub fn Draw_i32__cordl_bool__cordl_bool2(
        &mut self,
        position: crate::UnityEngine::Rect,
        content: *mut crate::UnityEngine::GUIContent,
        controlId: i32,
        isHover: bool,
        isActive: bool,
        on: bool,
        hasKeyboardFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Draw",
                (position, content, controlId, isHover, isActive, on, hasKeyboardFocus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRectOffsetPtr(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetRectOffsetPtr", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn GetStyleStatePtr(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetStyleStatePtr", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn Internal_Draw(
        &mut self,
        screenRect: crate::UnityEngine::Rect,
        content: *mut crate::UnityEngine::GUIContent,
        isHover: bool,
        isActive: bool,
        on: bool,
        hasKeyboardFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_Draw",
                (screenRect, content, isHover, isActive, on, hasKeyboardFocus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Internal_Draw2(
        &mut self,
        position: crate::UnityEngine::Rect,
        content: *mut crate::UnityEngine::GUIContent,
        controlID: i32,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_Draw2", (position, content, controlID, on))?;
        Ok(__cordl_ret)
    }
    pub fn Internal_Draw2_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        content: *mut crate::UnityEngine::GUIContent,
        controlID: i32,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_Draw2_Injected", (position, content, controlID, on))?;
        Ok(__cordl_ret)
    }
    pub fn Internal_Draw_Injected(
        &mut self,
        screenRect: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        content: *mut crate::UnityEngine::GUIContent,
        isHover: bool,
        isActive: bool,
        on: bool,
        hasKeyboardFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_Draw_Injected",
                (screenRect, content, isHover, isActive, on, hasKeyboardFocus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn get_fixedHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fixedWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_margin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectOffset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectOffset = __cordl_object
            .invoke("get_margin", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GUIStyleState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GUIStyleState = __cordl_object
            .invoke("get_normal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_padding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectOffset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectOffset = __cordl_object
            .invoke("get_padding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rawName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_rawName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stretchHeight(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_stretchHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stretchWidth(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_stretchWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_name(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rawName(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rawName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_stretchHeight(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stretchHeight", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+GUIStyle")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUIStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
