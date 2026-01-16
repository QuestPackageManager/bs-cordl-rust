#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UniversalRenderPipelineRuntimeTextures {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Version: i32,
    pub m_BlueNoise64LTex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub m_BayerMatrixTex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub m_DebugFontTex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "UniversalRenderPipelineRuntimeTextures";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_Rendering_IRenderPipelineGraphicsSettings_get_isAvailableInPlayerBuild(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >(
                        "UnityEngine.Rendering.IRenderPipelineGraphicsSettings.get_isAvailableInPlayerBuild",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.Rendering.IRenderPipelineGraphicsSettings.get_isAvailableInPlayerBuild",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn get_bayerMatrixTex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        0usize,
                    >("get_bayerMatrixTex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_bayerMatrixTex", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_blueNoise64LTex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        0usize,
                    >("get_blueNoise64LTex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_blueNoise64LTex", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_debugFontTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                        0usize,
                    >("get_debugFontTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_debugFontTexture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_version")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_version",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_bayerMatrixTex(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_bayerMatrixTex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_bayerMatrixTex", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_blueNoise64LTex(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_blueNoise64LTex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_blueNoise64LTex", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_debugFontTexture(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_debugFontTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_debugFontTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl AsRef<crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings>
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl AsMut<crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings>
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl AsRef<crate::UnityEngine::Rendering::IRenderPipelineResources>
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IRenderPipelineResources {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineRuntimeTextures")]
impl AsMut<crate::UnityEngine::Rendering::IRenderPipelineResources>
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineRuntimeTextures
{
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Rendering::IRenderPipelineResources {
        unsafe { std::mem::transmute(self) }
    }
}
