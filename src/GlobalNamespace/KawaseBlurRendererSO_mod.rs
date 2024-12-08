#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
#[repr(C)]
#[derive(Debug)]
pub struct KawaseBlurRendererSO_BloomKernel {
    __cordl_parent: crate::System::Object,
    pub kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    pub sharedPartWithNext: i32,
}
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::KawaseBlurRendererSO_BloomKernel => ""
    ."KawaseBlurRendererSO/BloomKernel"
);
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl std::ops::Deref for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl std::ops::DerefMut for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "KawaseBlurRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct KawaseBlurRendererSO {
    __cordl_parent: PersistentScriptableObject,
    pub _kawaseBlurShader: *mut crate::UnityEngine::Shader,
    pub _additiveShader: *mut crate::UnityEngine::Shader,
    pub _tintShader: *mut crate::UnityEngine::Shader,
    pub _kawaseBlurMaterial: *mut crate::UnityEngine::Material,
    pub _additiveMaterial: *mut crate::UnityEngine::Material,
    pub _tintMaterial: *mut crate::UnityEngine::Material,
    pub _commandBuffersMaterial: *mut crate::UnityEngine::Material,
    pub _kernels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub _bloomKernels: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel,
    >,
    pub _blurTextures: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::RenderTexture,
    >,
}
#[cfg(feature = "KawaseBlurRendererSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for KawaseBlurRendererSO => ""."KawaseBlurRendererSO"
);
#[cfg(feature = "KawaseBlurRendererSO")]
impl std::ops::Deref for KawaseBlurRendererSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO")]
impl std::ops::DerefMut for KawaseBlurRendererSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "KawaseBlurRendererSO")]
impl KawaseBlurRendererSO {
    pub const kMaxBloomIterations: i32 = 5i32;
    #[cfg(feature = "KawaseBlurRendererSO+KernelSize")]
    pub type KernelSize = crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize;
    #[cfg(feature = "KawaseBlurRendererSO+BloomKernel")]
    pub type BloomKernel = crate::GlobalNamespace::KawaseBlurRendererSO_BloomKernel;
    #[cfg(feature = "KawaseBlurRendererSO+Pass")]
    pub type Pass = crate::GlobalNamespace::KawaseBlurRendererSO_Pass;
    #[cfg(feature = "KawaseBlurRendererSO+WeightsType")]
    pub type WeightsType = crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType;
    pub fn AlphaWeights(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
        dest: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AlphaWeights", (src, dest))?;
        Ok(__cordl_ret)
    }
    pub fn Bloom(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
        dest: *mut crate::UnityEngine::RenderTexture,
        iterationsStart: i32,
        iterations: i32,
        boost: f32,
        alphaWeights: f32,
        blurStartWeightsType: crate::GlobalNamespace::KawaseBlurRendererSO_WeightsType,
        bloomIterationWeights: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Bloom",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn Blur_KawaseBlurRendererSO_KernelSize_i32_0(
        &mut self,
        src: *mut crate::UnityEngine::Texture,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        downsample: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("Blur", (src, kernelSize, downsample))?;
        Ok(__cordl_ret)
    }
    pub fn Blur_RenderTexture_Il2CppArray_f32_i32_i32_i32_f32_f32__cordl_bool__cordl_bool_KawaseBlurRendererSO_WeightsType2(
        &mut self,
        src: *mut crate::UnityEngine::Texture,
        dest: *mut crate::UnityEngine::RenderTexture,
        kernel: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Blur",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn Blur_RenderTexture_KawaseBlurRendererSO_KernelSize_f32_i32_1(
        &mut self,
        src: *mut crate::UnityEngine::Texture,
        dest: *mut crate::UnityEngine::RenderTexture,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost: f32,
        downsample: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blur", (src, dest, kernelSize, boost, downsample))?;
        Ok(__cordl_ret)
    }
    pub fn CreateBlurCommandBuffer(
        &mut self,
        width: i32,
        height: i32,
        globalTextureName: *mut crate::System::String,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Rendering::CommandBuffer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Rendering::CommandBuffer = __cordl_object
            .invoke(
                "CreateBlurCommandBuffer",
                (width, height, globalTextureName, kernelSize, boost),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DoubleBlur(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
        dest: *mut crate::UnityEngine::RenderTexture,
        kernelSize0: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost0: f32,
        kernelSize1: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
        boost1: f32,
        secondBlurAlpha: f32,
        downsample: i32,
        gammaCorrection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DoubleBlur",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetBlurKernel(
        &mut self,
        kernelSize: crate::GlobalNamespace::KawaseBlurRendererSO_KernelSize,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetBlurKernel", (kernelSize))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "KawaseBlurRendererSO")]
impl quest_hook::libil2cpp::ObjectType for KawaseBlurRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "KawaseBlurRendererSO+KernelSize")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KawaseBlurRendererSO_KernelSize {
    Kernel127 = 5i32,
    Kernel135 = 6i32,
    Kernel143 = 7i32,
    Kernel15 = 1i32,
    Kernel23 = 2i32,
    Kernel35 = 3i32,
    Kernel63 = 4i32,
    Kernel7 = 0i32,
}
#[cfg(feature = "KawaseBlurRendererSO+KernelSize")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::KawaseBlurRendererSO_KernelSize
    => ""."KawaseBlurRendererSO/KernelSize"
);
#[cfg(feature = "KawaseBlurRendererSO+Pass")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KawaseBlurRendererSO_Pass {
    AlphaAndDepthWeights = 4i32,
    AlphaWeights = 0i32,
    Blur = 1i32,
    BlurAndAdd = 2i32,
    BlurGamma = 5i32,
    BlurGammaAndAdd = 6i32,
    BlurWithAlphaWeights = 3i32,
}
#[cfg(feature = "KawaseBlurRendererSO+Pass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::KawaseBlurRendererSO_Pass => ""
    ."KawaseBlurRendererSO/Pass"
);
#[cfg(feature = "KawaseBlurRendererSO+WeightsType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KawaseBlurRendererSO_WeightsType {
    AlphaAndDepthWeights = 2i32,
    AlphaWeights = 1i32,
    None = 0i32,
}
#[cfg(feature = "KawaseBlurRendererSO+WeightsType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::KawaseBlurRendererSO_WeightsType => ""
    ."KawaseBlurRendererSO/WeightsType"
);
