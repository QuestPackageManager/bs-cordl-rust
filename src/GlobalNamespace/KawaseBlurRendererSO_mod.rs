#[cfg(feature = "cordl_class_KawaseBlurRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct KawaseBlurRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _kawaseBlurShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _additiveShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _tintShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _kawaseBlurMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _additiveMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _tintMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _commandBuffersMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _kernels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
    >,
    pub _bloomKernels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel,
            >,
        >,
    >,
    pub _blurTextures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
    >,
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::KawaseBlurRendererSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KawaseBlurRendererSO";
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
#[cfg(feature = "KawaseBlurRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::KawaseBlurRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::KawaseBlurRendererSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO")]
impl crate::GlobalNamespace::KawaseBlurRendererSO {
    pub const kMaxBloomIterations: i32 = 5i32;
    #[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
    pub type BloomKernel = crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel;
    #[cfg(feature = "KawaseBlurRendererSO+KernelSize")]
    pub type KernelSize = crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize;
    #[cfg(feature = "KawaseBlurRendererSO+Pass")]
    pub type Pass = crate::GlobalNamespace::KawaseBlurRendererSO_Pass;
    #[cfg(feature = "KawaseBlurRendererSO+WeightsType")]
    pub type WeightsType = crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType;
    pub fn AlphaWeights(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AlphaWeights")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AlphaWeights", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, dest))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Bloom(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        iterationsStart: i32,
        iterations: i32,
        boost: f32,
        alphaWeights: f32,
        blurStartWeightsType: crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType,
        bloomIterationWeights: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            i32,
                            i32,
                            f32,
                            f32,
                            crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >("Bloom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Bloom",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        src,
                        dest,
                        iterationsStart,
                        iterations,
                        boost,
                        alphaWeights,
                        blurStartWeightsType,
                        bloomIterationWeights,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blur_KawaseBlurRendererSO_KernelSize_i32_0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        downsample: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        3usize,
                    >("Blur")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blur",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            cordl_method_info.invoke_unchecked(self, (src, kernelSize, downsample))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blur_RenderTexture_Il2CppArray_f32_i32_i32_i32_f32_f32__cordl_bool__cordl_bool_KawaseBlurRendererSO_WeightsType2(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        kernel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        boost: f32,
        downsample: i32,
        startIdx: i32,
        length: i32,
        alphaWeights: f32,
        additiveAlpha: f32,
        additivelyBlendToDest: bool,
        gammaCorrection: bool,
        blurStartWeightsType: crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<i32>,
                            >,
                            f32,
                            i32,
                            i32,
                            i32,
                            f32,
                            f32,
                            bool,
                            bool,
                            crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("Blur")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blur",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        src,
                        dest,
                        kernel,
                        boost,
                        downsample,
                        startIdx,
                        length,
                        alphaWeights,
                        additiveAlpha,
                        additivelyBlendToDest,
                        gammaCorrection,
                        blurStartWeightsType,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blur_RenderTexture_KawaseBlurRendererSO_KernelSize_f32_i32_1(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost: f32,
        downsample: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
                            f32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Blur")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Blur",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (src, dest, kernelSize, boost, downsample))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateBlurCommandBuffer(
        &mut self,
        width: i32,
        height: i32,
        globalTextureName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >,
                        5usize,
                    >("CreateBlurCommandBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateBlurCommandBuffer", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CommandBuffer,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (width, height, globalTextureName, kernelSize, boost),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoubleBlur(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        kernelSize0: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost0: f32,
        kernelSize1: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost1: f32,
        secondBlurAlpha: f32,
        downsample: i32,
        gammaCorrection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
                            f32,
                            crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
                            f32,
                            f32,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("DoubleBlur")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DoubleBlur", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        src,
                        dest,
                        kernelSize0,
                        boost0,
                        kernelSize1,
                        boost1,
                        secondBlurAlpha,
                        downsample,
                        gammaCorrection,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBlurKernel(
        &mut self,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i32>,
                        >,
                        1usize,
                    >("GetBlurKernel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBlurKernel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (kernelSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDisable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEnable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::KawaseBlurRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+BloomKernel")]
#[repr(C)]
#[derive(Debug)]
pub struct KawaseBlurRendererSO_BloomKernel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub sharedPartWithNext: i32,
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+BloomKernel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KawaseBlurRendererSO/BloomKernel";
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
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl std::ops::Deref for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl std::ops::DerefMut for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+BloomKernel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+KernelSize")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KawaseBlurRendererSO_KernelSize {
    #[default]
    Kernel127 = 5i32,
    Kernel135 = 6i32,
    Kernel143 = 7i32,
    Kernel15 = 1i32,
    Kernel23 = 2i32,
    Kernel35 = 3i32,
    Kernel63 = 4i32,
    Kernel7 = 0i32,
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+KernelSize")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KawaseBlurRendererSO/KernelSize";
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+KernelSize")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+KernelSize")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+KernelSize")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+KernelSize")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+Pass")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KawaseBlurRendererSO_Pass {
    #[default]
    AlphaAndDepthWeights = 4i32,
    AlphaWeights = 0i32,
    Blur = 1i32,
    BlurAndAdd = 2i32,
    BlurGamma = 5i32,
    BlurGammaAndAdd = 6i32,
    BlurWithAlphaWeights = 3i32,
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::KawaseBlurRendererSO_Pass {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KawaseBlurRendererSO/Pass";
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::KawaseBlurRendererSO_Pass {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::KawaseBlurRendererSO_Pass {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::KawaseBlurRendererSO_Pass {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::KawaseBlurRendererSO_Pass {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+WeightsType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KawaseBlurRendererSO_WeightsType {
    #[default]
    AlphaAndDepthWeights = 2i32,
    AlphaWeights = 1i32,
    None = 0i32,
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+WeightsType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KawaseBlurRendererSO/WeightsType";
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+WeightsType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+WeightsType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+WeightsType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType {
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
#[cfg(feature = "cordl_class_KawaseBlurRendererSO+WeightsType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType {
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
