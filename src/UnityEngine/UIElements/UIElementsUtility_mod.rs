#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIElementsUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "UIElementsUtility";
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
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIElementsUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl crate::UnityEngine::UIElements::UIElementsUtility {
    pub fn BeginContainerGUI(
        cache: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::GUILayoutUtility_LayoutCache,
        >,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        container: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IMGUIContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GUILayoutUtility_LayoutCache,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::IMGUIContainer,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("BeginContainerGUI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginContainerGUI", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cache, evt, container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_Event0(
        systemEvent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::EventBase,
                        >,
                        1usize,
                    >("CreateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = unsafe { cordl_method_info.invoke_unchecked((), (systemEvent))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEvent_EventType1(
        systemEvent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        eventType: crate::UnityEngine::EventType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                            crate::UnityEngine::EventType,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::EventBase,
                        >,
                        2usize,
                    >("CreateEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = unsafe { cordl_method_info.invoke_unchecked((), (systemEvent, eventType))? };
        Ok(__cordl_ret.into())
    }
    pub fn DoDispatch(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseVisualElementPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::BaseVisualElementPanel,
                        >),
                        bool,
                        1usize,
                    >("DoDispatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoDispatch", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (panel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndContainerGUI(
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        layoutSize: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
                            crate::UnityEngine::Rect,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("EndContainerGUI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndContainerGUI", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (evt, layoutSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPanels(
        panels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
            >,
        >,
        contextType: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::UIElements::Panel,
                                    >,
                                >,
                            >,
                            crate::UnityEngine::UIElements::ContextType,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetAllPanels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllPanels", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (panels, contextType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPanelsIterator() -> quest_hook::libil2cpp::Result<
        crate::System::Collections::Generic::Dictionary_2_Enumerator<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::System::Collections::Generic::Dictionary_2_Enumerator<
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Panel,
                            >,
                        >,
                        0usize,
                    >("GetPanelsIterator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPanelsIterator", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Collections::Generic::Dictionary_2_Enumerator<
            i32,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseMenuName(
        menuName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("ParseMenuName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseMenuName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (menuName))? };
        Ok(__cordl_ret.into())
    }
    pub fn PixelsPerUnitScaleForElement(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::VisualElement,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        ),
                        f32,
                        2usize,
                    >("PixelsPerUnitScaleForElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PixelsPerUnitScaleForElement", 2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (ve, sprite))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCachedPanel(
        instanceID: i32,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::Panel,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RegisterCachedPanel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterCachedPanel", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (instanceID, panel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCachedPanel(
        instanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RemoveCachedPanel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveCachedPanel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (instanceID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPanel(
        instanceID: i32,
        panel: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Panel>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::UIElements::Panel,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >("TryGetPanel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetPanel", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (instanceID, panel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_CleanupRoots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("UnityEngine.UIElements.IUIElementsUtility.CleanupRoots")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IUIElementsUtility.CleanupRoots",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_EndContainerGUIFromException(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                        bool,
                        1usize,
                    >(
                        "UnityEngine.UIElements.IUIElementsUtility.EndContainerGUIFromException",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IUIElementsUtility.EndContainerGUIFromException",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_MakeCurrentIMGUIContainerDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >(
                        "UnityEngine.UIElements.IUIElementsUtility.MakeCurrentIMGUIContainerDirty",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IUIElementsUtility.MakeCurrentIMGUIContainerDirty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_ProcessEvent(
        &mut self,
        instanceID: i32,
        nativeEventPtr: crate::System::IntPtr,
        eventHandled: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        bool,
                        3usize,
                    >("UnityEngine.UIElements.IUIElementsUtility.ProcessEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IUIElementsUtility.ProcessEvent",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instanceID, nativeEventPtr, eventHandled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_ReleaseCapture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("UnityEngine.UIElements.IUIElementsUtility.ReleaseCapture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IUIElementsUtility.ReleaseCapture",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IUIElementsUtility_TakeCapture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >("UnityEngine.UIElements.IUIElementsUtility.TakeCapture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.UIElements.IUIElementsUtility.TakeCapture",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl AsRef<crate::UnityEngine::UIElements::IUIElementsUtility>
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IUIElementsUtility {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsUtility")]
impl AsMut<crate::UnityEngine::UIElements::IUIElementsUtility>
for crate::UnityEngine::UIElements::UIElementsUtility {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IUIElementsUtility {
        unsafe { std::mem::transmute(self) }
    }
}
