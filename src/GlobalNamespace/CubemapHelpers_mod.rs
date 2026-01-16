#[cfg(feature = "cordl_class_CubemapHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct CubemapHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_CubemapHelpers")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::CubemapHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "CubemapHelpers";
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
#[cfg(feature = "CubemapHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::CubemapHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::CubemapHelpers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CubemapHelpers")]
impl crate::GlobalNamespace::CubemapHelpers {
    pub const kCubemapDownsamplePass: i32 = 0i32;
    pub const kCubemapHelpersShaderName: &'static str = "Hidden/CubemapHelpers";
    pub const kCubemapTo2DTexturePass: i32 = 1i32;
    pub fn Create2DTextureFromCubemap(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        1usize,
                    >("Create2DTextureFromCubemap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Create2DTextureFromCubemap", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> =
            unsafe { cordl_method_info.invoke_unchecked((), (src))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDownsampledCubemap(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                        i32,
                    ), quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>, 2usize>(
                        "CreateDownsampledCubemap",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDownsampledCubemap",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture> =
            unsafe { cordl_method_info.invoke_unchecked((), (src, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn Downsample(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Downsample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Downsample",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (src, dest))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawCubemapFace(
        cubemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        cubemapFace: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::CubemapFace,
                    ), quest_hook::libil2cpp::Void, 2usize>("DrawCubemapFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawCubemapFace",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cubemap, cubemapFace))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cubemapHelpersMaterial(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        0usize,
                    >("get_cubemapHelpersMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cubemapHelpersMaterial", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_CubemapHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CubemapHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
