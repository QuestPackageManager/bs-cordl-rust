#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicsFormatUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "GraphicsFormatUtility";
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
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl std::ops::Deref for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    pub fn CanDecompressFormat_GraphicsFormat1(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("CanDecompressFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CanDecompressFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn CanDecompressFormat__cordl_bool0(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        wholeImage: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        bool,
                    ), bool, 2usize>("CanDecompressFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanDecompressFormat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (format, wholeImage))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAlphaComponentCount(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        u32,
                        1usize,
                    >("GetAlphaComponentCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAlphaComponentCount", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBlockSize(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        u32,
                        1usize,
                    >("GetBlockSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBlockSize", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentCount(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        u32,
                        1usize,
                    >("GetComponentCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetComponentCount", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthBits(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        i32,
                        1usize,
                    >("GetDepthBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDepthBits", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthStencilFormatFromBitsLegacy_Native(
        minimumDepthBits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetDepthStencilFormatFromBitsLegacy_Native")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDepthStencilFormatFromBitsLegacy_Native", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (minimumDepthBits))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthStencilFormat_i32_0(
        depthBits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetDepthStencilFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDepthStencilFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (depthBits))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDepthStencilFormat_i32_1(
        minimumDepthBits: i32,
        minimumStencilBits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32, i32),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        2usize,
                    >("GetDepthStencilFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDepthStencilFormat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = unsafe {
            cordl_method_info.invoke_unchecked((), (minimumDepthBits, minimumStencilBits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFormat(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFormatString(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetFormatString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFormatString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFormatString_Injected(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "GetFormatString_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFormatString_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (format, ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFormat_Injected(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetFormat_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFormat_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_Native_RenderTextureFormat(
        format: crate::UnityEngine::RenderTextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::RenderTextureFormat, bool),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        2usize,
                    >("GetGraphicsFormat_Native_RenderTextureFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGraphicsFormat_Native_RenderTextureFormat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format, isSRGB))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_Native_TextureFormat(
        format: crate::UnityEngine::TextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextureFormat, bool),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        2usize,
                    >("GetGraphicsFormat_Native_TextureFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGraphicsFormat_Native_TextureFormat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format, isSRGB))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_RenderTextureFormat_RenderTextureReadWrite2(
        format: crate::UnityEngine::RenderTextureFormat,
        readWrite: crate::UnityEngine::RenderTextureReadWrite,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::RenderTextureFormat,
                        crate::UnityEngine::RenderTextureReadWrite,
                    ), crate::UnityEngine::Experimental::Rendering::GraphicsFormat, 2usize>(
                        "GetGraphicsFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetGraphicsFormat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format, readWrite))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_RenderTextureFormat__cordl_bool1(
        format: crate::UnityEngine::RenderTextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::RenderTextureFormat, bool),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        2usize,
                    >("GetGraphicsFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGraphicsFormat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format, isSRGB))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsFormat_TextureFormat__cordl_bool0(
        format: crate::UnityEngine::TextureFormat,
        isSRGB: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::TextureFormat, bool),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        2usize,
                    >("GetGraphicsFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGraphicsFormat", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format, isSRGB))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLinearFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetLinearFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLinearFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderTextureFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureFormat> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::RenderTextureFormat,
                        1usize,
                    >("GetRenderTextureFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRenderTextureFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSRGBFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Experimental::Rendering::GraphicsFormat>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetSRGBFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSRGBFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSwizzleA(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::FormatSwizzle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Rendering::FormatSwizzle,
                        1usize,
                    >("GetSwizzleA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSwizzleA", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::FormatSwizzle =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSwizzleB(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::FormatSwizzle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Rendering::FormatSwizzle,
                        1usize,
                    >("GetSwizzleB")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSwizzleB", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::FormatSwizzle =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSwizzleG(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::FormatSwizzle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Rendering::FormatSwizzle,
                        1usize,
                    >("GetSwizzleG")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSwizzleG", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::FormatSwizzle =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSwizzleR(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::FormatSwizzle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        crate::UnityEngine::Rendering::FormatSwizzle,
                        1usize,
                    >("GetSwizzleR")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSwizzleR", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::FormatSwizzle =
            unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasAlphaChannel(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("HasAlphaChannel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasAlphaChannel", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsAlphaOnlyFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsAlphaOnlyFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsAlphaOnlyFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsCompressedFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::TextureFormat), bool, 1usize>(
                        "IsCompressedFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsCompressedFormat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsCompressedFormat_Native_TextureFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::TextureFormat), bool, 1usize>(
                        "IsCompressedFormat_Native_TextureFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsCompressedFormat_Native_TextureFormat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsCrunchFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::TextureFormat), bool, 1usize>(
                        "IsCrunchFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsCrunchFormat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsDepthFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsDepthFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsDepthFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsDepthStencilFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsDepthStencilFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsDepthStencilFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsFloatFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsFloatFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsFloatFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHalfFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsHalfFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsHalfFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsPVRTCFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsPVRTCFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsPVRTCFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSRGBFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsSRGBFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsSRGBFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsStencilFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsStencilFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsStencilFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
