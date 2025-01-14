#[cfg(feature = "UnityEngine+Yoga+Native")]
#[repr(C)]
#[derive(Debug)]
pub struct Native {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Yoga::Native {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Yoga";
    const CLASS_NAME: &'static str = "Native";
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
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl std::ops::Deref for crate::UnityEngine::Yoga::Native {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::Native {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl crate::UnityEngine::Yoga::Native {
    pub fn YGConfigFree(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGConfigFree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigFree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (config))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigFreeInternal(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGConfigFreeInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigFreeInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (config))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigGetDefault() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::IntPtr,
                0usize,
            >("YGConfigGetDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigGetDefault", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigGetUseWebDefaults(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("YGConfigGetUseWebDefaults")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigGetUseWebDefaults", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (config)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigNew() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::IntPtr, 0usize>("YGConfigNew")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigNew", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigSetPointScaleFactor(
        config: crate::System::IntPtr,
        pixelsInPoint: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGConfigSetPointScaleFactor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigSetPointScaleFactor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (config, pixelsInPoint))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGConfigSetUseWebDefaults(
        config: crate::System::IntPtr,
        useWebDefaults: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGConfigSetUseWebDefaults")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGConfigSetUseWebDefaults", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (config, useWebDefaults))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeBaselineInvoke(
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        height: f32,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
                    f32,
                    f32,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("YGNodeBaselineInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeBaselineInvoke", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, width, height, returnValueAddress))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeCalculateLayout(
        node: crate::System::IntPtr,
        availableWidth: f32,
        availableHeight: f32,
        parentDirection: crate::UnityEngine::Yoga::YogaDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    f32,
                    f32,
                    crate::UnityEngine::Yoga::YogaDirection,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("YGNodeCalculateLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeCalculateLayout", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (node, availableWidth, availableHeight, parentDirection),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeCopyStyle(
        dstNode: crate::System::IntPtr,
        srcNode: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeCopyStyle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeCopyStyle", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (dstNode, srcNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeFree(
        ygNode: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeFree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeFree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ygNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeFreeInternal(
        ygNode: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeFreeInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeFreeInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ygNode))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeGetHasNewLayout(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                bool,
                1usize,
            >("YGNodeGetHasNewLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeGetHasNewLayout", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeInsertChild(
        node: crate::System::IntPtr,
        child: crate::System::IntPtr,
        index: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr, u32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeInsertChild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeInsertChild", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, child, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeIsDirty(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(crate::System::IntPtr), bool, 1usize>("YGNodeIsDirty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeIsDirty", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetBorder(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge),
                f32,
                2usize,
            >("YGNodeLayoutGetBorder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetBorder", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node, edge)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetBottom(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("YGNodeLayoutGetBottom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetBottom", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetHeight(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("YGNodeLayoutGetHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetHeight", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetLeft(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("YGNodeLayoutGetLeft")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetLeft", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetMargin(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge),
                f32,
                2usize,
            >("YGNodeLayoutGetMargin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetMargin", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node, edge)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetPadding(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge),
                f32,
                2usize,
            >("YGNodeLayoutGetPadding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetPadding", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node, edge)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetRight(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("YGNodeLayoutGetRight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetRight", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetTop(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("YGNodeLayoutGetTop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetTop", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeLayoutGetWidth(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                f32,
                1usize,
            >("YGNodeLayoutGetWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeLayoutGetWidth", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeMarkDirty(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeMarkDirty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeMarkDirty", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeMeasureInvoke(
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
        width: f32,
        widthMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
                    f32,
                    crate::UnityEngine::Yoga::YogaMeasureMode,
                    f32,
                    crate::UnityEngine::Yoga::YogaMeasureMode,
                    crate::System::IntPtr,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("YGNodeMeasureInvoke")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeMeasureInvoke", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (node, width, widthMode, height, heightMode, returnValueAddress),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeNewWithConfig(
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::System::IntPtr,
                1usize,
            >("YGNodeNewWithConfig")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeNewWithConfig", 1usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (config))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeRemoveChild(
        node: crate::System::IntPtr,
        child: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeRemoveChild")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeRemoveChild", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, child))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeRemoveMeasureFunc(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeRemoveMeasureFunc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeRemoveMeasureFunc", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeSetConfig(
        ygNode: crate::System::IntPtr,
        config: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeSetConfig")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeSetConfig", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ygNode, config))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeSetHasNewLayout(
        node: crate::System::IntPtr,
        hasNewLayout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeSetHasNewLayout")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeSetHasNewLayout", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, hasNewLayout))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeSetMeasureFunc(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeSetMeasureFunc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeSetMeasureFunc", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleGetDirection(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaDirection> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                crate::UnityEngine::Yoga::YogaDirection,
                1usize,
            >("YGNodeStyleGetDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleGetDirection", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Yoga::YogaDirection = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetAlignContent(
        node: crate::System::IntPtr,
        alignContent: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaAlign),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetAlignContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetAlignContent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, alignContent))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetAlignItems(
        node: crate::System::IntPtr,
        alignItems: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaAlign),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetAlignItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetAlignItems", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, alignItems))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetAlignSelf(
        node: crate::System::IntPtr,
        alignSelf: crate::UnityEngine::Yoga::YogaAlign,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaAlign),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetAlignSelf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetAlignSelf", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, alignSelf))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetBorder(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        border: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetBorder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetBorder", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, border))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetDisplay(
        node: crate::System::IntPtr,
        display: crate::UnityEngine::Yoga::YogaDisplay,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaDisplay),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetDisplay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetDisplay", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, display))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlex(
        node: crate::System::IntPtr,
        flex: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlex", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexBasis(
        node: crate::System::IntPtr,
        flexBasis: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlexBasis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexBasis", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexBasis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexBasisAuto(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeStyleSetFlexBasisAuto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexBasisAuto", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexBasisPercent(
        node: crate::System::IntPtr,
        flexBasis: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlexBasisPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexBasisPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexBasis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexDirection(
        node: crate::System::IntPtr,
        flexDirection: crate::UnityEngine::Yoga::YogaFlexDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaFlexDirection),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlexDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexDirection", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexDirection))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexGrow(
        node: crate::System::IntPtr,
        flexGrow: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlexGrow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexGrow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexGrow))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexShrink(
        node: crate::System::IntPtr,
        flexShrink: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlexShrink")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexShrink", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexShrink))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetFlexWrap(
        node: crate::System::IntPtr,
        flexWrap: crate::UnityEngine::Yoga::YogaWrap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaWrap),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetFlexWrap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetFlexWrap", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexWrap))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetHeight(
        node: crate::System::IntPtr,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetHeight", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, height))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetHeightAuto(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeStyleSetHeightAuto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetHeightAuto", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetHeightPercent(
        node: crate::System::IntPtr,
        height: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetHeightPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetHeightPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, height))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetJustifyContent(
        node: crate::System::IntPtr,
        justifyContent: crate::UnityEngine::Yoga::YogaJustify,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaJustify),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetJustifyContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetJustifyContent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, justifyContent))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMargin(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        margin: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetMargin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMargin", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, margin))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMarginAuto(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMarginAuto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMarginAuto", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMarginPercent(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        margin: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetMarginPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMarginPercent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, margin))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxHeight(
        node: crate::System::IntPtr,
        maxHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMaxHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMaxHeight", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, maxHeight))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxHeightPercent(
        node: crate::System::IntPtr,
        maxHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMaxHeightPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMaxHeightPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, maxHeight))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxWidth(
        node: crate::System::IntPtr,
        maxWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMaxWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMaxWidth", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, maxWidth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMaxWidthPercent(
        node: crate::System::IntPtr,
        maxWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMaxWidthPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMaxWidthPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, maxWidth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinHeight(
        node: crate::System::IntPtr,
        minHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMinHeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMinHeight", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, minHeight))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinHeightPercent(
        node: crate::System::IntPtr,
        minHeight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMinHeightPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMinHeightPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, minHeight))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinWidth(
        node: crate::System::IntPtr,
        minWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMinWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMinWidth", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, minWidth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetMinWidthPercent(
        node: crate::System::IntPtr,
        minWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetMinWidthPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetMinWidthPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, minWidth))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetOverflow(
        node: crate::System::IntPtr,
        flexWrap: crate::UnityEngine::Yoga::YogaOverflow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaOverflow),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetOverflow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetOverflow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, flexWrap))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPadding(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        padding: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetPadding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetPadding", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, padding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPaddingPercent(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        padding: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetPaddingPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetPaddingPercent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, padding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPosition(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        position: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetPosition", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPositionPercent(
        node: crate::System::IntPtr,
        edge: crate::UnityEngine::Yoga::YogaEdge,
        position: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaEdge, f32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("YGNodeStyleSetPositionPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetPositionPercent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, edge, position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetPositionType(
        node: crate::System::IntPtr,
        positionType: crate::UnityEngine::Yoga::YogaPositionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, crate::UnityEngine::Yoga::YogaPositionType),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetPositionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetPositionType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, positionType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetWidth(
        node: crate::System::IntPtr,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetWidth")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetWidth", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, width))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetWidthAuto(
        node: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("YGNodeStyleSetWidthAuto")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetWidthAuto", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGNodeStyleSetWidthPercent(
        node: crate::System::IntPtr,
        width: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGNodeStyleSetWidthPercent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGNodeStyleSetWidthPercent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (node, width))
        };
        Ok(__cordl_ret.into())
    }
    pub fn YGSetManagedObject(
        ygNode: crate::System::IntPtr,
        node: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("YGSetManagedObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "YGSetManagedObject", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ygNode, node))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Yoga+Native")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::Native {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
