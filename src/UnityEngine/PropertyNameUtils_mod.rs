#[cfg(feature = "UnityEngine+PropertyNameUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyNameUtils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PropertyNameUtils => "UnityEngine"
    ."PropertyNameUtils"
);
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl std::ops::Deref for crate::UnityEngine::PropertyNameUtils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl std::ops::DerefMut for crate::UnityEngine::PropertyNameUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl crate::UnityEngine::PropertyNameUtils {
    pub fn PropertyNameFromString(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_ret: crate::UnityEngine::PropertyName = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PropertyNameFromString", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn PropertyNameFromString_Injected(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PropertyName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PropertyNameFromString_Injected", (name, ret))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+PropertyNameUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::PropertyNameUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
