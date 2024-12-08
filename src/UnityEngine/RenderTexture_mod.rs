#[cfg(feature = "UnityEngine+RenderTexture")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderTexture {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "UnityEngine+RenderTexture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RenderTexture => "UnityEngine"
    ."RenderTexture"
);
#[cfg(feature = "UnityEngine+RenderTexture")]
impl std::ops::Deref for crate::UnityEngine::RenderTexture {
    type Target = crate::UnityEngine::Texture;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RenderTexture")]
impl std::ops::DerefMut for crate::UnityEngine::RenderTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RenderTexture")]
impl crate::UnityEngine::RenderTexture {
    pub fn Initialize(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (width, height, depth, format, readWrite, mipCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorFormat(
        &mut self,
        suppressWarnings: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = __cordl_object
            .invoke("GetColorFormat", (suppressWarnings))?;
        Ok(__cordl_ret)
    }
    pub fn get_graphicsFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = __cordl_object
            .invoke("get_graphicsFormat", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_depthBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderBuffer = __cordl_object
            .invoke("get_depthBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_vrUsage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::VRTextureUsage> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::VRTextureUsage = __cordl_object
            .invoke("get_vrUsage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_antiAliasing(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_antiAliasing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderBuffer = __cordl_object
            .invoke("get_colorBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_antiAliasing(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_antiAliasing", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorFormat(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorFormat", (format))?;
        Ok(__cordl_ret)
    }
    pub fn SetSRGBReadWrite(
        &mut self,
        srgb: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSRGBReadWrite", (srgb))?;
        Ok(__cordl_ret)
    }
    pub fn get_sRGB(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_sRGB", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDepthBuffer_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDepthBuffer_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn DiscardContents__cordl_bool__cordl_bool0(
        &mut self,
        discardColor: bool,
        discardDepth: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DiscardContents", (discardColor, discardDepth))?;
        Ok(__cordl_ret)
    }
    pub fn DiscardContents_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DiscardContents", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_descriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureDescriptor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderTextureDescriptor = __cordl_object
            .invoke("get_descriptor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDescriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureDescriptor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderTextureDescriptor = __cordl_object
            .invoke("GetDescriptor", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_dimension(
        &mut self,
        value: crate::UnityEngine::Rendering::TextureDimension,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dimension", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderBuffer = __cordl_object
            .invoke("GetColorBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetColorBuffer_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetColorBuffer_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn IsCreated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCreated", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_width(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_width", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetDescriptor_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDescriptor_Injected", (ret))?;
        Ok(__cordl_ret)
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_width", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetRenderTextureDescriptor_Injected(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::RenderTextureDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRenderTextureDescriptor_Injected", (desc))?;
        Ok(__cordl_ret)
    }
    pub fn SetMipMapCount(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMipMapCount", (count))?;
        Ok(__cordl_ret)
    }
    pub fn get_height(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_height", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_height(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_height", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_depthStencilFormat(
        &mut self,
        value: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_depthStencilFormat", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetRenderTextureDescriptor(
        &mut self,
        desc: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRenderTextureDescriptor", (desc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RenderTextureDescriptor1(
        &mut self,
        desc: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (desc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RenderTexture2(
        &mut self,
        textureToCopy: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (textureToCopy))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_DefaultFormat3(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_GraphicsFormat4(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_GraphicsFormat_i32_5(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, mipCount))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_GraphicsFormat_GraphicsFormat_i32_6(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (width, height, colorFormat, depthStencilFormat, mipCount),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_GraphicsFormat_GraphicsFormat7(
        &mut self,
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, colorFormat, depthStencilFormat))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_RenderTextureFormat_RenderTextureReadWrite8(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, readWrite))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_RenderTextureFormat9(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_10(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_RenderTextureFormat_i32_11(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, depth, format, mipCount))?;
        Ok(__cordl_ret)
    }
    pub fn set_graphicsFormat(
        &mut self,
        value: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_graphicsFormat", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useMipMap(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useMipMap", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureFormat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat = __cordl_object
            .invoke("get_format", ())?;
        Ok(__cordl_ret)
    }
    pub fn Create(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Create", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_depthStencilFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = __cordl_object
            .invoke("get_depthStencilFormat", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDepthBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::RenderBuffer = __cordl_object
            .invoke("GetDepthBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_RenderTextureDescriptor1(
        desc: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (desc))?;
        Ok(__cordl_object)
    }
    pub fn New_RenderTexture2(
        textureToCopy: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (textureToCopy))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_DefaultFormat3(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_GraphicsFormat4(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_GraphicsFormat_i32_5(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, mipCount))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_GraphicsFormat_GraphicsFormat_i32_6(
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, colorFormat, depthStencilFormat, mipCount),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_GraphicsFormat_GraphicsFormat7(
        width: i32,
        height: i32,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, colorFormat, depthStencilFormat))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_RenderTextureFormat_RenderTextureReadWrite8(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, readWrite))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_RenderTextureFormat9(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_10(
        width: i32,
        height: i32,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_RenderTextureFormat_i32_11(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
        mipCount: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, depth, format, mipCount))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+RenderTexture")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RenderTexture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
