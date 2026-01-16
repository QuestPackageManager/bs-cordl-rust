#[cfg(feature = "cordl_class_TubeBloomPrePassLightReflectionEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLightReflectionEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _mainTubeBloomPrePassLight: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
    >,
    pub _tubeBloomPrePassLightBounces: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
            >,
        >,
    >,
    pub _environmentLayerMask: crate::UnityEngine::LayerMask,
    pub _environmentCollisionRepository: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentCollisionRepository,
    >,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "cordl_class_TubeBloomPrePassLightReflectionEffect")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TubeBloomPrePassLightReflectionEffect";
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
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl std::ops::Deref for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect {
    #[cfg(feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint")]
    pub type TubeBloomPrePassLightWithHitPoint = crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint;
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
    pub fn LateUpdate(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LateUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LateUpdate",
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
    pub fn RaycastAndSetLightLength(
        &mut self,
        bounce: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
        >,
        rayWorldOrigin: crate::UnityEngine::Vector3,
        rayDirection: crate::UnityEngine::Vector3,
        hitWorldPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        hitReflection: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        length: quest_hook::libil2cpp::ByRefMut<f32>,
        endAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
                            >,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                            quest_hook::libil2cpp::ByRefMut<f32>,
                        ),
                        bool,
                        7usize,
                    >("RaycastAndSetLightLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RaycastAndSetLightLength", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    bounce,
                    rayWorldOrigin,
                    rayDirection,
                    hitWorldPosition,
                    hitReflection,
                    length,
                    endAlpha,
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
#[cfg(feature = "cordl_class_TubeBloomPrePassLightReflectionEffect")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub light: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TubeBloomPrePassLight>,
    pub showHitPoint: bool,
    pub hitPointGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub hitPointTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub hitPointLightWithId:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::InstancedMaterialLightWithId>,
    pub hitPointDistanceToAlphaCurve: quest_hook::libil2cpp::Gc<crate::UnityEngine::AnimationCurve>,
    pub _hitPointState: bool,
}
#[cfg(
    feature = "cordl_class_TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TubeBloomPrePassLightReflectionEffect/TubeBloomPrePassLightWithHitPoint";
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
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint")]
impl std::ops::Deref
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint")]
impl
    crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint
{
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetActive(
        &mut self,
        enabled: bool,
        mainLightColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetActive", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (enabled, mainLightColor))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetCollisionLength(
        &mut self,
        rayHitGeometry: bool,
        hit: crate::UnityEngine::RaycastHit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, crate::UnityEngine::RaycastHit),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetCollisionLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCollisionLength", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rayHitGeometry, hit))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        remainingLength: f32,
        startAlpha: f32,
        hitWorldPosition: crate::UnityEngine::Vector3,
        hitReflection: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        f32,
                        f32,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Vector3,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetData",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (remainingLength, startAlpha, hitWorldPosition, hitReflection),
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
#[cfg(
    feature = "cordl_class_TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
