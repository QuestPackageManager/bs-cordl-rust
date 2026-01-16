#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalDrawSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct DecalDrawSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_EntityManager:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DecalEntityManager>,
    pub m_WorldToDecals: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    >,
    pub m_NormalToDecals: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    >,
    pub m_DecalLayerMasks: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub m_Sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub _overrideMaterial_k__BackingField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalDrawSystem")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalDrawSystem
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalDrawSystem";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalDrawSystem")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalDrawSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalDrawSystem")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::DecalDrawSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalDrawSystem")]
impl crate::UnityEngine::Rendering::Universal::DecalDrawSystem {
    pub fn DrawInstanced_ByRefMut1(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        decalDrawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("DrawInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawInstanced",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cameraData,
                    decalEntityChunk,
                    decalCachedChunk,
                    decalDrawCallChunk,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawInstanced_RasterCommandBuffer_i32_0(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        decalDrawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("DrawInstanced")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawInstanced",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    decalEntityChunk,
                    decalCachedChunk,
                    decalDrawCallChunk,
                    passIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Draw_ByRefMut1(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        decalDrawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("Draw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Draw",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cameraData,
                    decalEntityChunk,
                    decalCachedChunk,
                    decalDrawCallChunk,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Draw_RasterCommandBuffer_i32_0(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        decalDrawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("Draw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Draw",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    decalEntityChunk,
                    decalCachedChunk,
                    decalDrawCallChunk,
                    passIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_ByRefMut3(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::CameraData,
                    >), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_ByRefMut_DecalEntityChunk_DecalCachedChunk_DecalDrawCallChunk_i32_4(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        decalDrawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cameraData,
                    decalEntityChunk,
                    decalCachedChunk,
                    decalDrawCallChunk,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_CommandBuffer0(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_RasterCommandBuffer1(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_RasterCommandBuffer_DecalEntityChunk_DecalCachedChunk_DecalDrawCallChunk_i32_2(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        decalEntityChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
        >,
        decalCachedChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
        >,
        decalDrawCallChunk: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalCachedChunk,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawCallChunk,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 5usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    decalEntityChunk,
                    decalCachedChunk,
                    decalDrawCallChunk,
                    count,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
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
        sampler: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sampler, entityManager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        sampler: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        entityManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalEntityManager,
                        >,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (sampler, entityManager))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        0usize,
                    >("get_overrideMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_overrideMaterial", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideMaterial(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_overrideMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_overrideMaterial", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalDrawSystem")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalDrawSystem
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
