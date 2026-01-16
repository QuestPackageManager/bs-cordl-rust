#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutManager")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Index: i32,
    pub m_Nodes: crate::UnityEngine::UIElements::Layout::LayoutDataStore,
    pub m_Configs: crate::UnityEngine::UIElements::Layout::LayoutDataStore,
    pub m_SyncRoot: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_NodesToFree: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<
            crate::UnityEngine::UIElements::Layout::LayoutHandle,
        >,
    >,
    pub m_DefaultConfig: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    pub m_ManagedMeasureFunctions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Layout::ManagedObjectStore_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
            >,
        >,
    >,
    pub m_ManagedBaselineFunctions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Layout::ManagedObjectStore_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction,
            >,
        >,
    >,
    pub m_ManagedOwners: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Layout::ManagedObjectStore_1<
            quest_hook::libil2cpp::Gc<
                crate::System::WeakReference_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                >,
            >,
        >,
    >,
    pub m_HighMark: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::Layout::LayoutManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.Layout";
    const CLASS_NAME: &'static str = "LayoutManager";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Layout::LayoutManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Layout::LayoutManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutManager")]
impl crate::UnityEngine::UIElements::Layout::LayoutManager {
    pub fn CreateConfig(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Layout::LayoutConfig> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::Layout::LayoutConfig,
                        0usize,
                    >("CreateConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateConfig", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Layout::LayoutConfig =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Layout::LayoutNode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::UIElements::Layout::LayoutNode, 0usize>(
                        "CreateNode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateNode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Layout::LayoutNode =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateNodeInternal(
        &mut self,
        configHandle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Layout::LayoutNode> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        crate::UnityEngine::UIElements::Layout::LayoutNode,
                        1usize,
                    >("CreateNodeInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateNodeInternal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Layout::LayoutNode =
            unsafe { cordl_method_info.invoke_unchecked(self, (configHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyConfig(
        &mut self,
        config: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::Layout::LayoutConfig,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::Layout::LayoutConfig,
                    >), quest_hook::libil2cpp::Void, 1usize>("DestroyConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DestroyConfig",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (config))? };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyNode(
        &mut self,
        node: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Layout::LayoutNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::Layout::LayoutNode,
                    >), quest_hook::libil2cpp::Void, 1usize>("DestroyNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DestroyNode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (node))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FreeNode(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeNode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAccess(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Layout::LayoutDataAccess>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::Layout::LayoutDataAccess,
                        0usize,
                    >("GetAccess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAccess", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Layout::LayoutDataAccess =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBaselineFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction,
                        >,
                        1usize,
                    >("GetBaselineFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBaselineFunction", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutBaselineFunction,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultConfig(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Layout::LayoutConfig> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::Layout::LayoutConfig,
                        0usize,
                    >("GetDefaultConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultConfig", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Layout::LayoutConfig =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetManager(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Layout::LayoutManager>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutManager,
                        >,
                        1usize,
                    >("GetManager")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetManager", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutManager,
        > = unsafe { cordl_method_info.invoke_unchecked((), (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMeasureFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
                        >,
                        1usize,
                    >("GetMeasureFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMeasureFunction", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOwner(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::UIElements::Layout::LayoutHandle),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >,
                        1usize,
                    >("GetOwner")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOwner", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement> =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (allocator))?;
        Ok(__cordl_object.into())
    }
    pub fn SetMeasureFunction(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::Layout::LayoutHandle,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutMeasureFunction,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetMeasureFunction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMeasureFunction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetOwner(
        &mut self,
        handle: crate::UnityEngine::UIElements::Layout::LayoutHandle,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::Layout::LayoutHandle,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetOwner")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetOwner",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (handle, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Shutdown")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Shutdown",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryFreeNodes(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("TryFreeNodes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryFreeNodes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::Allocator),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SharedManager() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Layout::LayoutManager>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::Layout::LayoutManager,
                        >,
                        0usize,
                    >("get_SharedManager")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SharedManager", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Layout::LayoutManager,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+Layout+LayoutManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Layout::LayoutManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutManager")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::UIElements::Layout::LayoutManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Layout+LayoutManager")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::UIElements::Layout::LayoutManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
