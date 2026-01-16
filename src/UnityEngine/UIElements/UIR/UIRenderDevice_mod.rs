#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRenderDevice {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_MockDevice: bool,
    pub m_DefaultStencilState: crate::System::IntPtr,
    pub m_VertexDecl: crate::System::IntPtr,
    pub m_FirstPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub m_NextPageVertexCount: u32,
    pub m_LargeMeshVertexCount: u32,
    pub m_IndexToVertexCountRatio: f32,
    pub m_DeferredFrees: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree,
                >,
            >,
        >,
    >,
    pub m_Updates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
                >,
            >,
        >,
    >,
    pub m_CommandLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
                >,
            >,
        >,
    >,
    pub m_Fences: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub m_ConstantProps: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    pub m_BatchProps: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    pub m_FrameIndex: u32,
    pub m_NextUpdateID: u32,
    pub m_DrawStats: crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics,
    pub m_MeshHandles: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::LinkedPool_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        >,
    >,
    pub m_DrawParams: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::DrawParams>,
    pub m_TextureSlotManager:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::TextureSlotManager>,
    pub _breakBatches_k__BackingField: bool,
    pub _isFlat_k__BackingField: bool,
    pub _drawsInCameras_k__BackingField: bool,
    pub currentFrameCommandListCount: i32,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRenderDevice";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
    pub type AllocToFree = crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
    pub type AllocToUpdate = crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
    pub type DeviceToFree = crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
    pub type DrawStatistics = crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics;
    #[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
    pub type EvaluationState = crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState;
    pub fn ActiveUpdatesForMeshHandle(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::MeshHandle,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
                            >,
                        >,
                        1usize,
                    >("ActiveUpdatesForMeshHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ActiveUpdatesForMeshHandle", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (mesh))? };
        Ok(__cordl_ret.into())
    }
    pub fn AdvanceFrame(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("AdvanceFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdvanceFrame",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Allocate_MeshHandle_u32__cordl_bool1(
        &mut self,
        meshHandle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeSlice_1<u16>>,
        shortLived: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
                        u32,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<
                                crate::UnityEngine::UIElements::Vertex,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<u16>,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Allocate",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    meshHandle,
                    vertexCount,
                    indexCount,
                    vertexData,
                    indexData,
                    shortLived,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Allocate_u32_ByRefMut0(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeSlice_1<u16>>,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeSlice_1<
                                    crate::UnityEngine::UIElements::Vertex,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeSlice_1<u16>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u16>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::MeshHandle,
                        >,
                        5usize,
                    >("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Allocate", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::MeshHandle,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (vertexCount, indexCount, vertexData, indexData, indexOffset),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyBatchState(
        &mut self,
        st: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState,
                    >), quest_hook::libil2cpp::Void, 1usize>("ApplyBatchState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ApplyBatchState",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (st))? };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyDrawCommandState(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
        textureSlot: i32,
        newMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        newMatDiffers: bool,
        st: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                        >,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "ApplyDrawCommandState"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ApplyDrawCommandState",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (cmd, textureSlot, newMat, newMatDiffers, st))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompleteCreation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CompleteCreation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompleteCreation",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (disposing))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRanges(
        &mut self,
        ib: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<u16>,
        >,
        vb: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<
                crate::UnityEngine::UIElements::Vertex,
            >,
        >,
        ranges: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::DrawBufferRange,
        >,
        commandList: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<u16>,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::Utility_GPUBuffer_1<
                                crate::UnityEngine::UIElements::Vertex,
                            >,
                        >,
                        crate::Unity::Collections::NativeSlice_1<
                            crate::UnityEngine::UIElements::UIR::DrawBufferRange,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawRanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRanges",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ib, vb, ranges, commandList))? };
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateChain(
        &mut self,
        head: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::RenderChainCommand>,
        initialMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        defaultMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        gradientSettings: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        shaderInfo: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        pixelsPerPoint: f32,
        immediateException: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Exception>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>("EvaluateChain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EvaluateChain",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    head,
                    initialMat,
                    defaultMat,
                    gradientSettings,
                    shaderInfo,
                    pixelsPerPoint,
                    immediateException,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlushAllPendingDeviceDisposes(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "FlushAllPendingDeviceDisposes",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FlushAllPendingDeviceDisposes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Free(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::UIR::MeshHandle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Free")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Free",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mesh))? };
        Ok(__cordl_ret.into())
    }
    pub fn GatherDrawStatistics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics,
                        0usize,
                    >("GatherDrawStatistics")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GatherDrawStatistics", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InitVertexDeclaration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InitVertexDeclaration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitVertexDeclaration",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn KickRanges(
        &mut self,
        ranges: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rangesReady: quest_hook::libil2cpp::ByRefMut<i32>,
        rangesStart: quest_hook::libil2cpp::ByRefMut<i32>,
        rangesCount: i32,
        curPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
        commandList: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
                    ), quest_hook::libil2cpp::Void, 6usize>("KickRanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "KickRanges",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    ranges,
                    rangesReady,
                    rangesStart,
                    rangesCount,
                    curPage,
                    commandList,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New__cordl_bool1(
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
        mockDevice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (initialVertexCapacity, initialIndexCapacity, mockDevice),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_u32_0(
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialVertexCapacity, initialIndexCapacity))?;
        Ok(__cordl_object.into())
    }
    pub fn OnEngineUpdateGlobal() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OnEngineUpdateGlobal",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEngineUpdateGlobal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnFlushPendingResources() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OnFlushPendingResources",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnFlushPendingResources",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnFrameRenderingBegin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnFrameRenderingBegin")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnFrameRenderingBegin",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareForGfxDeviceRecreate(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "PrepareForGfxDeviceRecreate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareForGfxDeviceRecreate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessDeviceFreeQueue() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ProcessDeviceFreeQueue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ProcessDeviceFreeQueue",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PruneUnusedPages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PruneUnusedPages")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PruneUnusedPages",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn PtrToSlice<T>(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeSlice_1<T>>
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
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), crate::Unity::Collections::NativeSlice_1<T>, 2usize>(
                        "PtrToSlice"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PtrToSlice",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<T> =
            unsafe { cordl_method_info.invoke_unchecked((), (p, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryAllocFromPage(
        &mut self,
        page: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
        vertexCount: u32,
        indexCount: u32,
        va: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::UIR::Alloc>,
        ia: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::UIR::Alloc>,
        shortLived: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
                        u32,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::UIR::Alloc>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::UIR::Alloc>,
                        bool,
                    ), bool, 6usize>("TryAllocFromPage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAllocFromPage",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (page, vertexCount, indexCount, va, ia, shortLived))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAfterGPUUsedData(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeSlice_1<u16>>,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
        allocToUpdate: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
        >,
        copyBackIndices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
                        u32,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<
                                crate::UnityEngine::UIElements::Vertex,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<u16>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<u16>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "UpdateAfterGPUUsedData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateAfterGPUUsedData",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    vertexCount,
                    indexCount,
                    vertexData,
                    indexData,
                    indexOffset,
                    allocToUpdate,
                    copyBackIndices,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCopyBackIndices(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        copyBackIndices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateCopyBackIndices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateCopyBackIndices",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mesh, copyBackIndices))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFenceValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateFenceValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateFenceValue",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Update_ByRefMut0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<
                                crate::UnityEngine::UIElements::Vertex,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (mesh, vertexCount, vertexData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Update_u32_ByRefMut_ByRefMut_ByRefMut1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
        vertexCount: u32,
        indexCount: u32,
        vertexData: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::UIElements::Vertex>,
        >,
        indexData: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeSlice_1<u16>>,
        indexOffset: quest_hook::libil2cpp::ByRefMut<u16>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
                        u32,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<
                                crate::UnityEngine::UIElements::Vertex,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeSlice_1<u16>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<u16>,
                    ), quest_hook::libil2cpp::Void, 6usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    mesh,
                    vertexCount,
                    indexCount,
                    vertexData,
                    indexData,
                    indexOffset,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitOnCpuFence(
        &mut self,
        fence: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("WaitOnCpuFence")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WaitOnCpuFence",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (fence))? };
        Ok(__cordl_ret.into())
    }
    pub fn WrapUpGfxDeviceRecreate() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "WrapUpGfxDeviceRecreate",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WrapUpGfxDeviceRecreate",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
        mockDevice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32, u32, bool), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (initialVertexCapacity, initialIndexCapacity, mockDevice),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_u32_0(
        &mut self,
        initialVertexCapacity: u32,
        initialIndexCapacity: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32, u32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (initialVertexCapacity, initialIndexCapacity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_breakBatches(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_breakBatches")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_breakBatches",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_commandLists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::UIElements::UIR::CommandList,
                                    >,
                                >,
                            >,
                        >,
                    >, 0usize>("get_commandLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_commandLists",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentFrameCommandLists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::UIElements::UIR::CommandList,
                            >,
                        >,
                    >, 0usize>("get_currentFrameCommandLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_currentFrameCommandLists",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_disposed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_disposed",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_drawsInCameras(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_drawsInCameras")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_drawsInCameras",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_frameIndex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("get_frameIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_frameIndex",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_fullyCreated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_fullyCreated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_fullyCreated",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isFlat(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isFlat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isFlat",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxVerticesPerPage() -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), u32, 0usize>("get_maxVerticesPerPage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_maxVerticesPerPage",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_breakBatches(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_breakBatches")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_breakBatches",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_disposed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_disposed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_drawsInCameras(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_drawsInCameras",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_drawsInCameras",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isFlat(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_isFlat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_isFlat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::UIElements::UIR::UIRenderDevice {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UIRenderDevice_AllocToFree {
    pub alloc: crate::UnityEngine::UIElements::UIR::Alloc,
    pub page: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub vertices: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRenderDevice/AllocToFree";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToFree")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToFree {}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UIRenderDevice_AllocToUpdate {
    pub id: u32,
    pub allocTime: u32,
    pub meshHandle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    pub permAllocVerts: crate::UnityEngine::UIElements::UIR::Alloc,
    pub permAllocIndices: crate::UnityEngine::UIElements::UIR::Alloc,
    pub permPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub copyBackIndices: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRenderDevice/AllocToUpdate";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+AllocToUpdate")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_AllocToUpdate {}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UIRenderDevice_DeviceToFree {
    pub handle: u32,
    pub page: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub commandLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRenderDevice/DeviceToFree";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DeviceToFree")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_DeviceToFree {
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
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UIRenderDevice_DrawStatistics {
    pub currentFrameIndex: i32,
    pub totalIndices: u32,
    pub commandCount: u32,
    pub skippedCommandCount: u32,
    pub drawCommandCount: u32,
    pub disableCommandCount: u32,
    pub materialSetCount: u32,
    pub drawRangeCount: u32,
    pub drawRangeCallCount: u32,
    pub immediateDraws: u32,
    pub stencilRefChanges: u32,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRenderDevice/DrawStatistics";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+DrawStatistics")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_DrawStatistics {}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct UIRenderDevice_EvaluationState {
    pub activeCommandList:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::CommandList>,
    pub constantProps: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    pub batchProps: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    pub defaultMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub curState: crate::UnityEngine::UIElements::UIR::State,
    pub curPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub mustApplyMaterial: bool,
    pub mustApplyBatchProps: bool,
    pub mustApplyStencil: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRenderDevice/EvaluationState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRenderDevice+EvaluationState")]
impl crate::UnityEngine::UIElements::UIR::UIRenderDevice_EvaluationState {}
