#[cfg(feature = "UnityEngine+GUILayoutUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutUtility => "UnityEngine"
    ."GUILayoutUtility"
);
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl crate::UnityEngine::GUILayoutUtility {
    #[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
    pub type LayoutCache = crate::UnityEngine::GUILayoutUtility_LayoutCache;
    pub fn Begin(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Begin", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginContainer(
        cache: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginContainer", (cache))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginLayoutArea(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        layoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginLayoutArea", (style, layoutType))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginLayoutGroup(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
        layoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginLayoutGroup", (style, options, layoutType))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginWindow(
        windowID: i32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeginWindow", (windowID, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGUILayoutGroupInstanceOfType(
        LayoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateGUILayoutGroupInstanceOfType", (LayoutType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoGetRect_GUIContent_GUIStyle_Il2CppArray0(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoGetRect", (content, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoGetRect_f32_f32_f32_f32_GUIStyle_Il2CppArray1(
        minWidth: f32,
        maxWidth: f32,
        minHeight: f32,
        maxHeight: f32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DoGetRect",
                (minWidth, maxWidth, minHeight, maxHeight, style, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndLayoutArea() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndLayoutArea", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EndLayoutGroup() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndLayoutGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLayoutCache(
        instanceID: i32,
        isWindow: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutUtility_LayoutCache>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLayoutCache", (instanceID, isWindow))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRect_GUIContent_GUIStyle_Il2CppArray0(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRect", (content, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRect_f32_f32_GUIStyle_Il2CppArray1(
        width: f32,
        height: f32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GUILayoutOption>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRect", (width, height, style, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetWindowRect(
        windowID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetWindowRect", (windowID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetWindowRect_Injected(
        windowID: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetWindowRect_Injected", (windowID, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_MoveWindow(
        windowID: i32,
        r: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_MoveWindow", (windowID, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_MoveWindow_Injected(
        windowID: i32,
        r: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_MoveWindow_Injected", (windowID, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn Layout() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Layout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LayoutFreeGroup(
        toplevel: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LayoutFreeGroup", (toplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn LayoutFromContainer(
        w: f32,
        h: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LayoutFromContainer", (w, h))?;
        Ok(__cordl_ret.into())
    }
    pub fn LayoutFromEditorWindow() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LayoutFromEditorWindow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LayoutSingleGroup(
        i: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LayoutSingleGroup", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSelectedIdList(
        instanceID: i32,
        isWindow: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveSelectedIdList", (instanceID, isWindow))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectIDList(
        instanceID: i32,
        isWindow: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutUtility_LayoutCache>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectIDList", (instanceID, isWindow))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spaceStyle() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_spaceStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unbalancedgroupscount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unbalancedgroupscount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_unbalancedgroupscount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_unbalancedgroupscount", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GUILayoutUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutUtility_LayoutCache {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id_k__BackingField: i32,
    pub topLevel: *mut crate::UnityEngine::GUILayoutGroup,
    pub layoutGroups: *mut crate::UnityEngineInternal::GenericStack,
    pub windows: *mut crate::UnityEngine::GUILayoutGroup,
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GUILayoutUtility_LayoutCache =>
    "UnityEngine"."GUILayoutUtility/LayoutCache"
);
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl std::ops::Deref for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl std::ops::DerefMut for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl crate::UnityEngine::GUILayoutUtility_LayoutCache {
    pub fn New(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (instanceID))?;
        Ok(__cordl_object.into())
    }
    pub fn ResetCursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetCursor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (instanceID))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_id", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
