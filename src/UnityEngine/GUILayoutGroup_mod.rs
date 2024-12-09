#[cfg(feature = "UnityEngine+GUILayoutGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutGroup {
    __cordl_parent: crate::UnityEngine::GUILayoutEntry,
    pub entries: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::GUILayoutEntry,
    >,
    pub isVertical: bool,
    pub resetCoords: bool,
    pub spacing: f32,
    pub sameSize: bool,
    pub isWindow: bool,
    pub windowID: i32,
    pub m_Cursor: i32,
    pub m_StretchableCountX: i32,
    pub m_StretchableCountY: i32,
    pub m_UserSpecifiedWidth: bool,
    pub m_UserSpecifiedHeight: bool,
    pub m_ChildMinWidth: f32,
    pub m_ChildMaxWidth: f32,
    pub m_ChildMinHeight: f32,
    pub m_ChildMaxHeight: f32,
    pub m_MarginLeft: i32,
    pub m_MarginRight: i32,
    pub m_MarginTop: i32,
    pub m_MarginBottom: i32,
}
#[cfg(feature = "UnityEngine+GUILayoutGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutGroup => "UnityEngine"
    ."GUILayoutGroup"
);
#[cfg(feature = "UnityEngine+GUILayoutGroup")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutGroup {
    type Target = crate::UnityEngine::GUILayoutEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutGroup")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutGroup")]
impl crate::UnityEngine::GUILayoutGroup {
    pub fn ApplyOptions(
        &mut self,
        options: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GUILayoutOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyOptions", (options))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: *mut crate::UnityEngine::GUIStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
        Ok(__cordl_ret)
    }
    pub fn CalcHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalcWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalcWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ResetCursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetCursor", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetHorizontal(
        &mut self,
        x: f32,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHorizontal", (x, width))?;
        Ok(__cordl_ret)
    }
    pub fn SetVertical(
        &mut self,
        y: f32,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertical", (y, height))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
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
    pub fn get_marginBottom(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginBottom", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_marginLeft(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginLeft", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_marginRight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginRight", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_marginTop(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_marginTop", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+GUILayoutGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayoutGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
