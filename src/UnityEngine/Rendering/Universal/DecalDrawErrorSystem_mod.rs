#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalDrawErrorSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct DecalDrawErrorSystem {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::DecalDrawSystem,
    pub m_Technique: crate::UnityEngine::Rendering::Universal::DecalTechnique,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalDrawErrorSystem")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalDrawErrorSystem
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalDrawErrorSystem";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalDrawErrorSystem")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalDrawErrorSystem {
    type Target = crate::UnityEngine::Rendering::Universal::DecalDrawSystem;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalDrawErrorSystem")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::DecalDrawErrorSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalDrawErrorSystem")]
impl crate::UnityEngine::Rendering::Universal::DecalDrawErrorSystem {
    pub fn GetMaterial(
        &mut self,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                    >), quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, 1usize>(
                        "GetMaterial",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaterial",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked(self, (decalEntityChunk))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPassIndex(
        &mut self,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                    >), i32, 1usize>("GetPassIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPassIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (decalCachedChunk))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
        technique: crate::UnityEngine::Rendering::Universal::DecalTechnique,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entityManager, technique))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
        technique: crate::UnityEngine::Rendering::Universal::DecalTechnique,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
                        >,
                        crate::UnityEngine::Rendering::Universal::DecalTechnique,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (entityManager, technique))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalDrawErrorSystem")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalDrawErrorSystem
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
