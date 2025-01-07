#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicsFormatUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "GraphicsFormatUtility";
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
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    pub fn CanDecompressFormat_GraphicsFormat1(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanDecompressFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanDecompressFormat__cordl_bool0(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        wholeImage: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanDecompressFormat", (format, wholeImage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthBits(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDepthBits", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthStencilFormatFromBitsLegacy_Native(
        minimumDepthBits: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDepthStencilFormatFromBitsLegacy_Native", (minimumDepthBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthStencilFormat_i32_0(
        minimumDepthBits: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDepthStencilFormat", (minimumDepthBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthStencilFormat_i32_1(
        minimumDepthBits: i32,
        minimumStencilBits: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDepthStencilFormat", (minimumDepthBits, minimumStencilBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_Native_RenderTextureFormat(
        format: crate::UnityEngine::RenderTextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsFormat_Native_RenderTextureFormat", (format, isSRGB))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_Native_TextureFormat(
        format: crate::UnityEngine::TextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsFormat_Native_TextureFormat", (format, isSRGB))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_RenderTextureFormat_RenderTextureReadWrite2(
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsFormat", (format, readWrite))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_RenderTextureFormat__cordl_bool1(
        format: crate::UnityEngine::RenderTextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsFormat", (format, isSRGB))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_TextureFormat__cordl_bool0(
        format: crate::UnityEngine::TextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetGraphicsFormat", (format, isSRGB))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLinearFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLinearFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTextureFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureFormat> {
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderTextureFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSRGBFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSRGBFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCompressedFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCompressedFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCompressedFormat_Native_TextureFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCompressedFormat_Native_TextureFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCrunchFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCrunchFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDepthStencilFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDepthStencilFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPVRTCFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPVRTCFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSRGBFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSRGBFormat", (format))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
