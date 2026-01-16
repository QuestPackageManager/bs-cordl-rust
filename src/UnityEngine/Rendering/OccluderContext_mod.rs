#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OccluderContext {
    pub version: i32,
    pub depthBufferSize: crate::UnityEngine::Vector2Int,
    pub subviewData: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::OccluderDerivedData,
    >,
    pub subviewValidMask: i32,
    pub occluderMipBounds: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::OccluderMipBounds,
    >,
    pub occluderMipLayoutSize: crate::UnityEngine::Vector2Int,
    pub occluderDepthPyramidSize: crate::UnityEngine::Vector2Int,
    pub occluderDepthPyramid: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RTHandle,
    >,
    pub occlusionDebugOverlaySize: i32,
    pub occlusionDebugOverlay: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GraphicsBuffer,
    >,
    pub debugNeedsClear: bool,
    pub constantBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub constantBufferData: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::OccluderDepthPyramidConstants,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OccluderContext {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OccluderContext";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OccluderContext {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OccluderContext {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OccluderContext {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OccluderContext {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OccluderContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OccluderContext")]
impl crate::UnityEngine::Rendering::OccluderContext {
    pub const k_FirstDepthMipIndex: i32 = 3i32;
    pub const k_MaxOccluderMips: i32 = 8i32;
    pub const k_MaxSilhouettePlanes: i32 = 6i32;
    pub const k_MaxSubviewsPerView: i32 = 6i32;
    #[cfg(feature = "UnityEngine+Rendering+OccluderContext+ShaderIDs")]
    pub type ShaderIDs = crate::UnityEngine::Rendering::OccluderContext_ShaderIDs;
    pub fn AllocateTexturesIfNecessary(
        &mut self,
        debugOverlayEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("AllocateTexturesIfNecessary")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateTexturesIfNecessary", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (debugOverlayEnabled))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFarDepthPyramid(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ComputeCommandBuffer,
        >,
        occluderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderParameters,
        >,
        occluderSubviewUpdates: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
        >,
        occluderHandles: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderHandles,
        >,
        silhouettePlanes: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Plane,
        >,
        occluderDepthPyramidCS: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ComputeShader,
        >,
        occluderDepthDownscaleKernel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ComputeCommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::OccluderParameters,
                            >,
                            crate::System::ReadOnlySpan_1<
                                crate::UnityEngine::Rendering::OccluderSubviewUpdate,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::OccluderHandles,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Plane,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("CreateFarDepthPyramid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateFarDepthPyramid", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        cmd,
                        occluderParams,
                        occluderSubviewUpdates,
                        occluderHandles,
                        silhouettePlanes,
                        occluderDepthPyramidCS,
                        occluderDepthDownscaleKernel,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDebugOutput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::OcclusionCullingDebugOutput,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::OcclusionCullingDebugOutput,
                        0usize,
                    >("GetDebugOutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDebugOutput", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::OcclusionCullingDebugOutput = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Import(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::OccluderHandles> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >),
                        crate::UnityEngine::Rendering::OccluderHandles,
                        1usize,
                    >("Import")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Import",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::OccluderHandles = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderGraph))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSubviewValid(
        &mut self,
        subviewIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("IsSubviewValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSubviewValid", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (subviewIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareOccluders(
        &mut self,
        occluderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderParameters,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PrepareOccluders")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PrepareOccluders", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (occluderParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyword(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ComputeCommandBuffer,
        >,
        cs: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        keyword: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::LocalKeyword,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ComputeCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::LocalKeyword,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SetKeyword")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetKeyword", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, cs, keyword, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupFarDepthPyramidConstants(
        &mut self,
        occluderSubviewUpdates: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
        >,
        silhouettePlanes: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Plane,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::OccluderDepthPyramidConstants,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::ReadOnlySpan_1<
                                crate::UnityEngine::Rendering::OccluderSubviewUpdate,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Plane,
                            >,
                        ),
                        crate::UnityEngine::Rendering::OccluderDepthPyramidConstants,
                        2usize,
                    >("SetupFarDepthPyramidConstants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupFarDepthPyramidConstants", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::OccluderDepthPyramidConstants = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (occluderSubviewUpdates, silhouettePlanes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMipBounds(
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
                    >("UpdateMipBounds")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateMipBounds", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_depthBufferSizeInOccluderPixels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Vector2,
                        0usize,
                    >("get_depthBufferSizeInOccluderPixels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_depthBufferSizeInOccluderPixels", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_subviewCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_subviewCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_subviewCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+OccluderContext")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::Rendering::OccluderContext {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+OccluderContext")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::Rendering::OccluderContext {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext+ShaderIDs")]
#[repr(C)]
#[derive(Debug)]
pub struct OccluderContext_ShaderIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext+ShaderIDs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OccluderContext_ShaderIDs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OccluderContext/ShaderIDs";
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
#[cfg(feature = "UnityEngine+Rendering+OccluderContext+ShaderIDs")]
impl std::ops::Deref for crate::UnityEngine::Rendering::OccluderContext_ShaderIDs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OccluderContext+ShaderIDs")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::OccluderContext_ShaderIDs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OccluderContext+ShaderIDs")]
impl crate::UnityEngine::Rendering::OccluderContext_ShaderIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OccluderContext+ShaderIDs")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::OccluderContext_ShaderIDs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
