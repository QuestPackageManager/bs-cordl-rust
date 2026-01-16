#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct RTHandleSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_HardwareDynamicResRequested: bool,
    pub m_AutoSizedRTs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_AutoSizedRTsArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_ResizeOnDemandRTs: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_RTHandleProperties: crate::UnityEngine::Rendering::RTHandleProperties,
    pub m_MaxWidths: i32,
    pub m_MaxHeights: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::RTHandleSystem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "RTHandleSystem";
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
#[cfg(feature = "UnityEngine+Rendering+RTHandleSystem")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RTHandleSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RTHandleSystem")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RTHandleSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RTHandleSystem")]
impl crate::UnityEngine::Rendering::RTHandleSystem {
    #[cfg(feature = "UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
    pub type ResizeMode = crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode;
    pub fn AllocAutoSizedRenderTexture_RTHandleAllocInfo1(
        &mut self,
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
                    .find_method::<
                        (i32, i32, crate::UnityEngine::Rendering::RTHandleAllocInfo),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        3usize,
                    >("AllocAutoSizedRenderTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AllocAutoSizedRenderTexture", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> =
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn AllocAutoSizedRenderTexture_i32_GraphicsFormat_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString0(
        &mut self,
        width: i32,
        height: i32,
        slices: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
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
                    .find_method::<(
                        i32,
                        i32,
                        i32,
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
                        "AllocAutoSizedRenderTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AllocAutoSizedRenderTexture",
                            20usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    width,
                    height,
                    slices,
                    format,
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
    pub fn Alloc_RTHandle15(
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
    pub fn Alloc_RenderTargetIdentifier13(
        &mut self,
        texture: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
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
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_RenderTargetIdentifier_Il2CppString14(
        &mut self,
        texture: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (texture, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_RenderTexture__cordl_bool11(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        transferOwnership: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (texture, transferOwnership))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_ScaleFunc_GraphicsFormat_i32_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString9(
        &mut self,
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
                    .find_method::<(
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
                self,
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
    pub fn Alloc_ScaleFunc_RTHandleAllocInfo10(
        &mut self,
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
                    .find_method::<(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (scaleFunc, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_ScaleFunc_i32_DepthBits_GraphicsFormat_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString8(
        &mut self,
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
                    .find_method::<(
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
                self,
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
    pub fn Alloc_Texture12(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
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
            unsafe { cordl_method_info.invoke_unchecked(self, (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Vector2_GraphicsFormat_i32_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString5(
        &mut self,
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
                    .find_method::<(
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
                self,
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
    pub fn Alloc_Vector2_RTHandleAllocInfo7(
        &mut self,
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
                    .find_method::<(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (scaleFactor, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_Vector2_i32_DepthBits_GraphicsFormat_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString6(
        &mut self,
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
                    .find_method::<(
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
                self,
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
    pub fn Alloc_i32_i32_GraphicsFormat_TextureWrapMode_TextureWrapMode_TextureWrapMode_i32_FilterMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString3(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        wrapModeU: crate::UnityEngine::TextureWrapMode,
        wrapModeV: crate::UnityEngine::TextureWrapMode,
        wrapModeW: crate::UnityEngine::TextureWrapMode,
        slices: i32,
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
                    .find_method::<(
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::TextureWrapMode,
                        i32,
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
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>, 22usize>(
                        "Alloc",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Alloc",
                            22usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    width,
                    height,
                    format,
                    wrapModeU,
                    wrapModeV,
                    wrapModeW,
                    slices,
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
    pub fn Alloc_i32_i32_GraphicsFormat_i32_FilterMode_TextureWrapMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString1(
        &mut self,
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
                    .find_method::<(
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
                self,
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
    pub fn Alloc_i32_i32_RTHandleAllocInfo4(
        &mut self,
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
                    .find_method::<
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
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height, info))? };
        Ok(__cordl_ret.into())
    }
    pub fn Alloc_i32_i32_TextureWrapMode_TextureWrapMode_TextureWrapMode_i32_DepthBits_GraphicsFormat_FilterMode_TextureDimension__cordl_bool__cordl_bool__cordl_bool__cordl_bool_i32_f32_MSAASamples__cordl_bool__cordl_bool__cordl_bool_RenderTextureMemoryless_VRTextureUsage_Il2CppString2(
        &mut self,
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
                    .find_method::<(
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
                self,
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
        &mut self,
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
                    .find_method::<(
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
                self,
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
        &mut self,
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
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
            unsafe { cordl_method_info.invoke_unchecked(self, (scaleFunc))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateDimensions_Vector2_0(
        &mut self,
        scaleFactor: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
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
            unsafe { cordl_method_info.invoke_unchecked(self, (scaleFactor))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateRatioAgainstMaxSize(
        &mut self,
        viewportSize: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2Int>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Vector2Int,
                        >),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("CalculateRatioAgainstMaxSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateRatioAgainstMaxSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewportSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRenderTexture(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        slices: i32,
        filterMode: crate::UnityEngine::FilterMode,
        wrapModeU: crate::UnityEngine::TextureWrapMode,
        wrapModeV: crate::UnityEngine::TextureWrapMode,
        wrapModeW: crate::UnityEngine::TextureWrapMode,
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        i32,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        crate::UnityEngine::TextureWrapMode,
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
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>, 22usize>(
                        "CreateRenderTexture",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRenderTexture",
                            22usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    width,
                    height,
                    format,
                    slices,
                    filterMode,
                    wrapModeU,
                    wrapModeV,
                    wrapModeW,
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
    pub fn DemandResize(
        &mut self,
        rth: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DemandResize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DemandResize", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rth))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (disposing))? };
        Ok(__cordl_ret.into())
    }
    pub fn DumpRTInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("DumpRTInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DumpRTInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetMaxHeight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxHeight",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetMaxWidth")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxWidth",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetStencilFormat(
        &mut self,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetStencilFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetStencilFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked(self, (depthStencilFormat))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize__cordl_bool1(
        &mut self,
        width: i32,
        height: i32,
        useLegacyDynamicResControl: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, bool), quest_hook::libil2cpp::Void, 3usize>(
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
            cordl_method_info.invoke_unchecked(self, (width, height, useLegacyDynamicResControl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize_i32_i32_0(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Initialize")
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
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
        rth: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
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
            unsafe { cordl_method_info.invoke_unchecked(self, (rth))? };
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        rth: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Remove")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Remove",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rth))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetReferenceSize(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        width: i32,
        height: i32,
        sizeChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, bool), quest_hook::libil2cpp::Void, 3usize>("Resize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Resize",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height, sizeChanged))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetHardwareDynamicResolutionState(
        &mut self,
        enableHWDynamicRes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (enableHWDynamicRes))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceSize__cordl_bool1(
        &mut self,
        width: i32,
        height: i32,
        reset: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32, bool), quest_hook::libil2cpp::Void, 3usize>(
                        "SetReferenceSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetReferenceSize",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height, reset))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetReferenceSize_i32_i32_0(
        &mut self,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
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
            unsafe { cordl_method_info.invoke_unchecked(self, (width, height))? };
        Ok(__cordl_ret.into())
    }
    pub fn SwitchResizeMode(
        &mut self,
        rth: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        mode: crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode,
                    ), quest_hook::libil2cpp::Void, 2usize>("SwitchResizeMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SwitchResizeMode",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rth, mode))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_rtHandleProperties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RTHandleProperties> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::Rendering::RTHandleProperties, 0usize>(
                        "get_rtHandleProperties",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_rtHandleProperties",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RTHandleProperties =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::RTHandleSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RTHandleSystem")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::RTHandleSystem {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RTHandleSystem")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::RTHandleSystem {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum RTHandleSystem_ResizeMode {
    #[default]
    Auto = 0i32,
    OnDemand = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "RTHandleSystem/ResizeMode";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RTHandleSystem+ResizeMode")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::RTHandleSystem_ResizeMode
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
