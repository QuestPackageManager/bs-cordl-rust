#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateDirectionProcessor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidCompensateDirectionProcessor {
    __cordl_parent: crate::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor,
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateDirectionProcessor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateDirectionProcessor =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidCompensateDirectionProcessor"
);
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateDirectionProcessor"
)]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateDirectionProcessor {
    type Target = crate::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateDirectionProcessor"
)]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateDirectionProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateDirectionProcessor"
)]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateDirectionProcessor {
    pub const kAccelerationMultiplier: f32 = -0.10197162f32;
    pub const kSensorStandardGravity: f32 = 9.80665f32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Process(
        &mut self,
        vector: crate::UnityEngine::Vector3,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("Process", (vector, control))?;
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
    feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidCompensateDirectionProcessor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidCompensateDirectionProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
