#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub shaders: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources,
    >,
    pub textures: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessData {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::PostProcessData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessData {
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
    pub type ShaderResources =
        crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
    pub type TextureResources =
        crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessData_ShaderResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub stopNanPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub subpixelMorphologicalAntialiasingPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub gaussianDepthOfFieldPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub bokehDepthOfFieldPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub cameraMotionBlurPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub paniniProjectionPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub lutBuilderLdrPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub lutBuilderHdrPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub bloomPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub temporalAntialiasingPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub LensFlareDataDrivenPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub LensFlareScreenSpacePS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub scalingSetupPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub easuPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub uberPostPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub finalPostPassPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessData/ShaderResources";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
impl crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData+ShaderResources")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessData_ShaderResources
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessData_TextureResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub blueNoise16LTex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    >,
    pub filmGrainTex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    >,
    pub smaaAreaTex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub smaaSearchTex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessData/TextureResources";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
impl crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessData+TextureResources")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessData_TextureResources
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
