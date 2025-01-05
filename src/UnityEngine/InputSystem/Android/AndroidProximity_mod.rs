#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidProximity")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidProximity {
    __cordl_parent: crate::UnityEngine::InputSystem::ProximitySensor,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidProximity")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::AndroidProximity =>
    "UnityEngine.InputSystem.Android"."AndroidProximity"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidProximity")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Android::AndroidProximity {
    type Target = crate::UnityEngine::InputSystem::ProximitySensor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidProximity")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Android::AndroidProximity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidProximity")]
impl crate::UnityEngine::InputSystem::Android::AndroidProximity {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidProximity")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Android::AndroidProximity {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
