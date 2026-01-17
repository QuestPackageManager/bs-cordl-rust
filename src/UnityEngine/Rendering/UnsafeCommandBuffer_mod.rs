#[cfg(feature = "cordl_class_UnityEngine+Rendering+UnsafeCommandBuffer")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct UnsafeCommandBuffer {
    __cordl_parent: crate::UnityEngine::Rendering::BaseCommandBuffer,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+UnsafeCommandBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::UnsafeCommandBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "UnsafeCommandBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::UnsafeCommandBuffer {
    type Target = crate::UnityEngine::Rendering::BaseCommandBuffer;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::UnsafeCommandBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl crate::UnityEngine::Rendering::UnsafeCommandBuffer {
    pub fn BeginSample_CustomSampler1(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Profiling::CustomSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Profiling::CustomSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn BeginSample_ProfilerMarker2(
        &mut self,
        marker: crate::Unity::Profiling::ProfilerMarker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Profiling::ProfilerMarker),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (marker))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRayTracingAccelerationStructure_RayTracingAccelerationStructure0(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "BuildRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildRayTracingAccelerationStructure",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (accelerationStructure))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRayTracingAccelerationStructure_Vector3_1(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
        relativeOrigin: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "BuildRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildRayTracingAccelerationStructure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (accelerationStructure, relativeOrigin))?
        };
        Ok(__cordl_ret.into())
    }
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
    pub fn ClearRenderTarget_RTClearFlags_Color_f32_u32_3(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RTClearFlags,
                        crate::UnityEngine::Color,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (clearFlags, backgroundColor, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget_RTClearFlags_Il2CppArray_f32_u32_4(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        backgroundColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RTClearFlags,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
                        >,
                        f32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearRenderTarget",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (clearFlags, backgroundColors, depth, stencil))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget__cordl_bool__cordl_bool_Color0(
        &mut self,
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (clearDepth, clearColor, backgroundColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget__cordl_bool__cordl_bool_Color_f32_1(
        &mut self,
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, crate::UnityEngine::Color, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (clearDepth, clearColor, backgroundColor, depth))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearRenderTarget__cordl_bool__cordl_bool_Color_f32_u32_2(
        &mut self,
        clearDepth: bool,
        clearColor: bool,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, crate::UnityEngine::Color, f32, u32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ClearRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearRenderTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (clearDepth, clearColor, backgroundColor, depth, stencil),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureFoveatedRendering(
        &mut self,
        platformData: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        "ConfigureFoveatedRendering",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ConfigureFoveatedRendering",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (platformData))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_ComputeBuffer_ComputeBuffer0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_ComputeBuffer_GraphicsBuffer2(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_GraphicsBuffer_ComputeBuffer1(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_GraphicsBuffer_GraphicsBuffer3(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyCounterValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_ByRefMut0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_ComputeShader_ByRefMut2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableKeyword_Material_ByRefMut1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableScissorRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DisableScissorRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableScissorRect",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableShaderKeyword(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DisableShaderKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisableShaderKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_ComputeBuffer_u32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchCompute",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, indirectBuffer, argsOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_GraphicsBuffer_u32_2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchCompute",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, indirectBuffer, argsOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_i32_i32_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        threadGroupsX: i32,
        threadGroupsY: i32,
        threadGroupsZ: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchCompute",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    threadGroupsX,
                    threadGroupsY,
                    threadGroupsZ,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchRays(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        rayGenName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        width: u32,
        height: u32,
        depth: u32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u32,
                        u32,
                        u32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), quest_hook::libil2cpp::Void, 6usize>("DispatchRays")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchRays",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, rayGenName, width, height, depth, camera),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_ComputeBuffer2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_ComputeBuffer_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_ComputeBuffer_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_GraphicsBuffer5(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_GraphicsBuffer_i32_4(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedIndirect_GraphicsBuffer_i32_MaterialPropertyBlock3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawMeshInstancedIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstancedProcedural(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawMeshInstancedProcedural"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstancedProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, count, properties),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_Mesh_i32_Material_i32_Il2CppArray2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstanced",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (mesh, submeshIndex, material, shaderPass, matrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstanced",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, submeshIndex, material, shaderPass, matrices, count),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMeshInstanced",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    submeshIndex,
                    material,
                    shaderPass,
                    matrices,
                    count,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_Mesh_Matrix4x4_Material3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mesh, matrix, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (mesh, matrix, material, submeshIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (mesh, matrix, material, submeshIndex, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMesh",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (mesh, matrix, material, submeshIndex, shaderPass, properties),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMultipleMeshes(
        &mut self,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>>,
        >,
        subsetIndices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        count: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawMultipleMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawMultipleMeshes",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrices,
                    meshes,
                    subsetIndices,
                    count,
                    material,
                    shaderPass,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawOcclusionMesh(
        &mut self,
        normalizedCamViewport: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::RectInt),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawOcclusionMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawOcclusionMesh", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (normalizedCamViewport))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer5(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_4(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_MaterialPropertyBlock3(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer11(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_10(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_MaterialPropertyBlock9(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer2(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (matrix, material, shaderPass, topology, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_1(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_ComputeBuffer_i32_MaterialPropertyBlock0(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer8(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (matrix, material, shaderPass, topology, bufferWithArgs),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_7(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProceduralIndirect_Matrix4x4_Material_i32_MeshTopology_GraphicsBuffer_i32_MaterialPropertyBlock6(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        bufferWithArgs: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        argsOffset: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DrawProceduralIndirect"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProceduralIndirect",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    bufferWithArgs,
                    argsOffset,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_i32_5(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_i32_i32_4(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
        instanceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                    instanceCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_GraphicsBuffer_Matrix4x4_Material_i32_MeshTopology_i32_i32_MaterialPropertyBlock3(
        &mut self,
        indexBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        indexCount: i32,
        instanceCount: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 8usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    indexBuffer,
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    indexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_Matrix4x4_Material_i32_MeshTopology_i32_2(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (matrix, material, shaderPass, topology, vertexCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_Matrix4x4_Material_i32_MeshTopology_i32_i32_1(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
        instanceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    vertexCount,
                    instanceCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawProcedural_Matrix4x4_Material_i32_MeshTopology_i32_i32_MaterialPropertyBlock0(
        &mut self,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        topology: crate::UnityEngine::MeshTopology,
        vertexCount: i32,
        instanceCount: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Matrix4x4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        crate::UnityEngine::MeshTopology,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 7usize>("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawProcedural",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    matrix,
                    material,
                    shaderPass,
                    topology,
                    vertexCount,
                    instanceCount,
                    properties,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRendererList(
        &mut self,
        rendererList: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RendererList),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DrawRendererList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawRendererList", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererList))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_Renderer_Material2(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 2usize>("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderer, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_i32_1(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderer",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderer, material, submeshIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_i32_i32_0(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRenderer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderer, material, submeshIndex, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_ByRefMut0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_ComputeShader_ByRefMut2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableKeyword_Material_ByRefMut1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableScissorRect(
        &mut self,
        scissor: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Rect), quest_hook::libil2cpp::Void, 1usize>(
                        "EnableScissorRect",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableScissorRect",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (scissor))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableShaderKeyword(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EnableShaderKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnableShaderKeyword", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_CustomSampler1(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Profiling::CustomSampler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Profiling::CustomSampler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample_ProfilerMarker2(
        &mut self,
        marker: crate::Unity::Profiling::ProfilerMarker,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Profiling::ProfilerMarker),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (marker))? };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementUpdateCount(
        &mut self,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
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
                    >("IncrementUpdateCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementUpdateCount", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (dest))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnRenderObjectCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "InvokeOnRenderObjectCallbacks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InvokeOnRenderObjectCallbacks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomBlit(
        &mut self,
        callback: crate::System::IntPtr,
        command: u32,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        commandParam: u32,
        commandFlags: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        u32,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        u32,
                        u32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "IssuePluginCustomBlit"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomBlit",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (callback, command, source, dest, commandParam, commandFlags),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginCustomTextureUpdateV2(
        &mut self,
        callback: crate::System::IntPtr,
        targetTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        userData: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "IssuePluginCustomTextureUpdateV2"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IssuePluginCustomTextureUpdateV2",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (callback, targetTexture, userData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEvent(
        &mut self,
        callback: crate::System::IntPtr,
        eventID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("IssuePluginEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEvent", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, eventID))? };
        Ok(__cordl_ret.into())
    }
    pub fn IssuePluginEventAndData(
        &mut self,
        callback: crate::System::IntPtr,
        eventID: i32,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr, i32, crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("IssuePluginEventAndData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IssuePluginEventAndData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callback, eventID, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn MarkLateLatchMatrixShaderPropertyID(
        &mut self,
        matrixPropertyType: crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
        shaderPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "MarkLateLatchMatrixShaderPropertyID"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MarkLateLatchMatrixShaderPropertyID",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (matrixPropertyType, shaderPropertyID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        wrapped: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        executingPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (wrapped, executingPass, isAsync))?;
        Ok(__cordl_object.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_ComputeBuffer_Action_1_0<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_ComputeBuffer_i32_i32_Action_1_1<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, _cordl_size, offset, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_GraphicsBuffer_Action_1_2<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_GraphicsBuffer_i32_i32_Action_1_3<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        _cordl_size: i32,
        offset: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, _cordl_size, offset, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_Action_1_4<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_Action_1_5<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (output, src, mipIndex, callback))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_GraphicsFormat_Action_1_7<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, mipIndex, dstFormat, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_TextureFormat_Action_1_6<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        dstFormat: crate::UnityEngine::TextureFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        crate::UnityEngine::TextureFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (output, src, mipIndex, dstFormat, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_i32_i32_i32_i32_i32_i32_Action_1_8<T>(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 10usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    output, src, mipIndex, x, width, y, height, z, depth, callback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_i32_i32_i32_i32_i32_i32_GraphicsFormat_Action_1_10<
        T,
    >(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        dstFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    output, src, mipIndex, x, width, y, height, z, depth, dstFormat, callback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequestAsyncReadbackIntoNativeArray_Texture_i32_i32_i32_i32_i32_i32_i32_TextureFormat_Action_1_9<
        T,
    >(
        &mut self,
        output: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeArray_1<T>>,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        mipIndex: i32,
        x: i32,
        width: i32,
        y: i32,
        height: i32,
        z: i32,
        depth: i32,
        dstFormat: crate::UnityEngine::TextureFormat,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::Rendering::AsyncGPUReadbackRequest>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<T>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::TextureFormat,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::Rendering::AsyncGPUReadbackRequest,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 11usize>(
                        "RequestAsyncReadbackIntoNativeArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequestAsyncReadbackIntoNativeArray",
                            11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    output, src, mipIndex, x, width, y, height, z, depth, dstFormat, callback,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferCounterValue_ComputeBuffer0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetBufferCounterValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferCounterValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferCounterValue_GraphicsBuffer1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        u32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetBufferCounterValue"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferCounterValue",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, counterValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_Array0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_Array_i32_i32_i32_3(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_List_1_1<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_List_1_i32_i32_i32_4<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_NativeArray_1_2<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_NativeArray_1_i32_i32_i32_5<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_Array6(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_Array_i32_i32_i32_9(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Array>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_List_1_7<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_List_1_i32_i32_i32_10<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        managedBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    managedBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_NativeArray_1_8<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (buffer, data))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_GraphicsBuffer_NativeArray_1_i32_i32_i32_11<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        data: crate::Unity::Collections::NativeArray_1<T>,
        nativeBufferStartIndex: i32,
        graphicsBufferStartIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        crate::Unity::Collections::NativeArray_1<T>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetBufferData",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    buffer,
                    data,
                    nativeBufferStartIndex,
                    graphicsBufferStartIndex,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_Il2CppString_ComputeBuffer1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, name, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_Il2CppString_GraphicsBuffer5(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, name, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_Il2CppString_GraphicsBufferHandle3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_i32_ComputeBuffer0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_i32_GraphicsBuffer4(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeBufferParam_i32_GraphicsBufferHandle2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeBufferParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_Il2CppString_ComputeBuffer1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_Il2CppString_GraphicsBuffer3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_i32_ComputeBuffer0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, nameID, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeConstantBufferParam_i32_GraphicsBuffer2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, nameID, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParams_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParams_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetComputeIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetComputeIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParams_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParams_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixArrayParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixArrayParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, name, rt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString_i32_2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, rt, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString_i32_RenderTextureSubElement4(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        mipLevel: i32,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, name, rt, mipLevel, element),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, kernelIndex, nameID, rt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_i32_i32_3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, rt, mipLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_i32_i32_RenderTextureSubElement5(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        mipLevel: i32,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        i32,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "SetComputeTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeTextureParam",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (computeShader, kernelIndex, nameID, rt, mipLevel, element),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorArrayParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorArrayParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetComputeVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetComputeVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetFoveatedRenderingMode(
        &mut self,
        foveatedRenderingMode: crate::UnityEngine::Rendering::FoveatedRenderingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::FoveatedRenderingMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetFoveatedRenderingMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetFoveatedRenderingMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (foveatedRenderingMode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Il2CppString_ComputeBuffer0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_Il2CppString_GraphicsBuffer2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_ComputeBuffer1(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalBuffer_i32_GraphicsBuffer3(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalBuffer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Color,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalColor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalColor_i32_0(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalColor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_ComputeBuffer_Il2CppString1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, name, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_ComputeBuffer_i32_0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_GraphicsBuffer_Il2CppString3(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, name, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalConstantBuffer_GraphicsBuffer_i32_2(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        nameID: i32,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetGlobalConstantBuffer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalConstantBuffer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, nameID, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalDepthBias(
        &mut self,
        bias: f32,
        slopeBias: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32, f32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalDepthBias",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalDepthBias",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (bias, slopeBias))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Il2CppString_Il2CppArray3(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_Il2CppString_List_1_1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_Il2CppArray0(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloatArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<f32>>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloatArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloatArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat_i32_0(
        &mut self,
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, f32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalFloat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalFloat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInt_i32_0(
        &mut self,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("SetGlobalInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInteger",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalInteger_i32_0(
        &mut self,
        nameID: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalInteger",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalInteger",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Il2CppString_Il2CppArray3(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_Il2CppString_List_1_1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_Il2CppArray0(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrixArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalMatrixArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrixArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalMatrix",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalMatrix_i32_0(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Matrix4x4),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalMatrix", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString_RenderTextureSubElement2(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value, element))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_RenderTextureSubElement3(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderTextureSubElement,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetGlobalTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalTexture",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value, element))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Il2CppString_Il2CppArray3(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_Il2CppString_List_1_1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (propertyName, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_Il2CppArray0(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVectorArray_i32_List_1_2(
        &mut self,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Vector4,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetGlobalVectorArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVectorArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetGlobalVector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetGlobalVector",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (name, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_i32_0(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Vector4),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetGlobalVector")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetGlobalVector", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nameID, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInstanceMultiplier(
        &mut self,
        multiplier: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(
                        "SetInstanceMultiplier",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInstanceMultiplier",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (multiplier))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetInvertCulling(
        &mut self,
        invertCulling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetInvertCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetInvertCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (invertCulling))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_ByRefMut__cordl_bool0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_ComputeShader_ByRefMut__cordl_bool2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword_Material_ByRefMut__cordl_bool1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetLateLatchProjectionMatrices(
        &mut self,
        projectionMat: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "SetLateLatchProjectionMatrices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetLateLatchProjectionMatrices",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (projectionMat))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_ComputeShader_i32_Il2CppString_RayTracingAccelerationStructure2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    name,
                    rayTracingAccelerationStructure,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_ComputeShader_i32_i32_RayTracingAccelerationStructure3(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        nameID: i32,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    computeShader,
                    kernelIndex,
                    nameID,
                    rayTracingAccelerationStructure,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_RayTracingShader_Il2CppString_RayTracingAccelerationStructure0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, name, rayTracingAccelerationStructure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingAccelerationStructure_RayTracingShader_i32_RayTracingAccelerationStructure1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        rayTracingAccelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingAccelerationStructure"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingAccelerationStructure",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, rayTracingAccelerationStructure),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_Il2CppString_ComputeBuffer0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, buffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_Il2CppString_GraphicsBuffer2(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, buffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_Il2CppString_GraphicsBufferHandle4(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_i32_ComputeBuffer1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_i32_GraphicsBuffer3(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingBufferParam_i32_GraphicsBufferHandle5(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        bufferHandle: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::GraphicsBufferHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingBufferParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, bufferHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_Il2CppString_ComputeBuffer1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_Il2CppString_GraphicsBuffer3(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, name, buffer, offset, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_i32_ComputeBuffer0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, buffer, offset, _cordl_size),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingConstantBufferParam_i32_GraphicsBuffer2(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "SetRayTracingConstantBufferParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingConstantBufferParam",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rayTracingShader, nameID, buffer, offset, _cordl_size),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParams_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingFloatParams_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingFloatParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingFloatParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParams_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingIntParams_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingIntParams"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingIntParams",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixArrayParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixArrayParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingMatrixParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Matrix4x4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingMatrixParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingMatrixParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingTextureParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingTextureParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingTextureParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingTextureParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingTextureParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, rt))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorArrayParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorArrayParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorArrayParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorArrayParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorParam_Il2CppString0(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRayTracingVectorParam_i32_1(
        &mut self,
        rayTracingShader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingShader,
        >,
        nameID: i32,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RayTracingShader>,
                        i32,
                        crate::UnityEngine::Vector4,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetRayTracingVectorParam"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetRayTracingVectorParam",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))? };
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
    pub fn SetShadowSamplingMode(
        &mut self,
        shadowmap: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mode: crate::UnityEngine::Rendering::ShadowSamplingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::ShadowSamplingMode,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetShadowSamplingMode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetShadowSamplingMode",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (shadowmap, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetSinglePassStereo(
        &mut self,
        mode: crate::UnityEngine::Rendering::SinglePassStereoMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::SinglePassStereoMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetSinglePassStereo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetSinglePassStereo", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices(
        &mut self,
        view: crate::UnityEngine::Matrix4x4,
        proj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetViewProjectionMatrices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetViewProjectionMatrices", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (view, proj))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewport(
        &mut self,
        pixelRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Rect), quest_hook::libil2cpp::Void, 1usize>(
                        "SetViewport",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewport",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pixelRect))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetWireframe(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetWireframe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetWireframe",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (enable))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupCameraProperties(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetupCameraProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupCameraProperties", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_DisableKeyword_ByRefMut0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.Rendering.IBaseCommandBuffer.DisableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.DisableKeyword",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_DisableKeyword_ComputeShader_ByRefMut2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.DisableKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.DisableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_DisableKeyword_Material_ByRefMut1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.DisableKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.DisableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_EnableKeyword_ByRefMut0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnityEngine.Rendering.IBaseCommandBuffer.EnableKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.EnableKeyword",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_EnableKeyword_ComputeShader_ByRefMut2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.EnableKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.EnableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_EnableKeyword_Material_ByRefMut1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.EnableKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.EnableKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_SetKeyword_ByRefMut__cordl_bool0(
        &mut self,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::GlobalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GlobalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.SetKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.SetKeyword",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_SetKeyword_ComputeShader_ByRefMut__cordl_bool2(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.SetKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.SetKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (computeShader, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_Rendering_IBaseCommandBuffer_SetKeyword_Material_ByRefMut__cordl_bool1(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        keyword: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::LocalKeyword>,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::LocalKeyword,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UnityEngine.Rendering.IBaseCommandBuffer.SetKeyword",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnityEngine.Rendering.IBaseCommandBuffer.SetKeyword",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, keyword, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnmarkLateLatchMatrix(
        &mut self,
        matrixPropertyType: crate::UnityEngine::Rendering::CameraLateLatchMatrixType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::CameraLateLatchMatrixType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UnmarkLateLatchMatrix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnmarkLateLatchMatrix", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (matrixPropertyType))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        wrapped: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        executingPass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (wrapped, executingPass, isAsync))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+UnsafeCommandBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::UnsafeCommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IBaseCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IBaseCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IComputeCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IComputeCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IComputeCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IComputeCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IRasterCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IRasterCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IRasterCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IRasterCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IUnsafeCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+UnsafeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IUnsafeCommandBuffer>
    for crate::UnityEngine::Rendering::UnsafeCommandBuffer
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IUnsafeCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
