#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CommandBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CommandBuffer";
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
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CommandBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CommandBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl crate::UnityEngine::Rendering::CommandBuffer {
    pub fn Blit_Identifier(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    crate::UnityEngine::Vector2,
                    crate::UnityEngine::Vector2,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("Blit_Identifier")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Blit_Identifier", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        source,
                        dest,
                        mat,
                        pass,
                        scale,
                        offset,
                        sourceDepthSlice,
                        destDepthSlice,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Identifier_Injected(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("Blit_Identifier_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Blit_Identifier_Injected", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        source,
                        dest,
                        mat,
                        pass,
                        scale,
                        offset,
                        sourceDepthSlice,
                        destDepthSlice,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Material1(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Blit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Blit", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, dest, mat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Material_i32_2(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Blit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Blit", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, dest, mat, pass))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_RenderTargetIdentifier_RenderTargetIdentifier0(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Blit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Blit", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (source, dest))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture(
        &mut self,
        src: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dst: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("CopyTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyTexture", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (src, dst))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Internal(
        &mut self,
        src: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        srcElement: i32,
        srcMip: i32,
        srcX: i32,
        srcY: i32,
        srcWidth: i32,
        srcHeight: i32,
        dst: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dstElement: i32,
        dstMip: i32,
        dstX: i32,
        dstY: i32,
        mode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    i32,
                    i32,
                    i32,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                13usize,
            >("CopyTexture_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CopyTexture_Internal", 13usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        src,
                        srcElement,
                        srcMip,
                        srcX,
                        srcY,
                        srcWidth,
                        srcHeight,
                        dst,
                        dstElement,
                        dstMip,
                        dstX,
                        dstY,
                        mode,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disposing))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("DrawMeshInstanced")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DrawMeshInstanced", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_Mesh_Matrix4x4_Material3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "DrawMesh", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mesh, matrix, material))
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "DrawMesh", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (mesh, matrix, material, submeshIndex))
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "DrawMesh", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (mesh, matrix, material, submeshIndex, shaderPass),
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    crate::UnityEngine::Matrix4x4,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("DrawMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DrawMesh", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (mesh, matrix, material, submeshIndex, shaderPass, properties),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "DrawRenderer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (renderer, material, submeshIndex, shaderPass))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Finalize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat3(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::FilterMode,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("GetTemporaryRT")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTemporaryRT", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nameID, width, height, depthBuffer, filter, format),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat_i32_2(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::FilterMode,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("GetTemporaryRT")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTemporaryRT", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nameID, width, height, depthBuffer, filter, format, antiAliasing),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat_i32__cordl_bool_RenderTextureMemoryless1(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::FilterMode,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    i32,
                    bool,
                    crate::UnityEngine::RenderTextureMemoryless,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >("GetTemporaryRT")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTemporaryRT", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        nameID,
                        width,
                        height,
                        depthBuffer,
                        filter,
                        format,
                        antiAliasing,
                        enableRandomWrite,
                        memorylessMode,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat_i32__cordl_bool_RenderTextureMemoryless__cordl_bool0(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
        useDynamicScale: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::FilterMode,
                    crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    i32,
                    bool,
                    crate::UnityEngine::RenderTextureMemoryless,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                10usize,
            >("GetTemporaryRT")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTemporaryRT", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        nameID,
                        width,
                        height,
                        depthBuffer,
                        filter,
                        format,
                        antiAliasing,
                        enableRandomWrite,
                        memorylessMode,
                        useDynamicScale,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_RenderTextureFormat4(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    i32,
                    i32,
                    crate::UnityEngine::FilterMode,
                    crate::UnityEngine::RenderTextureFormat,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("GetTemporaryRT")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTemporaryRT", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nameID, width, height, depthBuffer, filter, format),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitBuffer() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), crate::System::IntPtr, 0usize>("InitBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitBuffer", 0usize
                )
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    crate::UnityEngine::Matrix4x4,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("Internal_DrawMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_DrawMesh", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (mesh, matrix, material, submeshIndex, shaderPass, properties),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstanced(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("Internal_DrawMeshInstanced")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_DrawMeshInstanced", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh_Injected(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("Internal_DrawMesh_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_DrawMesh_Injected", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (mesh, matrix, material, submeshIndex, shaderPass, properties),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawRenderer(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Internal_DrawRenderer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_DrawRenderer", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (renderer, material, submeshIndex, shaderPass))
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
    pub fn ReleaseBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ReleaseBuffer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReleaseBuffer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseTemporaryRT(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReleaseTemporaryRT")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReleaseTemporaryRT", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat(
        &mut self,
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGlobalFloat", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameID, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGlobalTexture", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (name, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Impl(
        &mut self,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    crate::UnityEngine::Rendering::RenderTextureSubElement,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalTexture_Impl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGlobalTexture_Impl", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameID, rt, element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_RenderTextureSubElement1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    crate::UnityEngine::Rendering::RenderTextureSubElement,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetGlobalTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGlobalTexture", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameID, value, element))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, crate::UnityEngine::Vector4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGlobalVector", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameID, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Injected(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetGlobalVector_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGlobalVector_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameID, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Rendering::RenderTargetIdentifier),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetRenderTarget")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetRenderTarget", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rt))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetSingle_Internal(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    crate::UnityEngine::Rendering::RenderBufferLoadAction,
                    crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    crate::UnityEngine::Rendering::RenderBufferLoadAction,
                    crate::UnityEngine::Rendering::RenderBufferStoreAction,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetRenderTargetSingle_Internal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetRenderTargetSingle_Internal", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        rt,
                        colorLoadAction,
                        colorStoreAction,
                        depthLoadAction,
                        depthStoreAction,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetSingle_Internal_Injected(
        &mut self,
        rt: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    >,
                    crate::UnityEngine::Rendering::RenderBufferLoadAction,
                    crate::UnityEngine::Rendering::RenderBufferStoreAction,
                    crate::UnityEngine::Rendering::RenderBufferLoadAction,
                    crate::UnityEngine::Rendering::RenderBufferStoreAction,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetRenderTargetSingle_Internal_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetRenderTargetSingle_Internal_Injected", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        rt,
                        colorLoadAction,
                        colorStoreAction,
                        depthLoadAction,
                        depthStoreAction,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices(
        &mut self,
        view: crate::UnityEngine::Matrix4x4,
        proj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Matrix4x4, crate::UnityEngine::Matrix4x4),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetViewProjectionMatrices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetViewProjectionMatrices", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (view, proj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices_Injected(
        &mut self,
        view: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        proj: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetViewProjectionMatrices_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetViewProjectionMatrices_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (view, proj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAgainstExecutionFlags(
        &mut self,
        requiredFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
        invalidFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                    crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
                ),
                bool,
                2usize,
            >("ValidateAgainstExecutionFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ValidateAgainstExecutionFlags", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (requiredFlags, invalidFlags))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_name")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_name", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
