#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputArrayExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::InputArrayExtensions =>
    "UnityEngine.InputSystem.Utilities"."InputArrayExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    pub fn Contains<TValue>(
        array: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsReference<TValue>(
        array: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsReference", (array, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfReference<TValue>(
        array: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfReference", (array, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
