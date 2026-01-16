#[cfg(feature = "cordl_class_UnityEngine+Rendering+IComputeCommandBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct IComputeCommandBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+IComputeCommandBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::IComputeCommandBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "IComputeCommandBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+IComputeCommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::IComputeCommandBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IComputeCommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::IComputeCommandBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IComputeCommandBuffer")]
impl crate::UnityEngine::Rendering::IComputeCommandBuffer {
    pub fn BuildRayTracingAccelerationStructure_RayTracingAccelerationStructure0(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BuildRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildRayTracingAccelerationStructure", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (accelerationStructure))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRayTracingAccelerationStructure_Vector3_1(
        &mut self,
        accelerationStructure: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
        >,
        relativeOrigin: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                            >,
                            crate::UnityEngine::Vector3,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("BuildRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildRayTracingAccelerationStructure", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (accelerationStructure, relativeOrigin))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_ComputeBuffer_ComputeBuffer0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyCounterValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_ComputeBuffer_GraphicsBuffer2(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyCounterValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_GraphicsBuffer_ComputeBuffer1(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyCounterValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyCounterValue_GraphicsBuffer_GraphicsBuffer3(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        dstOffsetBytes: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CopyCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyCounterValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, dst, dstOffsetBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchCompute_ComputeBuffer_u32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        indirectBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        argsOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchCompute", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchCompute", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DispatchCompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchCompute", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            u32,
                            u32,
                            u32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DispatchRays")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DispatchRays", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (rayTracingShader, rayGenName, width, height, depth, camera),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferCounterValue_ComputeBuffer0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferCounterValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, counterValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferCounterValue_GraphicsBuffer1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
        counterValue: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferCounterValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferCounterValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, counterValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBufferData_ComputeBuffer_Array0(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, data))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, data))?
        };
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            crate::Unity::Collections::NativeArray_1<T>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, data))?
        };
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            crate::Unity::Collections::NativeArray_1<T>,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, data))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Array>,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, data))?
        };
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::Unity::Collections::NativeArray_1<T>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buffer, data))?
        };
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::Unity::Collections::NativeArray_1<T>,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetBufferData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetBufferData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeBufferParam", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, buffer))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeBufferParam", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, buffer))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::GraphicsBufferHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeBufferParam", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, kernelIndex, name, bufferHandle),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeBufferParam", 4usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeBufferParam", 4usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            crate::UnityEngine::GraphicsBufferHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeBufferParam", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, kernelIndex, nameID, bufferHandle),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetComputeConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, name, buffer, offset, _cordl_size),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetComputeConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, name, buffer, offset, _cordl_size),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetComputeConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, nameID, buffer, offset, _cordl_size),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetComputeConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, nameID, buffer, offset, _cordl_size),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeFloatParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeFloatParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeFloatParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeFloatParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParams_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeFloatParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeFloatParams", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeFloatParams_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeFloatParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeFloatParams", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeIntParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeIntParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParams_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeIntParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeIntParams", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeIntParams_i32_1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeIntParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeIntParams", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeMatrixArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeMatrixArrayParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeMatrixArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeMatrixArrayParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeMatrixParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeMatrixParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeMatrixParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeMatrixParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeMatrixParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeTextureParam_Il2CppString0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        kernelIndex: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        rt: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeTextureParam", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, name, rt))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetComputeTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeTextureParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, kernelIndex, name, rt, mipLevel),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            i32,
                            crate::UnityEngine::Rendering::RenderTextureSubElement,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetComputeTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeTextureParam", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetComputeTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeTextureParam", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (computeShader, kernelIndex, nameID, rt))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetComputeTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeTextureParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, kernelIndex, nameID, rt, mipLevel),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            i32,
                            crate::UnityEngine::Rendering::RenderTextureSubElement,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetComputeTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeTextureParam", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeVectorArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeVectorArrayParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeVectorArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeVectorArrayParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, values))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_Il2CppString1(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeVectorParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeVectorParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, name, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetComputeVectorParam_i32_0(
        &mut self,
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        nameID: i32,
        val: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetComputeVectorParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetComputeVectorParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (computeShader, nameID, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingAccelerationStructure", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, kernelIndex, name, rayTracingAccelerationStructure),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingAccelerationStructure", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (computeShader, kernelIndex, nameID, rayTracingAccelerationStructure),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingAccelerationStructure", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingAccelerationStructure,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingAccelerationStructure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingAccelerationStructure", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingBufferParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, buffer))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingBufferParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, buffer))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::GraphicsBufferHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingBufferParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, name, bufferHandle))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingBufferParam", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingBufferParam", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            crate::UnityEngine::GraphicsBufferHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingBufferParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rayTracingShader, nameID, bufferHandle))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRayTracingConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (rayTracingShader, name, buffer, offset, _cordl_size),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRayTracingConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (rayTracingShader, name, buffer, offset, _cordl_size),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRayTracingConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRayTracingConstantBufferParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingConstantBufferParam", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingFloatParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingFloatParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingFloatParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingFloatParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingFloatParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingFloatParams", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingFloatParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingFloatParams", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingIntParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingIntParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingIntParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingIntParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingIntParams", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingIntParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingIntParams", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingMatrixArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingMatrixArrayParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingMatrixArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingMatrixArrayParam", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingMatrixParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingMatrixParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            crate::UnityEngine::Matrix4x4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingMatrixParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingMatrixParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingTextureParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, rt))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingTextureParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingTextureParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, rt))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingVectorArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingVectorArrayParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, values))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Vector4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingVectorArrayParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingVectorArrayParam", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingVectorParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingVectorParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, name, val))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RayTracingShader,
                            >,
                            i32,
                            crate::UnityEngine::Vector4,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SetRayTracingVectorParam")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRayTracingVectorParam", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rayTracingShader, nameID, val))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+IComputeCommandBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::IComputeCommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+IComputeCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IBaseCommandBuffer>
for crate::UnityEngine::Rendering::IComputeCommandBuffer {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IComputeCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IBaseCommandBuffer>
for crate::UnityEngine::Rendering::IComputeCommandBuffer {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
