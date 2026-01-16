#[cfg(feature = "cordl_class_NoteCutParticlesEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutParticlesEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _sparklesPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _explosionPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _explosionCorePS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _explosionPrePassBloomPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _sparklesPSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _sparklesPSMainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _sparklesPSShapeModule: crate::UnityEngine::ParticleSystem_ShapeModule,
    pub _sparklesLifetimeMinMaxCurve: crate::UnityEngine::ParticleSystem_MinMaxCurve,
    pub _explosionPSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _explosionCorePSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _explosionCorePSMainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _explosionCorePSShapeModule: crate::UnityEngine::ParticleSystem_ShapeModule,
    pub _explosionPrePassBloomPSShapeModule: crate::UnityEngine::ParticleSystem_ShapeModule,
}
#[cfg(feature = "cordl_class_NoteCutParticlesEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NoteCutParticlesEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoteCutParticlesEffect";
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
#[cfg(feature = "NoteCutParticlesEffect")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutParticlesEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutParticlesEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutParticlesEffect {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutParticlesEffect")]
impl crate::GlobalNamespace::NoteCutParticlesEffect {
    pub fn Awake(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Awake",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SpawnParticles(
        &mut self,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        saberDir: crate::UnityEngine::Vector3,
        saberSpeed: f32,
        noteMovementVec: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color32,
        sparkleParticlesCount: i32,
        explosionParticlesCount: i32,
        lifetimeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                        f32,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Color32,
                        i32,
                        i32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 9usize>("SpawnParticles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SpawnParticles",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cutPoint,
                    cutNormal,
                    saberDir,
                    saberSpeed,
                    noteMovementVec,
                    color,
                    sparkleParticlesCount,
                    explosionParticlesCount,
                    lifetimeMultiplier,
                ),
            )?
        };
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
}
#[cfg(feature = "cordl_class_NoteCutParticlesEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NoteCutParticlesEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
