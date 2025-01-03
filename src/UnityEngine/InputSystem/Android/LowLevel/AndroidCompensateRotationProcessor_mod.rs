#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateRotationProcessor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidCompensateRotationProcessor {
    __cordl_parent: crate::UnityEngine::InputSystem::Processors::CompensateRotationProcessor,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateRotationProcessor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateRotationProcessor =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidCompensateRotationProcessor"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateRotationProcessor"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateRotationProcessor {
    type Target = crate::UnityEngine::InputSystem::Processors::CompensateRotationProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateRotationProcessor"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateRotationProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateRotationProcessor"
)]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateRotationProcessor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Process(
        &mut self,
        value: crate::UnityEngine::Quaternion,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("Process", (value, control))?;
        Ok(__cordl_ret.into())
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
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateRotationProcessor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateRotationProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
