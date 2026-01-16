#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter")]
#[repr(C)]
#[derive(Debug)]
pub struct Blitter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::Blitter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "Blitter";
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
#[cfg(feature = "UnityEngine+Rendering+Blitter")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Blitter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Blitter")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Blitter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Blitter")]
impl crate::UnityEngine::Rendering::Blitter {
    #[cfg(feature = "UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
    pub type BlitColorAndDepthPassNames =
        crate::UnityEngine::Rendering::Blitter_BlitColorAndDepthPassNames;
    #[cfg(feature = "UnityEngine+Rendering+Blitter+BlitShaderIDs")]
    pub type BlitShaderIDs = crate::UnityEngine::Rendering::Blitter_BlitShaderIDs;
    #[cfg(feature = "UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
    pub type BlitShaderPassNames = crate::UnityEngine::Rendering::Blitter_BlitShaderPassNames;
    pub fn BlitCameraTexture2D(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "BlitCameraTexture2D"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCameraTexture2D",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, source, destination, mipLevel, bilinear))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCameraTexture_Material_i32_1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitCameraTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCameraTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, destination, material, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCameraTexture_Rect_f32__cordl_bool4(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destViewport: crate::UnityEngine::Rect,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rect,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("BlitCameraTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCameraTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (cmd, source, destination, destViewport, mipLevel, bilinear),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCameraTexture_RenderBufferLoadAction_RenderBufferStoreAction_Material_i32_2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>("BlitCameraTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCameraTexture",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    destination,
                    loadAction,
                    storeAction,
                    material,
                    pass,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCameraTexture_Vector4_f32__cordl_bool3(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("BlitCameraTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCameraTexture",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (cmd, source, destination, scaleBias, mipLevel, bilinear),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCameraTexture_f32__cordl_bool0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitCameraTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCameraTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, source, destination, mipLevel, bilinear))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitColorAndDepth_CommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        sourceColor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceDepth: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        blitDepth: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("BlitColorAndDepth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitColorAndDepth",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    sourceColor,
                    sourceDepth,
                    scaleBias,
                    mipLevel,
                    blitDepth,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitColorAndDepth_RasterCommandBuffer0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        sourceColor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        sourceDepth: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        blitDepth: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("BlitColorAndDepth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitColorAndDepth",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    sourceColor,
                    sourceDepth,
                    scaleBias,
                    mipLevel,
                    blitDepth,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCubeToOctahedral2DQuad(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "BlitCubeToOctahedral2DQuad"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCubeToOctahedral2DQuad",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBiasRT, mipLevelTex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCubeToOctahedral2DQuadSingleChannel(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "BlitCubeToOctahedral2DQuadSingleChannel",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCubeToOctahedral2DQuadSingleChannel",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBiasRT, mipLevelTex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitCubeToOctahedral2DQuadWithPadding(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureSize: crate::UnityEngine::Vector2,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
        bilinear: bool,
        paddingInPixels: i32,
        decodeInstructions: crate::System::Nullable_1<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector4,
                        i32,
                        bool,
                        i32,
                        crate::System::Nullable_1<crate::UnityEngine::Vector4>,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "BlitCubeToOctahedral2DQuadWithPadding"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitCubeToOctahedral2DQuadWithPadding",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    textureSize,
                    scaleBiasRT,
                    mipLevelTex,
                    bilinear,
                    paddingInPixels,
                    decodeInstructions,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitOctahedralWithPadding(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureSize: crate::UnityEngine::Vector2,
        scaleBiasTex: crate::UnityEngine::Vector4,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
        bilinear: bool,
        paddingInPixels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector4,
                        crate::UnityEngine::Vector4,
                        i32,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "BlitOctahedralWithPadding"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitOctahedralWithPadding",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    textureSize,
                    scaleBiasTex,
                    scaleBiasRT,
                    mipLevelTex,
                    bilinear,
                    paddingInPixels,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitOctahedralWithPaddingMultiply(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureSize: crate::UnityEngine::Vector2,
        scaleBiasTex: crate::UnityEngine::Vector4,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
        bilinear: bool,
        paddingInPixels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector4,
                        crate::UnityEngine::Vector4,
                        i32,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "BlitOctahedralWithPaddingMultiply"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitOctahedralWithPaddingMultiply",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    textureSize,
                    scaleBiasTex,
                    scaleBiasRT,
                    mipLevelTex,
                    bilinear,
                    paddingInPixels,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitQuad(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleBiasTex: crate::UnityEngine::Vector4,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        crate::UnityEngine::Vector4,
                        i32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("BlitQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitQuad",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    scaleBiasTex,
                    scaleBiasRT,
                    mipLevelTex,
                    bilinear,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitQuadSingleChannel(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleBiasTex: crate::UnityEngine::Vector4,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector4,
                        crate::UnityEngine::Vector4,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "BlitQuadSingleChannel"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitQuadSingleChannel",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, source, scaleBiasTex, scaleBiasRT, mipLevelTex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitQuadWithPadding(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureSize: crate::UnityEngine::Vector2,
        scaleBiasTex: crate::UnityEngine::Vector4,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
        bilinear: bool,
        paddingInPixels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector4,
                        crate::UnityEngine::Vector4,
                        i32,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "BlitQuadWithPadding"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitQuadWithPadding",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    textureSize,
                    scaleBiasTex,
                    scaleBiasRT,
                    mipLevelTex,
                    bilinear,
                    paddingInPixels,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitQuadWithPaddingMultiply(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureSize: crate::UnityEngine::Vector2,
        scaleBiasTex: crate::UnityEngine::Vector4,
        scaleBiasRT: crate::UnityEngine::Vector4,
        mipLevelTex: i32,
        bilinear: bool,
        paddingInPixels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Vector4,
                        crate::UnityEngine::Vector4,
                        i32,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>(
                        "BlitQuadWithPaddingMultiply"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitQuadWithPaddingMultiply",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    textureSize,
                    scaleBiasTex,
                    scaleBiasRT,
                    mipLevelTex,
                    bilinear,
                    paddingInPixels,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture2D_CommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture2D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture2D",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, mipLevel, bilinear))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture2D_RasterCommandBuffer0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture2D")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture2D",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, mipLevel, bilinear))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RTHandle_Vector4_Material_i32_5(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, material, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RTHandle_Vector4_Material_i32_f32_i32_1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        sourceMipLevel: f32,
        sourceDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        f32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    scaleBias,
                    material,
                    pass,
                    sourceMipLevel,
                    sourceDepthSlice,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RTHandle_Vector4_f32__cordl_bool3(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, mipLevel, bilinear))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RTHandle_Vector4_f32_i32__cordl_bool0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        sourceMipLevel: f32,
        sourceDepthSlice: i32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        f32,
                        i32,
                        bool,
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
                (),
                (
                    cmd,
                    source,
                    scaleBias,
                    sourceMipLevel,
                    sourceDepthSlice,
                    bilinear,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RenderTargetIdentifier_RenderTargetIdentifier_Material_i32_8(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        destination: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, destination, material, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RenderTargetIdentifier_RenderTargetIdentifier_RenderBufferLoadAction_RenderBufferStoreAction_Material_i32_9(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        destination: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    destination,
                    loadAction,
                    storeAction,
                    material,
                    pass,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_RenderTargetIdentifier_Vector4_Material_i32_7(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, material, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_CommandBuffer_Vector4_Material_i32_10(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, scaleBias, material, pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_RasterCommandBuffer_RTHandle_Vector4_Material_i32_4(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, material, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_RasterCommandBuffer_RTHandle_Vector4_f32__cordl_bool2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        scaleBias: crate::UnityEngine::Vector4,
        mipLevel: f32,
        bilinear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Vector4,
                        f32,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, mipLevel, bilinear))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_RasterCommandBuffer_RenderTargetIdentifier_Vector4_Material_i32_6(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, source, scaleBias, material, pass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BlitTexture_RasterCommandBuffer_Vector4_Material_i32_11(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        scaleBias: crate::UnityEngine::Vector4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        crate::UnityEngine::Vector4,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 4usize>("BlitTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BlitTexture",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, scaleBias, material, pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn CanCopyMSAA() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("CanCopyMSAA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanCopyMSAA",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        isMSAA: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>("CopyTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyTexture",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, isMSAA))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawQuadMesh(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        propertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawQuadMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawQuadMesh",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass, propertyBlock))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawQuad_CommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawQuad",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawQuad_CommandBuffer_MaterialPropertyBlock2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        propertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawQuad",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass, propertyBlock))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawQuad_RasterCommandBuffer_MaterialPropertyBlock0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        propertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawQuad")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawQuad",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass, propertyBlock))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawTriangle_CommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawTriangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawTriangle",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawTriangle_CommandBuffer_MaterialPropertyBlock2(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        propertyBlock: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawTriangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawTriangle",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass, propertyBlock))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawTriangle_RasterCommandBuffer0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawTriangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawTriangle",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, material, shaderPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBlitMaterial(
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        singleSlice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::TextureDimension, bool),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        2usize,
                    >("GetBlitMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBlitMaterial", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), (dimension, singleSlice))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        blitPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
        blitColorAndDepthPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (blitPS, blitColorAndDepthPS))? };
        Ok(__cordl_ret.into())
    }
    pub fn _Initialize_g__GetFullScreenTriangleTexCoord_14_1() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >, 0usize>(
                        "<Initialize>g__GetFullScreenTriangleTexCoord|14_1"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<Initialize>g__GetFullScreenTriangleTexCoord|14_1",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _Initialize_g__GetFullScreenTriangleVertexPosition_14_0(
        z: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >, 1usize>(
                        "<Initialize>g__GetFullScreenTriangleVertexPosition|14_0"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<Initialize>g__GetFullScreenTriangleVertexPosition|14_0",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (z))? };
        Ok(__cordl_ret.into())
    }
    pub fn _Initialize_g__GetQuadTexCoord_14_3() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >, 0usize>("<Initialize>g__GetQuadTexCoord|14_3")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<Initialize>g__GetQuadTexCoord|14_3",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _Initialize_g__GetQuadVertexPosition_14_2(
        z: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                    >, 1usize>("<Initialize>g__GetQuadVertexPosition|14_2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<Initialize>g__GetQuadVertexPosition|14_2",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (z))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::Blitter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Blitter_BlitColorAndDepthPassNames {
    #[default]
    ColorAndDepth = 1i32,
    ColorOnly = 0i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Blitter_BlitColorAndDepthPassNames
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "Blitter/BlitColorAndDepthPassNames";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Blitter_BlitColorAndDepthPassNames
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Blitter_BlitColorAndDepthPassNames
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Blitter_BlitColorAndDepthPassNames
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitColorAndDepthPassNames")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Blitter_BlitColorAndDepthPassNames
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderIDs")]
#[repr(C)]
#[derive(Debug)]
pub struct Blitter_BlitShaderIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderIDs")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::Blitter_BlitShaderIDs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "Blitter/BlitShaderIDs";
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
#[cfg(feature = "UnityEngine+Rendering+Blitter+BlitShaderIDs")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Blitter_BlitShaderIDs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Blitter+BlitShaderIDs")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Blitter_BlitShaderIDs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Blitter+BlitShaderIDs")]
impl crate::UnityEngine::Rendering::Blitter_BlitShaderIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderIDs")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::Blitter_BlitShaderIDs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Blitter_BlitShaderPassNames {
    #[default]
    Bilinear = 1i32,
    BilinearCubeToOctahedralPadding = 22i32,
    BilinearQuad = 3i32,
    BilinearQuadAlpha = 19i32,
    BilinearQuadLuminance = 18i32,
    BilinearQuadPadding = 5i32,
    BilinearQuadPaddingAlphaBlend = 10i32,
    BilinearQuadPaddingAlphaBlendOctahedral = 13i32,
    BilinearQuadPaddingAlphaBlendRepeat = 12i32,
    BilinearQuadPaddingOctahedral = 8i32,
    BilinearQuadPaddingRepeat = 7i32,
    BilinearQuadRed = 20i32,
    CubeToOctahedral = 14i32,
    CubeToOctahedralAlpha = 16i32,
    CubeToOctahedralLuminance = 15i32,
    CubeToOctahedralRed = 17i32,
    Nearest = 0i32,
    NearestCubeToOctahedralPadding = 21i32,
    NearestQuad = 2i32,
    NearestQuadPadding = 4i32,
    NearestQuadPaddingAlphaBlend = 9i32,
    NearestQuadPaddingAlphaBlendRepeat = 11i32,
    NearestQuadPaddingRepeat = 6i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Blitter_BlitShaderPassNames
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "Blitter/BlitShaderPassNames";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Blitter_BlitShaderPassNames
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Blitter_BlitShaderPassNames
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Blitter_BlitShaderPassNames
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Blitter+BlitShaderPassNames")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Blitter_BlitShaderPassNames
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
