#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct KnucklesUnityXRHapticsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _hapticsCoroutine: quest_hook::libil2cpp::Gc<crate::UnityEngine::Coroutine>,
    pub _node: crate::UnityEngine::XR::XRNode,
    pub _remainingTime: f32,
    pub _amplitude: f32,
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "KnucklesUnityXRHapticsHandler";
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
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    pub const kRate: f32 = 0.0125f32;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::KnucklesUnityXRHapticsHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::KnucklesUnityXRHapticsHandler as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HapticsCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::KnucklesUnityXRHapticsHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("HapticsCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::KnucklesUnityXRHapticsHandler as
                    quest_hook::libil2cpp::Type > ::class(), "HapticsCoroutine", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        node: crate::UnityEngine::XR::XRNode,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node, coroutineRunner))?;
        Ok(__cordl_object.into())
    }
    pub fn StopHaptics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::KnucklesUnityXRHapticsHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("StopHaptics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::KnucklesUnityXRHapticsHandler as
                    quest_hook::libil2cpp::Type > ::class(), "StopHaptics", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TriggerHapticPulse(
        &mut self,
        strength: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::KnucklesUnityXRHapticsHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32, f32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("TriggerHapticPulse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::KnucklesUnityXRHapticsHandler as
                    quest_hook::libil2cpp::Type > ::class(), "TriggerHapticPulse", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (strength, duration))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::KnucklesUnityXRHapticsHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::KnucklesUnityXRHapticsHandler as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node, coroutineRunner))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl AsRef<crate::GlobalNamespace::IUnityXRHapticsHandler>
for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IUnityXRHapticsHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl AsMut<crate::GlobalNamespace::IUnityXRHapticsHandler>
for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IUnityXRHapticsHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::KnucklesUnityXRHapticsHandler {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
