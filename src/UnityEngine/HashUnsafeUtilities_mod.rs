#[cfg(feature = "UnityEngine+HashUnsafeUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct HashUnsafeUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+HashUnsafeUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HashUnsafeUtilities =>
    "UnityEngine"."HashUnsafeUtilities"
);
#[cfg(feature = "UnityEngine+HashUnsafeUtilities")]
impl std::ops::Deref for crate::UnityEngine::HashUnsafeUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HashUnsafeUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::HashUnsafeUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HashUnsafeUtilities")]
impl crate::UnityEngine::HashUnsafeUtilities {
    pub fn ComputeHash128_Il2CppObject0(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dataSize: u64,
        hash1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        hash2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeHash128", (data, dataSize, hash1, hash2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeHash128_Il2CppObject_u64_Il2CppObject1(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dataSize: u64,
        hash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeHash128", (data, dataSize, hash))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+HashUnsafeUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::HashUnsafeUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
