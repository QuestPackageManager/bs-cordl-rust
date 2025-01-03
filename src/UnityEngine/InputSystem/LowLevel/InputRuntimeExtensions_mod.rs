#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRuntimeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions =>
    "UnityEngine.InputSystem.LowLevel"."InputRuntimeExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    pub fn DeviceCommand<TCommand>(
        runtime: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::LowLevel::IInputRuntime,
        >,
        deviceId: i32,
        command: quest_hook::libil2cpp::ByRefMut<TCommand>,
    ) -> quest_hook::libil2cpp::Result<i64>
    where
        TCommand: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeviceCommand", (runtime, deviceId, command))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputRuntimeExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputRuntimeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
