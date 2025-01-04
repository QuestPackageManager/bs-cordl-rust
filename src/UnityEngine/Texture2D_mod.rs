#[cfg(feature = "UnityEngine+Texture2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Texture2D {
    __cordl_parent: crate::UnityEngine::Texture,
}
#[cfg(feature = "UnityEngine+Texture2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Texture2D => "UnityEngine"
    ."Texture2D"
);
#[cfg(feature = "UnityEngine+Texture2D")]
impl std::ops::Deref for crate::UnityEngine::Texture2D {
    type Target = crate::UnityEngine::Texture;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2D")]
impl std::ops::DerefMut for crate::UnityEngine::Texture2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Texture2D")]
impl crate::UnityEngine::Texture2D {
    pub const streamingMipmapsPriorityMax: i32 = 127i32;
    pub const streamingMipmapsPriorityMin: i32 = -128i32;
    #[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
    pub type EXRFlags = crate::UnityEngine::Texture2D_EXRFlags;
    pub fn ApplyImpl(
        &mut self,
        updateMipmaps: bool,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyImpl", (updateMipmaps, makeNoLongerReadable))?;
        Ok(__cordl_ret.into())
    }
    pub fn Apply_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Apply__cordl_bool1(
        &mut self,
        updateMipmaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (updateMipmaps))?;
        Ok(__cordl_ret.into())
    }
    pub fn Apply__cordl_bool__cordl_bool0(
        &mut self,
        updateMipmaps: bool,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (updateMipmaps, makeNoLongerReadable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearMinimumMipmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearMinimumMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearRequestedMipmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRequestedMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Compress(
        &mut self,
        highQuality: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Compress", (highQuality))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateExternalTexture(
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateExternalTexture",
                (width, height, format, mipChain, linear, nativeTex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAtlas(
        sizes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        padding: i32,
        atlasSize: i32,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Rect>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateAtlas", (sizes, padding, atlasSize, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAtlasImpl(
        sizes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        padding: i32,
        atlasSize: i32,
        rect: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateAtlasImpl", (sizes, padding, atlasSize, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinearImpl(
        &mut self,
        image: i32,
        mip: i32,
        u: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetPixelBilinearImpl", (image, mip, u, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinearImpl_Injected(
        &mut self,
        image: i32,
        mip: i32,
        u: f32,
        v: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPixelBilinearImpl_Injected", (image, mip, u, v, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinear_f32_f32_0(
        &mut self,
        u: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetPixelBilinear", (u, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelBilinear_i32_1(
        &mut self,
        u: f32,
        v: f32,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetPixelBilinear", (u, v, mipLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelData<T>(
        &mut self,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = __cordl_object
            .invoke("GetPixelData", (mipLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelImpl(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetPixelImpl", (image, mip, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixelImpl_Injected(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPixelImpl_Injected", (image, mip, x, y, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixel_i32_1(
        &mut self,
        x: i32,
        y: i32,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetPixel", (x, y, mipLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixel_i32_i32_0(
        &mut self,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetPixel", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels32_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        > = __cordl_object.invoke("GetPixels32", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels32_i32_0(
        &mut self,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        > = __cordl_object.invoke("GetPixels32", (miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = __cordl_object.invoke("GetPixels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_i32_2(
        &mut self,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = __cordl_object.invoke("GetPixels", (miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_i32_i32_i32_i32_1(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = __cordl_object.invoke("GetPixels", (x, y, blockWidth, blockHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPixels_i32_i32_i32_i32_i32_0(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = __cordl_object
            .invoke("GetPixels", (x, y, blockWidth, blockHeight, miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawImageDataSize(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("GetRawImageDataSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawTextureData_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetRawTextureData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawTextureData_1<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = __cordl_object
            .invoke("GetRawTextureData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWritableImageData(
        &mut self,
        frame: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetWritableImageData", (frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        w: i32,
        h: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_Create",
                (
                    mono,
                    w,
                    h,
                    mipCount,
                    format,
                    colorSpace,
                    flags,
                    nativeTex,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateImpl(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        w: i32,
        h: i32,
        mipCount: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        colorSpace: crate::UnityEngine::TextureColorSpace,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_CreateImpl",
                (
                    mono,
                    w,
                    h,
                    mipCount,
                    format,
                    colorSpace,
                    flags,
                    nativeTex,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRequestedMipmapLevelLoaded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsRequestedMipmapLevelLoaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureDataImpl(
        &mut self,
        data: crate::System::IntPtr,
        _cordl_size: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadRawTextureDataImpl", (data, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureDataImplArray(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadRawTextureDataImplArray", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureData_Il2CppArray1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRawTextureData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureData_IntPtr_i32_0(
        &mut self,
        data: crate::System::IntPtr,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRawTextureData", (data, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRawTextureData_NativeArray_1_2<T>(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadRawTextureData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DefaultFormat_TextureCreationFlags1(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_i32_Il2CppString_TextureCreationFlags3(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, mipCount, mipmapLimitGroupName, flags),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_DefaultFormat_i32_TextureCreationFlags2(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, mipCount, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags4(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_TextureCreationFlags_i32_IntPtr_Il2CppString0(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, flags, mipCount, nativeTex, mipmapLimitGroupName),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_i32_Il2CppString_TextureCreationFlags6(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, mipCount, mipmapLimitGroupName, flags),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_GraphicsFormat_i32_TextureCreationFlags5(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, format, mipCount, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool13(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, textureFormat, mipChain))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool11(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, textureFormat, mipChain, linear))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat__cordl_bool__cordl_bool__cordl_bool12(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, textureFormat, mipChain, linear, createUninitialized),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool8(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height, textureFormat, mipCount, linear))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool_IntPtr__cordl_bool__cordl_bool_Il2CppString7(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        nativeTex: crate::System::IntPtr,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    textureFormat,
                    mipCount,
                    linear,
                    nativeTex,
                    createUninitialized,
                    ignoreMipmapLimit,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool9(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, textureFormat, mipCount, linear, createUninitialized),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_TextureFormat_i32__cordl_bool__cordl_bool__cordl_bool_Il2CppString10(
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    width,
                    height,
                    textureFormat,
                    mipCount,
                    linear,
                    createUninitialized,
                    ignoreMipmapLimit,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_14(
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (width, height))?;
        Ok(__cordl_object.into())
    }
    pub fn PackTextures_Il2CppArray_i32_2(
        &mut self,
        textures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Texture2D>,
        >,
        padding: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        > = __cordl_object.invoke("PackTextures", (textures, padding))?;
        Ok(__cordl_ret.into())
    }
    pub fn PackTextures_i32_1(
        &mut self,
        textures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Texture2D>,
        >,
        padding: i32,
        maximumAtlasSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        > = __cordl_object
            .invoke("PackTextures", (textures, padding, maximumAtlasSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn PackTextures_i32__cordl_bool0(
        &mut self,
        textures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Texture2D>,
        >,
        padding: i32,
        maximumAtlasSize: i32,
        makeNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rect>,
        > = __cordl_object
            .invoke(
                "PackTextures",
                (textures, padding, maximumAtlasSize, makeNoLongerReadable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixelsImpl(
        &mut self,
        source: crate::UnityEngine::Rect,
        destX: i32,
        destY: i32,
        recalculateMipMaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadPixelsImpl", (source, destX, destY, recalculateMipMaps))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixelsImpl_Injected(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        destX: i32,
        destY: i32,
        recalculateMipMaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReadPixelsImpl_Injected",
                (source, destX, destY, recalculateMipMaps),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixels_Rect_i32_i32_1(
        &mut self,
        source: crate::UnityEngine::Rect,
        destX: i32,
        destY: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadPixels", (source, destX, destY))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPixels__cordl_bool0(
        &mut self,
        source: crate::UnityEngine::Rect,
        destX: i32,
        destY: i32,
        recalculateMipMaps: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadPixels", (source, destX, destY, recalculateMipMaps))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeImpl(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReinitializeImpl", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeWithFormatImpl(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ReinitializeWithFormatImpl", (width, height, format, hasMipMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeWithTextureFormatImpl(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ReinitializeWithTextureFormatImpl",
                (width, height, textureFormat, hasMipMap),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize_GraphicsFormat__cordl_bool2(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Reinitialize", (width, height, format, hasMipMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize_TextureFormat__cordl_bool1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Reinitialize", (width, height, format, hasMipMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize_i32_i32_0(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Reinitialize", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize_GraphicsFormat__cordl_bool2(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Resize", (width, height, format, hasMipMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize_TextureFormat__cordl_bool1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::TextureFormat,
        hasMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Resize", (width, height, format, hasMipMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize_i32_i32_0(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Resize", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAllPixels32(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAllPixels32", (colors, miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBlockOfPixels32(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetBlockOfPixels32",
                (x, y, blockWidth, blockHeight, colors, miplevel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelDataImpl(
        &mut self,
        data: crate::System::IntPtr,
        mipLevel: i32,
        elementSize: i32,
        dataArraySize: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetPixelDataImpl",
                (data, mipLevel, elementSize, dataArraySize, sourceDataStartIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelDataImplArray(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        mipLevel: i32,
        elementSize: i32,
        dataArraySize: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetPixelDataImplArray",
                (data, mipLevel, elementSize, dataArraySize, sourceDataStartIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelData_Il2CppArray0<T>(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        mipLevel: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixelData", (data, mipLevel, sourceDataStartIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelData_NativeArray_1_1<T>(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<T>,
        mipLevel: i32,
        sourceDataStartIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixelData", (data, mipLevel, sourceDataStartIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelImpl(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixelImpl", (image, mip, x, y, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelImpl_Injected(
        &mut self,
        image: i32,
        mip: i32,
        x: i32,
        y: i32,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixelImpl_Injected", (image, mip, x, y, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixel_i32_1(
        &mut self,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
        mipLevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixel", (x, y, color, mipLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixel_i32_i32_Color0(
        &mut self,
        x: i32,
        y: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixel", (x, y, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_Il2CppArray1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels32", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_Il2CppArray_i32_0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels32", (colors, miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_i32_i32_i32_i32_Il2CppArray3(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels32", (x, y, blockWidth, blockHeight, colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels32_i32_i32_i32_i32_Il2CppArray_i32_2(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels32", (x, y, blockWidth, blockHeight, colors, miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixelsImpl(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        pixel: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        miplevel: i32,
        frame: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixelsImpl", (x, y, w, h, pixel, miplevel, frame))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_Il2CppArray3(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_Il2CppArray_i32_2(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels", (colors, miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_i32_i32_i32_i32_Il2CppArray1(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels", (x, y, blockWidth, blockHeight, colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPixels_i32_i32_i32_i32_Il2CppArray_i32_0(
        &mut self,
        x: i32,
        y: i32,
        blockWidth: i32,
        blockHeight: i32,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        miplevel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPixels", (x, y, blockWidth, blockHeight, colors, miplevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateExternalTexture(
        &mut self,
        nativeTex: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateExternalTexture", (nativeTex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFormat_GraphicsFormat1(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateFormat", (format, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateFormat_TextureFormat0(
        &mut self,
        format: crate::UnityEngine::TextureFormat,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateFormat", (format, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_TextureCreationFlags1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, format, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_i32_Il2CppString_TextureCreationFlags3(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (width, height, format, mipCount, mipmapLimitGroupName, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DefaultFormat_i32_TextureCreationFlags2(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::DefaultFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, format, mipCount, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags4(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, format, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_TextureCreationFlags_i32_IntPtr_Il2CppString0(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
        mipCount: i32,
        nativeTex: crate::System::IntPtr,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (width, height, format, flags, mipCount, nativeTex, mipmapLimitGroupName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_i32_Il2CppString_TextureCreationFlags6(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (width, height, format, mipCount, mipmapLimitGroupName, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GraphicsFormat_i32_TextureCreationFlags5(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        mipCount: i32,
        flags: crate::UnityEngine::Experimental::Rendering::TextureCreationFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, format, mipCount, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool13(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, textureFormat, mipChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool11(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, textureFormat, mipChain, linear))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat__cordl_bool__cordl_bool__cordl_bool12(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipChain: bool,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (width, height, textureFormat, mipChain, linear, createUninitialized),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool8(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height, textureFormat, mipCount, linear))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool_IntPtr__cordl_bool__cordl_bool_Il2CppString7(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        nativeTex: crate::System::IntPtr,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    width,
                    height,
                    textureFormat,
                    mipCount,
                    linear,
                    nativeTex,
                    createUninitialized,
                    ignoreMipmapLimit,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool9(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (width, height, textureFormat, mipCount, linear, createUninitialized),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TextureFormat_i32__cordl_bool__cordl_bool__cordl_bool_Il2CppString10(
        &mut self,
        width: i32,
        height: i32,
        textureFormat: crate::UnityEngine::TextureFormat,
        mipCount: i32,
        linear: bool,
        createUninitialized: bool,
        ignoreMipmapLimit: bool,
        mipmapLimitGroupName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    width,
                    height,
                    textureFormat,
                    mipCount,
                    linear,
                    createUninitialized,
                    ignoreMipmapLimit,
                    mipmapLimitGroupName,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_14(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeMipmapLimit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_activeMipmapLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_blackTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_blackTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_calculatedMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_calculatedMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_desiredMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_desiredMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextureFormat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TextureFormat = __cordl_object
            .invoke("get_format", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_grayTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_grayTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ignoreMipmapLimit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ignoreMipmapLimit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isPreProcessed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPreProcessed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReadable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_linearGrayTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_linearGrayTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadAllMips(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_loadAllMips", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadedMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_loadedMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadingMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_loadingMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minimumMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_minimumMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mipmapLimitGroup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_mipmapLimitGroup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_normalTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_redTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_redTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_requestedMipmapLevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_requestedMipmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_streamingMipmaps(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_streamingMipmaps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_streamingMipmapsPriority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_streamingMipmapsPriority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vtOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_vtOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteTexture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_whiteTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ignoreMipmapLimit(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ignoreMipmapLimit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_loadAllMips(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_loadAllMips", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minimumMipmapLevel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minimumMipmapLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_requestedMipmapLevel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_requestedMipmapLevel", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Texture2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Texture2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Texture2D_EXRFlags {
    #[default]
    CompressPIZ = 8i32,
    CompressRLE = 4i32,
    CompressZIP = 2i32,
    None = 0i32,
    OutputAsFloat = 1i32,
}
#[cfg(feature = "UnityEngine+Texture2D+EXRFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Texture2D_EXRFlags => "UnityEngine"
    ."Texture2D/EXRFlags"
);
