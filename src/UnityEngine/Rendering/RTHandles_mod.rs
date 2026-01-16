#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandles")]
#[repr(C)]
#[derive(Debug)]
pub struct RTHandles {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandles")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::RTHandles {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "RTHandles";
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
#[cfg(feature = "UnityEngine+Rendering+RTHandles")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RTHandles {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RTHandles")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RTHandles {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RTHandles")]
impl crate::UnityEngine::Rendering::RTHandles {
    pub fn Alloc_ByRefMut_FilterMode_TextureWrapMode__cordl_bool_i32_f32_Il2CppString4(
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        bool,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 7usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    descriptor,
                    filterMode,
                    wrapMode,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_RTHandle17(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("Alloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Alloc",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (tex))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_RenderTargetIdentifier15(
        tex: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::RenderTargetIdentifier),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("Alloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Alloc",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (tex))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_RenderTargetIdentifier_Il2CppString16(
        tex: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 2usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (tex, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_RenderTexture__cordl_bool14(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        transferOwnership: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        bool,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 2usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (tex, transferOwnership))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_ScaleFunc_ByRefMut_FilterMode_TextureWrapMode__cordl_bool_i32_f32_Il2CppString11(
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        bool,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 8usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    scaleFunc,
                    descriptor,
                    filterMode,
                    wrapMode,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_ScaleFunc_GraphicsFormat_i32_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString10(
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        slices: i32,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 19usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            19usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    scaleFunc,
                    format,
                    slices,
                    filterMode,
                    wrapMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_ScaleFunc_RTHandleAllocInfo12(
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
        info: crate::UnityEngine::Rendering::RTHandleAllocInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
                        crate::UnityEngine::Rendering::RTHandleAllocInfo,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 2usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleFunc, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_ScaleFunc_i32_DepthBits_GraphicsFormat_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString9(
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
        slices: i32,
        depthBufferBits: crate::UnityEngine::Rendering::DepthBits,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
                        i32,
                        crate::UnityEngine::Rendering::DepthBits,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 20usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    scaleFunc,
                    slices,
                    depthBufferBits,
                    colorFormat,
                    filterMode,
                    wrapMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Texture13(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("Alloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Alloc",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (tex))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Vector2_ByRefMut_FilterMode_TextureWrapMode__cordl_bool_i32_f32_Il2CppString7(
        scaleFactor: crate::UnityEngine::Vector2,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector2,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        bool,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 8usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    scaleFactor,
                    descriptor,
                    filterMode,
                    wrapMode,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Vector2_GraphicsFormat_i32_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString6(
        scaleFactor: crate::UnityEngine::Vector2,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        slices: i32,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 19usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            19usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    scaleFactor,
                    format,
                    slices,
                    filterMode,
                    wrapMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Vector2_RTHandleAllocInfo8(
        scaleFactor: crate::UnityEngine::Vector2,
        info: crate::UnityEngine::Rendering::RTHandleAllocInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector2,
                        crate::UnityEngine::Rendering::RTHandleAllocInfo,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 2usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleFactor, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Vector2_i32_DepthBits_GraphicsFormat_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString5(
        scaleFactor: crate::UnityEngine::Vector2,
        slices: i32,
        depthBufferBits: crate::UnityEngine::Rendering::DepthBits,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Vector2,
                        i32,
                        crate::UnityEngine::Rendering::DepthBits,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 20usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    scaleFactor,
                    slices,
                    depthBufferBits,
                    colorFormat,
                    filterMode,
                    wrapMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_i32_i32_GraphicsFormat_i32_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString1(
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        slices: i32,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 20usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    width,
                    height,
                    format,
                    slices,
                    filterMode,
                    wrapMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_i32_i32_RTHandleAllocInfo3(
        width: i32,
        height: i32,
        info: crate::UnityEngine::Rendering::RTHandleAllocInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32, crate::UnityEngine::Rendering::RTHandleAllocInfo),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        3usize,
                    >("Alloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Alloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked((), (width, height, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_i32_i32_TextureWrapMode_TextureWrapMode_TextureWrapMode_i32_DepthBits_GraphicsFormat_FilterMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString2(
        width: i32,
        height: i32,
        wrapModeU: crate::UnityEngine::TextureWrapMode,
        wrapModeV: crate::UnityEngine::TextureWrapMode,
        wrapModeW: crate::UnityEngine::TextureWrapMode,
        slices: i32,
        depthBufferBits: crate::UnityEngine::Rendering::DepthBits,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        i32,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::TextureWrapMode,
                        i32,
                        crate::UnityEngine::Rendering::DepthBits,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 23usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            23usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    width,
                    height,
                    wrapModeU,
                    wrapModeV,
                    wrapModeW,
                    slices,
                    depthBufferBits,
                    colorFormat,
                    filterMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_i32_i32_i32_DepthBits_GraphicsFormat_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString0(
        width: i32,
        height: i32,
        slices: i32,
        depthBufferBits: crate::UnityEngine::Rendering::DepthBits,
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        dimension: crate::UnityEngine::Rendering::TextureDimension,
        enableRandomWrite: bool,
        useMipMap: bool,
        autoGenerateMips: bool,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        msaaSamples: crate::UnityEngine::Rendering::MSAASamples,
        bindTextureMS: bool,
        useDynamicScale: bool,
        useDynamicScaleExplicit: bool,
        memoryless: crate::UnityEngine::RenderTextureMemoryless,
        vrUsage: crate::UnityEngine::VRTextureUsage,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        i32,
                        i32,
                        i32,
                        crate::UnityEngine::Rendering::DepthBits,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::Rendering::TextureDimension,
                        bool,
                        bool,
                        bool,
                        bool,
                        i32,
                        f32,
                        crate::UnityEngine::Rendering::MSAASamples,
                        bool,
                        bool,
                        bool,
                        crate::UnityEngine::RenderTextureMemoryless,
                        crate::UnityEngine::VRTextureUsage,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 21usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            21usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    width,
                    height,
                    slices,
                    depthBufferBits,
                    colorFormat,
                    filterMode,
                    wrapMode,
                    dimension,
                    enableRandomWrite,
                    useMipMap,
                    autoGenerateMips,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    msaaSamples,
                    bindTextureMS,
                    useDynamicScale,
                    useDynamicScaleExplicit,
                    memoryless,
                    vrUsage,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateDimensions_ScaleFunc1(
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ScaleFunc,
                        >),
                        crate::UnityEngine::Vector2Int,
                        1usize,
                    >("CalculateDimensions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateDimensions", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2Int =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleFunc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateDimensions_Vector2_0(
        scaleFactor: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Vector2),
                        crate::UnityEngine::Vector2Int,
                        1usize,
                    >("CalculateDimensions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateDimensions", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2Int =
            unsafe { cordl_method_info.invoke_unchecked((), (scaleFactor))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateRatioAgainstMaxSize(
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), crate::UnityEngine::Vector2, 2usize>(
                        "CalculateRatioAgainstMaxSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateRatioAgainstMaxSize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked((), (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFormat(
        colorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    ), crate::UnityEngine::Experimental::Rendering::GraphicsFormat, 2usize>(
                        "GetFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFormat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (colorFormat, depthStencilFormat))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize__cordl_bool1(
        width: i32,
        height: i32,
        useLegacyDynamicResControl: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32, bool), quest_hook::libil2cpp::Void, 3usize>(
                        "Initialize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (width, height, useLegacyDynamicResControl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_i32_i32_0(
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "Initialize",
                    )
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
            unsafe { cordl_method_info.invoke_unchecked((), (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        rth: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Release")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Release",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (rth))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetReferenceSize(
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "ResetReferenceSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetReferenceSize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetHardwareDynamicResolutionState(
        hwDynamicResRequested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "SetHardwareDynamicResolutionState",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetHardwareDynamicResolutionState",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (hwDynamicResRequested))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceSize(
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "SetReferenceSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetReferenceSize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxHeight() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_maxHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_maxHeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_maxWidth() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("get_maxWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_maxWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_rtHandleProperties(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RTHandleProperties> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Rendering::RTHandleProperties,
                        0usize,
                    >("get_rtHandleProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_rtHandleProperties", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RTHandleProperties =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandles")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::RTHandles {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
