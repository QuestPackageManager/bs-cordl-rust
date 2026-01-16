#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas")]
#[repr(C)]
#[derive(Debug)]
pub struct PowerOfTwoTextureAtlas {
    __cordl_parent: crate::UnityEngine::Rendering::Texture2DAtlas,
    pub m_MipPadding: i32,
    pub m_RequestedTextures: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, crate::UnityEngine::Vector2Int>,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "PowerOfTwoTextureAtlas";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+PowerOfTwoTextureAtlas")]
impl std::ops::Deref for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas {
    type Target = crate::UnityEngine::Rendering::Texture2DAtlas;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+PowerOfTwoTextureAtlas")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+PowerOfTwoTextureAtlas")]
impl crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas {
    pub const k_MipmapFactorApprox: f32 = 1.33f32;
    #[cfg(feature = "UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
    pub type BlitType = crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType;
    pub fn AllocateTexture(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        width: i32,
        height: i32,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                        i32,
                    ), bool, 6usize>("AllocateTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllocateTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cmd, scaleOffset, texture, width, height, overrideInstanceID),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit2DTexture(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: crate::UnityEngine::Vector4,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceScaleOffset: crate::UnityEngine::Vector4,
        blitMips: bool,
        blitType: crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        bool,
                        crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType,
                    ), quest_hook::libil2cpp::Void, 6usize>("Blit2DTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit2DTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    scaleOffset,
                    texture,
                    sourceScaleOffset,
                    blitMips,
                    blitType,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitOctahedralTexture(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: crate::UnityEngine::Vector4,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceScaleOffset: crate::UnityEngine::Vector4,
        blitMips: bool,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "BlitOctahedralTexture"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitOctahedralTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    scaleOffset,
                    texture,
                    sourceScaleOffset,
                    blitMips,
                    overrideInstanceID,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitOctahedralTextureMultiply(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: crate::UnityEngine::Vector4,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceScaleOffset: crate::UnityEngine::Vector4,
        blitMips: bool,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "BlitOctahedralTextureMultiply"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitOctahedralTextureMultiply",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    scaleOffset,
                    texture,
                    sourceScaleOffset,
                    blitMips,
                    overrideInstanceID,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: crate::UnityEngine::Vector4,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceScaleOffset: crate::UnityEngine::Vector4,
        blitMips: bool,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    scaleOffset,
                    texture,
                    sourceScaleOffset,
                    blitMips,
                    overrideInstanceID,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTextureMultiply(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: crate::UnityEngine::Vector4,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceScaleOffset: crate::UnityEngine::Vector4,
        blitMips: bool,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "BlitTextureMultiply"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTextureMultiply",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    scaleOffset,
                    texture,
                    sourceScaleOffset,
                    blitMips,
                    overrideInstanceID,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetApproxCacheSizeInByte(
        nbElement: i32,
        resolution: i32,
        hasMipmap: bool,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        i32,
                        bool,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    ), i64, 4usize>("GetApproxCacheSizeInByte")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetApproxCacheSizeInByte",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe {
            cordl_method_info.invoke_unchecked((), (nbElement, resolution, hasMipmap, format))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxCacheSizeForWeightInByte(
        weight: i32,
        hasMipmap: bool,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        bool,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    ), i32, 3usize>("GetMaxCacheSizeForWeightInByte")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxCacheSizeForWeightInByte",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (weight, hasMipmap, format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPayloadScaleOffset_ByRefMut_ByRefMut1(
        textureSize: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        paddingSize: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                    ), crate::UnityEngine::Vector4, 3usize>(
                        "GetPayloadScaleOffset"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPayloadScaleOffset",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 = unsafe {
            cordl_method_info.invoke_unchecked((), (textureSize, paddingSize, scaleOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPayloadScaleOffset_Texture0(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                    ), crate::UnityEngine::Vector4, 2usize>(
                        "GetPayloadScaleOffset"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPayloadScaleOffset",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture, scaleOffset))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPowerOfTwoTextureSize(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("GetPowerOfTwoTextureSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPowerOfTwoTextureSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTexturePadding(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetTexturePadding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetTexturePadding",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_size: i32,
        mipPadding: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (_cordl_size, mipPadding, format, filterMode, name, useMipMap),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn RelayoutEntries(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("RelayoutEntries")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RelayoutEntries",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReserveSpace_Texture0(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        bool,
                        1usize,
                    >("ReserveSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReserveSpace", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReserveSpace_Texture_Texture_i32_i32_2(
        &mut self,
        textureA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                    ), bool, 4usize>("ReserveSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReserveSpace",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureA, textureB, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReserveSpace_Texture_i32_i32_1(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        i32,
                        i32,
                    ), bool, 3usize>("ReserveSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReserveSpace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture, width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReserveSpace_i32_i32_i32_3(
        &mut self,
        id: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, i32), bool, 3usize>("ReserveSpace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReserveSpace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (id, width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetRequestedTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetRequestedTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetRequestedTexture",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TextureSizeToPowerOfTwo(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        width: quest_hook::libil2cpp::ByRefMut<i32>,
        height: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "TextureSizeToPowerOfTwo"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TextureSizeToPowerOfTwo",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (texture, width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
        mipPadding: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::FilterMode,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (_cordl_size, mipPadding, format, filterMode, name, useMipMap),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_mipPadding(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_mipPadding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_mipPadding",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum PowerOfTwoTextureAtlas_BlitType {
    #[default]
    OctahedralPadding = 2i32,
    OctahedralPaddingMultiply = 3i32,
    Padding = 0i32,
    PaddingMultiply = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "PowerOfTwoTextureAtlas/BlitType";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType
{
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+PowerOfTwoTextureAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::PowerOfTwoTextureAtlas_BlitType
{
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
