#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UniversalRenderPipelineEditorResources {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub shaders: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources,
    >,
    pub materials: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "UniversalRenderPipelineEditorResources";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources
{
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources")]
impl crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
    )]
    pub type MaterialResources = crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
    )]
    pub type ShaderResources = crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources;
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UniversalRenderPipelineEditorResources_MaterialResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lit: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub particleLit: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub terrainLit: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub decal: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "UniversalRenderPipelineEditorResources/MaterialResources";
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
#[cfg(
    feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
)]
impl crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+MaterialResources"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_MaterialResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UniversalRenderPipelineEditorResources_ShaderResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub autodeskInteractivePS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub autodeskInteractiveTransparentPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub autodeskInteractiveMaskedPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub terrainDetailLitPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub terrainDetailGrassPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub terrainDetailGrassBillboardPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub defaultSpeedTree7PS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub defaultSpeedTree8PS: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "UniversalRenderPipelineEditorResources/ShaderResources";
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
#[cfg(
    feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
)]
impl
    crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources
{
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineEditorResources+ShaderResources"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineEditorResources_ShaderResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
