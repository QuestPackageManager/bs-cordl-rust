#[cfg(feature = "UnityEngine+InputSystem+Processors+CompensateDirectionProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct CompensateDirectionProcessor {
    __cordl_parent: crate::UnityEngine::InputSystem::InputProcessor_1<
        crate::UnityEngine::Vector3,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+CompensateDirectionProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor =>
    "UnityEngine.InputSystem.Processors"."CompensateDirectionProcessor"
);
#[cfg(feature = "UnityEngine+InputSystem+Processors+CompensateDirectionProcessor")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor {
    type Target = crate::UnityEngine::InputSystem::InputProcessor_1<
        crate::UnityEngine::Vector3,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+CompensateDirectionProcessor")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+CompensateDirectionProcessor")]
impl crate::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Process(
        &mut self,
        value: crate::UnityEngine::Vector3,
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("Process", (value, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Processors+CompensateDirectionProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Processors::CompensateDirectionProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
