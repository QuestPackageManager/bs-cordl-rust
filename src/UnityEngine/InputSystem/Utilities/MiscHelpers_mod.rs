#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct MiscHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::MiscHelpers
    => "UnityEngine.InputSystem.Utilities"."MiscHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    pub fn EveryNth<TValue>(
        enumerable: quest_hook::libil2cpp::Gc<TValue>,
        n: i32,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TValue>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TValue> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EveryNth", (enumerable, n, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValueOrDefault<TKey, TValue>(
        dictionary: quest_hook::libil2cpp::Gc<TKey, TValue>,
        key: TKey,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetValueOrDefault", (dictionary, key))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<TValue>(
        enumerable: quest_hook::libil2cpp::Gc<TValue>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (enumerable, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+MiscHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::MiscHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
