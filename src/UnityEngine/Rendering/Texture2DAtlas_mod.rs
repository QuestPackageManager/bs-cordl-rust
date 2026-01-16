#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas")]
#[repr(C)]
#[derive(Debug)]
pub struct Texture2DAtlas {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AtlasTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RTHandle,
    >,
    pub m_Width: i32,
    pub m_Height: i32,
    pub m_Format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub m_UseMipMaps: bool,
    pub m_IsAtlasTextureOwner: bool,
    pub m_AtlasAllocator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::AtlasAllocator,
    >,
    pub m_AllocationCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::System::ValueTuple_2<
                crate::UnityEngine::Vector4,
                crate::UnityEngine::Vector2Int,
            >,
        >,
    >,
    pub m_IsGPUTextureUpToDate: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, i32>,
    >,
    pub m_TextureHashes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, i32>,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Texture2DAtlas {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "Texture2DAtlas";
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
#[cfg(feature = "UnityEngine+Rendering+Texture2DAtlas")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Texture2DAtlas {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Texture2DAtlas")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Texture2DAtlas {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Texture2DAtlas")]
impl crate::UnityEngine::Rendering::Texture2DAtlas {
    pub const kGPUTexInvalid: i32 = 0i32;
    pub const kGPUTexValidMip0: i32 = 1i32;
    pub const kGPUTexValidMipAll: i32 = 2i32;
    #[cfg(feature = "UnityEngine+Rendering+Texture2DAtlas+BlitType")]
    pub type BlitType = crate::UnityEngine::Rendering::Texture2DAtlas_BlitType;
    pub fn AddTexture(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        bool,
                        3usize,
                    >("AddTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddTexture", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (cmd, scaleOffset, texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocateTexture(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        width: i32,
        height: i32,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            i32,
                        ),
                        bool,
                        6usize,
                    >("AllocateTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cmd, scaleOffset, texture, width, height, overrideInstanceID),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocateTextureWithoutBlit_Texture0(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        width: i32,
        height: i32,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        ),
                        bool,
                        4usize,
                    >("AllocateTextureWithoutBlit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateTextureWithoutBlit", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (texture, width, height, scaleOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AllocateTextureWithoutBlit_i32_1(
        &mut self,
        instanceId: i32,
        width: i32,
        height: i32,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        ),
                        bool,
                        4usize,
                    >("AllocateTextureWithoutBlit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocateTextureWithoutBlit", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instanceId, width, height, scaleOffset))?
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
        blitType: crate::UnityEngine::Rendering::Texture2DAtlas_BlitType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Vector4,
                            bool,
                            crate::UnityEngine::Rendering::Texture2DAtlas_BlitType,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Blit2DTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Blit2DTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cmd, scaleOffset, texture, sourceScaleOffset, blitMips, blitType),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCubeTexture2D(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleOffset: crate::UnityEngine::Vector4,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        blitMips: bool,
        overrideInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("BlitCubeTexture2D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BlitCubeTexture2D", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cmd, scaleOffset, texture, blitMips, overrideInstanceID),
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Vector4,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("BlitOctahedralTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BlitOctahedralTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Vector4,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            crate::UnityEngine::Vector4,
                            bool,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BlitTexture", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
    pub fn ClearTarget(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ClearTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearTarget", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cmd))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTextureSlot(
        &mut self,
        isUploadNeeded: quest_hook::libil2cpp::ByRefMut<bool>,
        scaleBias: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        key: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            i32,
                            i32,
                            i32,
                        ),
                        bool,
                        5usize,
                    >("EnsureTextureSlot")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureTextureSlot", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (isUploadNeeded, scaleBias, key, width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedTextureSize(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Vector2Int,
                        1usize,
                    >("GetCachedTextureSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCachedTextureSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2Int = unsafe {
            cordl_method_info.invoke_unchecked(self, (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureHash(
        &mut self,
        textureA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        i32,
                        2usize,
                    >("GetTextureHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureHash", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureA, textureB))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureID_Texture0(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        i32,
                        1usize,
                    >("GetTextureID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureID", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureID_Texture1(
        &mut self,
        textureA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        i32,
                        2usize,
                    >("GetTextureID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureID", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureA, textureB))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureMipmapCount(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), i32, 2usize>("GetTextureMipmapCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTextureMipmapCount", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (width, height))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Is2D(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        bool,
                        1usize,
                    >("Is2D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Is2D",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCached_Texture1(
        &mut self,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        bool,
                        2usize,
                    >("IsCached")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCached", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (scaleOffset, texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCached_Texture_Texture0(
        &mut self,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        textureA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        bool,
                        3usize,
                    >("IsCached")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCached", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (scaleOffset, textureA, textureB))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsCached_i32_2(
        &mut self,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            i32,
                        ),
                        bool,
                        2usize,
                    >("IsCached")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsCached", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (scaleOffset, id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSingleChannelBlit(
        &mut self,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        ),
                        bool,
                        2usize,
                    >("IsSingleChannelBlit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSingleChannelBlit", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (source, destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkGPUTextureInvalid(
        &mut self,
        instanceId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("MarkGPUTextureInvalid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MarkGPUTextureInvalid", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkGPUTextureValid(
        &mut self,
        instanceId: i32,
        mipAreValid: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("MarkGPUTextureValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MarkGPUTextureValid", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceId, mipAreValid))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsUpdate_Texture_Texture__cordl_bool2(
        &mut self,
        textureA: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureB: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        needMips: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            bool,
                        ),
                        bool,
                        3usize,
                    >("NeedsUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsUpdate", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (textureA, textureB, needMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsUpdate_Texture__cordl_bool0(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        needMips: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>, bool),
                        bool,
                        2usize,
                    >("NeedsUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (texture, needMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsUpdate_i32_i32__cordl_bool1(
        &mut self,
        id: i32,
        updateCount: i32,
        needMips: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, bool), bool, 3usize>("NeedsUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsUpdate", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (id, updateCount, needMips))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        powerOfTwoPadding: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (width, height, format, filterMode, powerOfTwoPadding, name, useMipMap),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Release",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAllocator(
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
                    >("ResetAllocator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetAllocator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTexture_ByRefMut__cordl_bool__cordl_bool1(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        updateIfNeeded: bool,
        blitMips: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            bool,
                            bool,
                        ),
                        bool,
                        5usize,
                    >("UpdateTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateTexture", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cmd, texture, scaleOffset, updateIfNeeded, blitMips),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTexture_Texture_ByRefMut_Vector4__cordl_bool__cordl_bool0(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        oldTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        newTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleOffset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        sourceScaleOffset: crate::UnityEngine::Vector4,
        updateIfNeeded: bool,
        blitMips: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                            crate::UnityEngine::Vector4,
                            bool,
                            bool,
                        ),
                        bool,
                        7usize,
                    >("UpdateTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateTexture", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        cmd,
                        oldTexture,
                        newTexture,
                        scaleOffset,
                        sourceScaleOffset,
                        updateIfNeeded,
                        blitMips,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        powerOfTwoPadding: bool,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useMipMap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            i32,
                            crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                            crate::UnityEngine::FilterMode,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        width,
                        height,
                        format,
                        filterMode,
                        powerOfTwoPadding,
                        name,
                        useMipMap,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AtlasTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        0usize,
                    >("get_AtlasTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AtlasTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RTHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxMipLevelPadding() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_maxMipLevelPadding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_maxMipLevelPadding", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Texture2DAtlas {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas+BlitType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Texture2DAtlas_BlitType {
    #[default]
    CubeTo2DOctahedral = 1i32,
    CubeTo2DOctahedralSingleChannel = 3i32,
    Default = 0i32,
    SingleChannel = 2i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Texture2DAtlas_BlitType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "Texture2DAtlas/BlitType";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Texture2DAtlas_BlitType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Texture2DAtlas_BlitType {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Texture2DAtlas_BlitType {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Texture2DAtlas+BlitType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Texture2DAtlas_BlitType {
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
