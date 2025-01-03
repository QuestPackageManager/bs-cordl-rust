#[cfg(feature = "UnityEngine+RenderTextureDescriptor")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RenderTextureDescriptor {
    pub _width_k__BackingField: i32,
    pub _height_k__BackingField: i32,
    pub _msaaSamples_k__BackingField: i32,
    pub _volumeDepth_k__BackingField: i32,
    pub _mipCount_k__BackingField: i32,
    pub _graphicsFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub _stencilFormat_k__BackingField: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub _depthStencilFormat_k__BackingField: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub _dimension_k__BackingField: crate::UnityEngine::Rendering::TextureDimension,
    pub _shadowSamplingMode_k__BackingField: crate::UnityEngine::Rendering::ShadowSamplingMode,
    pub _vrUsage_k__BackingField: crate::UnityEngine::VRTextureUsage,
    pub _flags: crate::UnityEngine::RenderTextureCreationFlags,
    pub _memoryless_k__BackingField: crate::UnityEngine::RenderTextureMemoryless,
}
#[cfg(feature = "UnityEngine+RenderTextureDescriptor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderTextureDescriptor =>
    "UnityEngine"."RenderTextureDescriptor"
);
#[cfg(feature = "UnityEngine+RenderTextureDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::RenderTextureDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RenderTextureDescriptor")]
impl crate::UnityEngine::RenderTextureDescriptor {
    pub fn SetOrClearRenderTextureCreationFlag(
        &mut self,
        value: bool,
        flag: crate::UnityEngine::RenderTextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetOrClearRenderTextureCreationFlag",
            (value, flag),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_GraphicsFormat3(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, colorFormat, depthStencilFormat),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_GraphicsFormat_i32_4(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, colorFormat, depthStencilFormat, mipCount),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RenderTextureFormat_i32_0(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::RenderTextureFormat,
        depthBufferBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, colorFormat, depthBufferBits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RenderTextureFormat_i32_i32_1(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::RenderTextureFormat,
        depthBufferBits: i32,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, colorFormat, depthBufferBits, mipCount),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RenderTextureFormat_i32_i32_RenderTextureReadWrite2(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::RenderTextureFormat,
        depthBufferBits: i32,
        mipCount: i32,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, colorFormat, depthBufferBits, mipCount, readWrite),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureFormat> {
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_colorFormat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_depthBufferBits(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_depthBufferBits",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_depthStencilFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_depthStencilFormat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dimension(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::TextureDimension> {
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dimension",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_graphicsFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_graphicsFormat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_msaaSamples(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_msaaSamples",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sRGB(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sRGB",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shadowSamplingMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::ShadowSamplingMode,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::ShadowSamplingMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_shadowSamplingMode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volumeDepth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_volumeDepth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_autoGenerateMips(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_autoGenerateMips",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colorFormat(
        &mut self,
        value: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_colorFormat",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_createdFromScript(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_createdFromScript",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_depthBufferBits(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_depthBufferBits",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_depthStencilFormat(
        &mut self,
        value: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_depthStencilFormat",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_dimension(
        &mut self,
        value: crate::UnityEngine::Rendering::TextureDimension,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_dimension",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableRandomWrite(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_enableRandomWrite",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_graphicsFormat(
        &mut self,
        value: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_graphicsFormat",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_height(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_height",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_memoryless(
        &mut self,
        value: crate::UnityEngine::RenderTextureMemoryless,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_memoryless",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mipCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mipCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_msaaSamples(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_msaaSamples",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sRGB(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_sRGB",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shadowSamplingMode(
        &mut self,
        value: crate::UnityEngine::Rendering::ShadowSamplingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_shadowSamplingMode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useDynamicScale(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_useDynamicScale",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useMipMap(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_useMipMap",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_volumeDepth(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_volumeDepth",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vrUsage(
        &mut self,
        value: crate::UnityEngine::VRTextureUsage,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_vrUsage",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_width(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_width",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
