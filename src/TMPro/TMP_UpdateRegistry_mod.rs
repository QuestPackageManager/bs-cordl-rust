#[cfg(feature = "cordl_class_TMPro+TMP_UpdateRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_UpdateRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LayoutRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        >,
    >,
    pub m_LayoutQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
    pub m_GraphicRebuildQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
        >,
    >,
    pub m_GraphicQueueLookup: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<i32>,
    >,
}
#[cfg(feature = "cordl_class_TMPro+TMP_UpdateRegistry")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::TMP_UpdateRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "TMP_UpdateRegistry";
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
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl std::ops::Deref for crate::TMPro::TMP_UpdateRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl std::ops::DerefMut for crate::TMPro::TMP_UpdateRegistry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_UpdateRegistry")]
impl crate::TMPro::TMP_UpdateRegistry {
    pub fn InternalRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        bool,
                        1usize,
                    >("InternalRegisterCanvasElementForGraphicRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalRegisterCanvasElementForGraphicRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        bool,
                        1usize,
                    >("InternalRegisterCanvasElementForLayoutRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalRegisterCanvasElementForLayoutRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterCanvasElementForGraphicRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InternalUnRegisterCanvasElementForGraphicRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalUnRegisterCanvasElementForGraphicRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalUnRegisterCanvasElementForLayoutRebuild(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InternalUnRegisterCanvasElementForLayoutRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InternalUnRegisterCanvasElementForLayoutRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PerformUpdateForCanvasRendererObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("PerformUpdateForCanvasRendererObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PerformUpdateForCanvasRendererObjects", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PerformUpdateForMeshRendererObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("PerformUpdateForMeshRendererObjects")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PerformUpdateForMeshRendererObjects", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCanvasElementForGraphicRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RegisterCanvasElementForGraphicRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterCanvasElementForGraphicRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterCanvasElementForLayoutRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RegisterCanvasElementForLayoutRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterCanvasElementForLayoutRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (element))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnRegisterCanvasElementForRebuild(
        element: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::ICanvasElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UI::ICanvasElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnRegisterCanvasElementForRebuild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnRegisterCanvasElementForRebuild", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (element))?
        };
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateRegistry>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateRegistry>,
                        0usize,
                    >("get_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_instance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_UpdateRegistry> = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_TMPro+TMP_UpdateRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_UpdateRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
