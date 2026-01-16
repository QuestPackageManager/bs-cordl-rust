#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils";
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
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils")]
impl crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils {
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
    )]
    pub type BlitFilterMode = crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
    )]
    pub type BlitMaterialParameters = crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
    )]
    pub type BlitMaterialPassData = crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
    )]
    pub type BlitPassData = crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
    )]
    pub type CopyPassData = crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
    )]
    pub type FullScreenGeometryType = crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType;
    pub fn AddBlitPass_RenderGraphUtils_BlitMaterialParameters_Il2CppString_Il2CppString_i32_1(
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        blitParameters: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("AddBlitPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddBlitPass", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (graph, blitParameters, passName, file, line))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBlitPass_TextureHandle_TextureHandle_Vector2_Vector2_i32_i32_i32_i32_i32_i32_RenderGraphUtils_BlitFilterMode_Il2CppString_Il2CppString_i32_0(
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        sourceSlice: i32,
        destinationSlice: i32,
        numSlices: i32,
        sourceMip: i32,
        destinationMip: i32,
        numMips: i32,
        filterMode: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        15usize,
                    >("AddBlitPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddBlitPass", 15usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        graph,
                        source,
                        destination,
                        scale,
                        offset,
                        sourceSlice,
                        destinationSlice,
                        numSlices,
                        sourceMip,
                        destinationMip,
                        numMips,
                        filterMode,
                        passName,
                        file,
                        line,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCopyPass(
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        sourceSlice: i32,
        destinationSlice: i32,
        sourceMip: i32,
        destinationMip: i32,
        passName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        file: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        line: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            i32,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >("AddCopyPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddCopyPass", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        graph,
                        source,
                        destination,
                        sourceSlice,
                        destinationSlice,
                        sourceMip,
                        destinationMip,
                        passName,
                        file,
                        line,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitMaterialRenderFunc(
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData,
        >,
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::UnsafeGraphContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::UnsafeGraphContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("BlitMaterialRenderFunc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BlitMaterialRenderFunc", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (data, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitRenderFunc(
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData,
        >,
        context: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::UnsafeGraphContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::UnsafeGraphContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("BlitRenderFunc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BlitRenderFunc", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (data, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CanAddCopyPassMSAA() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("CanAddCopyPassMSAA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanAddCopyPassMSAA", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyRenderFunc(
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData,
        >,
        rgContext: crate::UnityEngine::Rendering::RenderGraphModule::RasterGraphContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::RasterGraphContext,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("CopyRenderFunc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CopyRenderFunc", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (data, rgContext))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderGraphUtils_BlitFilterMode {
    #[default]
    ClampBilinear = 1i32,
    ClampNearest = 0i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils/BlitFilterMode";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitFilterMode"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderGraphUtils_BlitMaterialParameters {
    pub source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub scale: crate::UnityEngine::Vector2,
    pub offset: crate::UnityEngine::Vector2,
    pub sourceSlice: i32,
    pub destinationSlice: i32,
    pub numSlices: i32,
    pub sourceMip: i32,
    pub destinationMip: i32,
    pub numMips: i32,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub shaderPass: i32,
    pub propertyBlock: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub sourceTexturePropertyID: i32,
    pub sourceSlicePropertyID: i32,
    pub sourceMipPropertyID: i32,
    pub scaleBiasPropertyID: i32,
    pub geometry: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils/BlitMaterialParameters";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialParameters"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialParameters {
    pub fn _ctor_Material_i32_0(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (source, destination, material, shaderPass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Material_i32_MaterialPropertyBlock_RenderGraphUtils_FullScreenGeometryType_i32_i32_i32_4(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        mpb: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        geometry: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
        sourceTexturePropertyID: i32,
        sourceSlicePropertyID: i32,
        sourceMipPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        source,
                        destination,
                        material,
                        shaderPass,
                        mpb,
                        geometry,
                        sourceTexturePropertyID,
                        sourceSlicePropertyID,
                        sourceMipPropertyID,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Material_i32_MaterialPropertyBlock_i32_i32_i32_i32_i32_i32_RenderGraphUtils_FullScreenGeometryType_i32_i32_i32_2(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        mpb: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        destinationSlice: i32,
        destinationMip: i32,
        numSlices: i32,
        numMips: i32,
        sourceSlice: i32,
        sourceMip: i32,
        geometry: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
        sourceTexturePropertyID: i32,
        sourceSlicePropertyID: i32,
        sourceMipPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        15usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            15usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        source,
                        destination,
                        material,
                        shaderPass,
                        mpb,
                        destinationSlice,
                        destinationMip,
                        numSlices,
                        numMips,
                        sourceSlice,
                        sourceMip,
                        geometry,
                        sourceTexturePropertyID,
                        sourceSlicePropertyID,
                        sourceMipPropertyID,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector2_Vector2_Material_i32_1(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (source, destination, scale, offset, material, shaderPass),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector2_Vector2_Material_i32_MaterialPropertyBlock_RenderGraphUtils_FullScreenGeometryType_i32_i32_i32_i32_5(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        mpb: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        geometry: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
        sourceTexturePropertyID: i32,
        sourceSlicePropertyID: i32,
        sourceMipPropertyID: i32,
        scaleBiasPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        source,
                        destination,
                        scale,
                        offset,
                        material,
                        shaderPass,
                        mpb,
                        geometry,
                        sourceTexturePropertyID,
                        sourceSlicePropertyID,
                        sourceMipPropertyID,
                        scaleBiasPropertyID,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector2_Vector2_Material_i32_MaterialPropertyBlock_i32_i32_i32_i32_i32_i32_RenderGraphUtils_FullScreenGeometryType_i32_i32_i32_i32_3(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        mpb: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        destinationSlice: i32,
        destinationMip: i32,
        numSlices: i32,
        numMips: i32,
        sourceSlice: i32,
        sourceMip: i32,
        geometry: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
        sourceTexturePropertyID: i32,
        sourceSlicePropertyID: i32,
        sourceMipPropertyID: i32,
        scaleBiasPropertyID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::Vector2,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        18usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            18usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        source,
                        destination,
                        scale,
                        offset,
                        material,
                        shaderPass,
                        mpb,
                        destinationSlice,
                        destinationMip,
                        numSlices,
                        numMips,
                        sourceSlice,
                        sourceMip,
                        geometry,
                        sourceTexturePropertyID,
                        sourceSlicePropertyID,
                        sourceMipPropertyID,
                        scaleBiasPropertyID,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphUtils_BlitMaterialPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sourceTexturePropertyID: i32,
    pub source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub scale: crate::UnityEngine::Vector2,
    pub offset: crate::UnityEngine::Vector2,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub shaderPass: i32,
    pub propertyBlock: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::MaterialPropertyBlock,
    >,
    pub sourceSlice: i32,
    pub destinationSlice: i32,
    pub numSlices: i32,
    pub sourceMip: i32,
    pub destinationMip: i32,
    pub numMips: i32,
    pub geometry: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType,
    pub sourceSlicePropertyID: i32,
    pub sourceMipPropertyID: i32,
    pub scaleBiasPropertyID: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils/BlitMaterialPassData";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitMaterialPassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitMaterialPassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphUtils_BlitPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub scale: crate::UnityEngine::Vector2,
    pub offset: crate::UnityEngine::Vector2,
    pub sourceSlice: i32,
    pub destinationSlice: i32,
    pub numSlices: i32,
    pub sourceMip: i32,
    pub destinationMip: i32,
    pub numMips: i32,
    pub filterMode: crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitFilterMode,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils/BlitPassData";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+BlitPassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_BlitPassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphUtils_CopyPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub isMSAA: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils/CopyPassData";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+CopyPassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_CopyPassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderGraphUtils_FullScreenGeometryType {
    #[default]
    Mesh = 0i32,
    ProceduralQuad = 2i32,
    ProceduralTriangle = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.Util";
    const CLASS_NAME: &'static str = "RenderGraphUtils/FullScreenGeometryType";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+Util+RenderGraphUtils+FullScreenGeometryType"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::Util::RenderGraphUtils_FullScreenGeometryType {
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
