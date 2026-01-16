#[cfg(feature = "cordl_class_UnityEngine+Rendering+IUnsafeCommandBuffer")]
#[derive(Debug)]
#[repr(C)]
pub struct IUnsafeCommandBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+IUnsafeCommandBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "IUnsafeCommandBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier11(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (colors, depth))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Il2CppArray_RenderTargetIdentifier_i32_CubemapFace_i32_12(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (colors, depth, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetBinding14(
        &mut self,
        binding: crate::UnityEngine::Rendering::RenderTargetBinding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetBinding),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (binding))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetBinding_i32_CubemapFace_i32_13(
        &mut self,
        binding: crate::UnityEngine::Rendering::RenderTargetBinding,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetBinding,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (binding, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier0(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction1(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, loadAction, storeAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderBufferLoadAction_RenderBufferStoreAction2(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    rt,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction10(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    ), quest_hook::libil2cpp::Void, 6usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    color,
                    colorLoadAction,
                    colorStoreAction,
                    depth,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier6(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (color, depth))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_7(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (color, depth, mipLevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_CubemapFace8(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (color, depth, mipLevel, cubemapFace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_RenderTargetIdentifier_i32_CubemapFace_i32_9(
        &mut self,
        color: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        depth: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (color, depth, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_i32_3(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRenderTarget", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, mipLevel))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_i32_CubemapFace4(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rt, mipLevel, cubemapFace))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_RenderTargetIdentifier_i32_CubemapFace_i32_5(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mipLevel: i32,
        cubemapFace: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        i32,
                        crate::UnityEngine::CubemapFace,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rt, mipLevel, cubemapFace, depthSlice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IBaseCommandBuffer>
    for crate::UnityEngine::Rendering::IUnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IBaseCommandBuffer>
    for crate::UnityEngine::Rendering::IUnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IComputeCommandBuffer>
    for crate::UnityEngine::Rendering::IUnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IComputeCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IComputeCommandBuffer>
    for crate::UnityEngine::Rendering::IUnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IComputeCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IRasterCommandBuffer>
    for crate::UnityEngine::Rendering::IUnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IRasterCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IUnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IRasterCommandBuffer>
    for crate::UnityEngine::Rendering::IUnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IRasterCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
