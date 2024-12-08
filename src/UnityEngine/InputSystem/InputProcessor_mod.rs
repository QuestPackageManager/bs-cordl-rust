#[cfg(feature = "UnityEngine+InputSystem+InputProcessor+CachingPolicy")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputProcessor_CachingPolicy {
    CacheResult = 0i32,
    EvaluateOnEveryRead = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor+CachingPolicy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputProcessor_CachingPolicy => "UnityEngine.InputSystem"
    ."InputProcessor/CachingPolicy"
);
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct InputProcessor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputProcessor =>
    "UnityEngine.InputSystem"."InputProcessor"
);
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputProcessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor")]
impl crate::UnityEngine::InputSystem::InputProcessor {
    #[cfg(feature = "UnityEngine+InputSystem+InputProcessor+CachingPolicy")]
    pub type CachingPolicy = crate::UnityEngine::InputSystem::InputProcessor_CachingPolicy;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Process(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferSize: i32,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", (buffer, bufferSize, control))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAsObject(
        &mut self,
        value: *mut crate::System::Object,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ProcessAsObject", (value, control))?;
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
    pub fn get_cachingPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputProcessor_CachingPolicy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputProcessor_CachingPolicy = __cordl_object
            .invoke("get_cachingPolicy", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
