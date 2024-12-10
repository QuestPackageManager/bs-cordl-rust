#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct KnucklesUnityXRHapticsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _coroutineRunner: *mut crate::UnityEngine::MonoBehaviour,
    pub _hapticsCoroutine: *mut crate::UnityEngine::Coroutine,
    pub _node: crate::UnityEngine::XR::XRNode,
    pub _remainingTime: f32,
    pub _amplitude: f32,
}
#[cfg(feature = "KnucklesUnityXRHapticsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::KnucklesUnityXRHapticsHandler
    => ""."KnucklesUnityXRHapticsHandler"
);
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
    #[cfg(feature = "KnucklesUnityXRHapticsHandler+_HapticsCoroutine_d__9")]
    pub type _HapticsCoroutine_d__9 = crate::GlobalNamespace::KnucklesUnityXRHapticsHandler__HapticsCoroutine_d__9;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HapticsCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("HapticsCoroutine", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopHaptics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TriggerHapticPulse(
        &mut self,
        strength: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerHapticPulse", (strength, duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, coroutineRunner))?;
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
