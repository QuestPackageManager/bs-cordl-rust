#[cfg(feature = "UnityEngine+GUILayoutUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GUILayoutUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+GUILayoutUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::GUILayoutUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "GUILayoutUtility";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Begin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Begin", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (instanceID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginContainer(
        cache: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::GUILayoutUtility_LayoutCache,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BeginContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginContainer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (cache))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginLayoutArea(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        layoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
                2usize,
            >("BeginLayoutArea")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginLayoutArea", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup> = unsafe {
            method.invoke_unchecked((), (style, layoutType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginLayoutGroup(
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
        layoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
                3usize,
            >("BeginLayoutGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginLayoutGroup", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup> = unsafe {
            method.invoke_unchecked((), (style, options, layoutType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn BeginWindow(
        windowID: i32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("BeginWindow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BeginWindow", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (windowID, style, options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateGUILayoutGroupInstanceOfType(
        LayoutType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
                1usize,
            >("CreateGUILayoutGroupInstanceOfType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateGUILayoutGroupInstanceOfType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup> = unsafe {
            method.invoke_unchecked((), (LayoutType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoGetRect_GUIContent_GUIStyle_Il2CppArray0(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                ),
                crate::UnityEngine::Rect,
                3usize,
            >("DoGetRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoGetRect", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked((), (content, style, options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoGetRect_f32_f32_f32_f32_GUIStyle_Il2CppArray1(
        minWidth: f32,
        maxWidth: f32,
        minHeight: f32,
        maxHeight: f32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    f32,
                    f32,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                ),
                crate::UnityEngine::Rect,
                6usize,
            >("DoGetRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoGetRect", 6usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (minWidth, maxWidth, minHeight, maxHeight, style, options),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndLayoutArea() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("EndLayoutArea")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndLayoutArea", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndLayoutGroup() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("EndLayoutGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EndLayoutGroup", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLayoutCache(
        instanceID: i32,
        isWindow: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutUtility_LayoutCache>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, bool),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::GUILayoutUtility_LayoutCache,
                >,
                2usize,
            >("GetLayoutCache")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLayoutCache", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        > = unsafe { method.invoke_unchecked((), (instanceID, isWindow)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRect_GUIContent_GUIStyle_Il2CppArray0(
        content: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIContent>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                ),
                crate::UnityEngine::Rect,
                3usize,
            >("GetRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRect", 3usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked((), (content, style, options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRect_f32_f32_GUIStyle_Il2CppArray1(
        width: f32,
        height: f32,
        style: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
        options: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutOption>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    f32,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutOption,
                            >,
                        >,
                    >,
                ),
                crate::UnityEngine::Rect,
                4usize,
            >("GetRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRect", 4usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked((), (width, height, style, options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetWindowRect(
        windowID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::UnityEngine::Rect,
                1usize,
            >("Internal_GetWindowRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_GetWindowRect", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rect = unsafe {
            method.invoke_unchecked((), (windowID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetWindowRect_Injected(
        windowID: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Internal_GetWindowRect_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_GetWindowRect_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (windowID, ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_MoveWindow(
        windowID: i32,
        r: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::UnityEngine::Rect),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Internal_MoveWindow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_MoveWindow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (windowID, r))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_MoveWindow_Injected(
        windowID: i32,
        r: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Internal_MoveWindow_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_MoveWindow_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (windowID, r))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Layout() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Layout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Layout", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn LayoutFreeGroup(
        toplevel: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LayoutFreeGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LayoutFreeGroup", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (toplevel))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LayoutFromContainer(
        w: f32,
        h: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LayoutFromContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LayoutFromContainer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (w, h))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LayoutFromEditorWindow() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("LayoutFromEditorWindow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LayoutFromEditorWindow", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn LayoutSingleGroup(
        i: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LayoutSingleGroup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LayoutSingleGroup", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (i))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSelectedIdList(
        instanceID: i32,
        isWindow: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RemoveSelectedIdList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveSelectedIdList", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (instanceID, isWindow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectIDList(
        instanceID: i32,
        isWindow: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutUtility_LayoutCache>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, bool),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::GUILayoutUtility_LayoutCache,
                >,
                2usize,
            >("SelectIDList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SelectIDList", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        > = unsafe { method.invoke_unchecked((), (instanceID, isWindow)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_spaceStyle() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle>,
                0usize,
            >("get_spaceStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_spaceStyle", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUIStyle> = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_unbalancedgroupscount() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("get_unbalancedgroupscount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_unbalancedgroupscount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_unbalancedgroupscount(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_unbalancedgroupscount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_unbalancedgroupscount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))
        };
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
    pub topLevel: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
    pub layoutGroups: quest_hook::libil2cpp::Gc<
        crate::UnityEngineInternal::GenericStack,
    >,
    pub windows: quest_hook::libil2cpp::Gc<crate::UnityEngine::GUILayoutGroup>,
}
#[cfg(feature = "UnityEngine+GUILayoutUtility+LayoutCache")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::GUILayoutUtility_LayoutCache {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "GUILayoutUtility/LayoutCache";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetCursor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResetCursor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (instanceID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_id")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_id", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
