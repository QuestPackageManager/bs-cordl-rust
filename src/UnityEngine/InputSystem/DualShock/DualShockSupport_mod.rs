#[cfg(feature = "UnityEngine+InputSystem+DualShock+DualShockSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct DualShockSupport {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+DualShockSupport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DualShock::DualShockSupport =>
    "UnityEngine.InputSystem.DualShock"."DualShockSupport"
);
#[cfg(feature = "UnityEngine+InputSystem+DualShock+DualShockSupport")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::DualShock::DualShockSupport {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+DualShockSupport")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::DualShock::DualShockSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+DualShockSupport")]
impl crate::UnityEngine::InputSystem::DualShock::DualShockSupport {
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DualShock+DualShockSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::DualShock::DualShockSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
