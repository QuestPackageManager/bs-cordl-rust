#[cfg(feature = "cordl_class_UnityEngine+Rendering+IRasterCommandBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct IRasterCommandBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+IRasterCommandBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::IRasterCommandBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "IRasterCommandBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+IRasterCommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::IRasterCommandBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IRasterCommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::IRasterCommandBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IRasterCommandBuffer")]
impl crate::UnityEngine::Rendering::IRasterCommandBuffer {
    pub fn ClearRenderTarget_RTClearFlags_Color_f32_u32_3(
        &mut self,
        clearFlags: crate::UnityEngine::Rendering::RTClearFlags,
        backgroundColor: crate::UnityEngine::Color,
        depth: f32,
        stencil: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RTClearFlags,
                            crate::UnityEngine::Color,
                            f32,
                            u32,
                        ),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RTClearFlags,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Color,
                                >,
                            >,
                            f32,
                            u32,
                        ),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
            cordl_method_info
                .invoke_unchecked(self, (clearDepth, clearColor, backgroundColor))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
                .invoke_unchecked(
                    self,
                    (clearDepth, clearColor, backgroundColor, depth),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ConfigureFoveatedRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureFoveatedRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (platformData))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawMeshInstancedIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedIndirect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawMeshInstancedIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedIndirect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawMeshInstancedIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedIndirect", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawMeshInstancedIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedIndirect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawMeshInstancedIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedIndirect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawMeshInstancedIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedIndirect", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawMeshInstancedProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstancedProcedural", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstanced", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (mesh, submeshIndex, material, shaderPass, matrices),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstanced", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawMeshInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMeshInstanced", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMesh", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (mesh, matrix, material))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMesh", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (mesh, matrix, material, submeshIndex))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMesh", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (mesh, matrix, material, submeshIndex, shaderPass),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMesh", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        subsetIndices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        count: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Matrix4x4,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawMultipleMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawMultipleMeshes", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (normalizedCamViewport))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (indexBuffer, matrix, material, shaderPass, topology, bufferWithArgs),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (indexBuffer, matrix, material, shaderPass, topology, bufferWithArgs),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (matrix, material, shaderPass, topology, bufferWithArgs, argsOffset),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (matrix, material, shaderPass, topology, bufferWithArgs, argsOffset),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawProceduralIndirect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProceduralIndirect", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProcedural", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (indexBuffer, matrix, material, shaderPass, topology, indexCount),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProcedural", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::GraphicsBuffer,
                            >,
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProcedural", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProcedural", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (matrix, material, shaderPass, topology, vertexCount),
                )?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProcedural", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (matrix, material, shaderPass, topology, vertexCount, instanceCount),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Matrix4x4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            crate::UnityEngine::MeshTopology,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("DrawProcedural")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawProcedural", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_Renderer_Material2(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawRenderer", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderer, material))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer_i32_1(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawRenderer", 3usize
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("DrawRenderer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DrawRenderer", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderer, material, submeshIndex, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFoveatedRenderingMode(
        &mut self,
        foveatedRenderingMode: crate::UnityEngine::Rendering::FoveatedRenderingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (foveatedRenderingMode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInstanceMultiplier(
        &mut self,
        multiplier: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetInstanceMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetInstanceMultiplier", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (multiplier))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetWireframe(
        &mut self,
        enable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetWireframe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetWireframe", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (enable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+IRasterCommandBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::IRasterCommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+IRasterCommandBuffer")]
impl AsRef<crate::UnityEngine::Rendering::IBaseCommandBuffer>
for crate::UnityEngine::Rendering::IRasterCommandBuffer {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+IRasterCommandBuffer")]
impl AsMut<crate::UnityEngine::Rendering::IBaseCommandBuffer>
for crate::UnityEngine::Rendering::IRasterCommandBuffer {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IBaseCommandBuffer {
        unsafe { std::mem::transmute(self) }
    }
}
