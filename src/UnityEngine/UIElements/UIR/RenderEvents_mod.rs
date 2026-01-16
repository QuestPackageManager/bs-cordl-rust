#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderEvents")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::UIR::RenderEvents {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "RenderEvents";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderEvents")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::RenderEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderEvents")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::RenderEvents {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+RenderEvents")]
impl crate::UnityEngine::UIElements::UIR::RenderEvents {
    pub fn DepthFirstOnChildAdded(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        index: i32,
        resetState: bool,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        i32,
                        bool,
                    ), u32, 5usize>("DepthFirstOnChildAdded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DepthFirstOnChildAdded",
                            5usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked((), (renderChain, parent, ve, index, resetState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnChildRemoving(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                    ), u32, 2usize>("DepthFirstOnChildRemoving")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DepthFirstOnChildRemoving",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnClippingChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        hierarchical: bool,
        isRootOfChange: bool,
        isPendingHierarchicalRepaint: bool,
        inheritedClipRectIDChanged: bool,
        inheritedMaskingChanged: bool,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::UIRenderDevice>,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        bool,
                        bool,
                        bool,
                        bool,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "DepthFirstOnClippingChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DepthFirstOnClippingChanged",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    renderChain,
                    parent,
                    ve,
                    dirtyID,
                    hierarchical,
                    isRootOfChange,
                    isPendingHierarchicalRepaint,
                    inheritedClipRectIDChanged,
                    inheritedMaskingChanged,
                    device,
                    stats,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnOpacityChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        parentCompositeOpacity: f32,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        hierarchical: bool,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
        isDoingFullVertexRegeneration: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        f32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DepthFirstOnOpacityChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DepthFirstOnOpacityChanged",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    renderChain,
                    parentCompositeOpacity,
                    ve,
                    dirtyID,
                    hierarchical,
                    stats,
                    isDoingFullVertexRegeneration,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DepthFirstOnTransformOrSizeChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::UIRenderDevice>,
        isAncestorOfChangeSkinned: bool,
        transformChanged: bool,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                        >,
                        bool,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "DepthFirstOnTransformOrSizeChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DepthFirstOnTransformOrSizeChanged",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    renderChain,
                    parent,
                    ve,
                    dirtyID,
                    device,
                    isAncestorOfChangeSkinned,
                    transformChanged,
                    stats,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DetermineSelfClipMethod(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::ClipMethod> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                    ), crate::UnityEngine::UIElements::UIR::ClipMethod, 2usize>(
                        "DetermineSelfClipMethod",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DetermineSelfClipMethod",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::ClipMethod =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetClipRectIDClipInfo(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        crate::UnityEngine::Vector4,
                        1usize,
                    >("GetClipRectIDClipInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetClipRectIDClipInfo", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 =
            unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLastDeepestChild(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >,
                        1usize,
                    >("GetLastDeepestChild")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLastDeepestChild", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement> =
            unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformIDTransformInfo(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        crate::UnityEngine::Matrix4x4,
                        1usize,
                    >("GetTransformIDTransformInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTransformIDTransformInfo", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 =
            unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitColorIDs(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                    ), bool, 2usize>("InitColorIDs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitColorIDs",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsColorID(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        bool,
                        1usize,
                    >("NeedsColorID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsColorID", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsTextCoreSettings(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        bool,
                        1usize,
                    >("NeedsTextCoreSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsTextCoreSettings", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsTransformID(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        bool,
                        1usize,
                    >("NeedsTransformID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsTransformID", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn NudgeVerticesToNewSpace(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::UIRenderDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                        >,
                    ), bool, 3usize>("NudgeVerticesToNewSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NudgeVerticesToNewSpace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (ve, renderChain, device))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnColorChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("OnColorChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnColorChanged",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve, dirtyID, stats))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareNudgeVertices(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::UIRenderDevice>,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        src: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        dst: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        count: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::UIRenderDevice,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "PrepareNudgeVertices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareNudgeVertices",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ve, device, mesh, src, dst, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnClippingChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessOnClippingChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessOnClippingChanged",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve, dirtyID, stats))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnColorChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessOnColorChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessOnColorChanged",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve, dirtyID, stats))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnOpacityChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessOnOpacityChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessOnOpacityChanged",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve, dirtyID, stats))? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessOnTransformOrSizeChanged(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        dirtyID: u32,
        stats: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::ChainBuilderStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "ProcessOnTransformOrSizeChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessOnTransformOrSizeChanged",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve, dirtyID, stats))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetColorValues(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetColorValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetColorValues",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLocalFlipsWinding(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        bool,
                        1usize,
                    >("UpdateLocalFlipsWinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateLocalFlipsWinding", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTextCoreSettings(
        renderChain: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChain>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                    ), bool, 2usize>("UpdateTextCoreSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateTextCoreSettings",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (renderChain, ve))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateZeroScaling(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::VisualElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateZeroScaling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateZeroScaling", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ve))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+RenderEvents")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIR::RenderEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
