#[cfg(feature = "UnityEngine+InputSystem+Processors+NormalizeProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct NormalizeProcessor {
    __cordl_parent: crate::UnityEngine::InputSystem::InputProcessor_1<f32>,
    pub min: f32,
    pub max: f32,
    pub zero: f32,
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+NormalizeProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Processors::NormalizeProcessor =>
    "UnityEngine.InputSystem.Processors"."NormalizeProcessor"
);
#[cfg(feature = "UnityEngine+InputSystem+Processors+NormalizeProcessor")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Processors::NormalizeProcessor {
    type Target = crate::UnityEngine::InputSystem::InputProcessor_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+NormalizeProcessor")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Processors::NormalizeProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+NormalizeProcessor")]
impl crate::UnityEngine::InputSystem::Processors::NormalizeProcessor {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Process(
        &mut self,
        value: f32,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("Process", (value, control))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+NormalizeProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Processors::NormalizeProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}