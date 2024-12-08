#[cfg(feature = "UnityEngine+GUIScrollGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct GUIScrollGroup {
    __cordl_parent: crate::UnityEngine::GUILayoutGroup,
    pub calcMinWidth: f32,
    pub calcMaxWidth: f32,
    pub calcMinHeight: f32,
    pub calcMaxHeight: f32,
    pub clientWidth: f32,
    pub clientHeight: f32,
    pub allowHorizontalScroll: bool,
    pub allowVerticalScroll: bool,
    pub needsHorizontalScrollbar: bool,
    pub needsVerticalScrollbar: bool,
    pub horizontalScrollbar: *mut crate::UnityEngine::GUIStyle,
    pub verticalScrollbar: *mut crate::UnityEngine::GUIStyle,
}
#[cfg(feature = "UnityEngine+GUIScrollGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUIScrollGroup => "UnityEngine"
    ."GUIScrollGroup"
);
#[cfg(feature = "UnityEngine+GUIScrollGroup")]
impl std::ops::Deref for crate::UnityEngine::GUIScrollGroup {
    type Target = crate::UnityEngine::GUILayoutGroup;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIScrollGroup")]
impl std::ops::DerefMut for crate::UnityEngine::GUIScrollGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUIScrollGroup")]
impl crate::UnityEngine::GUIScrollGroup {
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
#[cfg(feature = "UnityEngine+GUIScrollGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUIScrollGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
