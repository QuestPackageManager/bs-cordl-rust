#[cfg(feature = "BeatEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spriteRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
    pub _spriteTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _tubeBloomPrePassLight: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
    pub _lightIntensityCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _spriteXScaleCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _spriteYScaleCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _transparencyCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LazyCopyHashSet_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatEffectDidFinishEvent>,
        >,
    >,
    pub _animationDuration: f32,
    pub _elapsedTime: f32,
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "BeatEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatEffect";
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
#[cfg(feature = "BeatEffect")]
impl std::ops::Deref for crate::GlobalNamespace::BeatEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect")]
impl crate::GlobalNamespace::BeatEffect {
    #[cfg(feature = "BeatEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::BeatEffect_Pool;
    pub fn Init(
        &mut self,
        color: crate::UnityEngine::Color,
        animationDuration: f32,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Color, f32, crate::UnityEngine::Quaternion),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (color, animationDuration, rotation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("ManualUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ManualUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (deltaTime))
        };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_didFinishEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatEffectDidFinishEvent,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ILazyCopyHashSet_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IBeatEffectDidFinishEvent,
                        >,
                    >,
                >,
                0usize,
            >("get_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_didFinishEvent", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILazyCopyHashSet_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatEffectDidFinishEvent,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatEffect>,
    >,
}
#[cfg(feature = "BeatEffect+Pool")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatEffect_Pool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatEffect/Pool";
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
#[cfg(feature = "BeatEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BeatEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatEffect>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatEffect_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect+Pool")]
impl crate::GlobalNamespace::BeatEffect_Pool {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatEffect+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatEffect_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
