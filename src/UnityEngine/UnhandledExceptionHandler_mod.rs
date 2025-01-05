#[cfg(feature = "UnityEngine+UnhandledExceptionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct UnhandledExceptionHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UnhandledExceptionHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UnhandledExceptionHandler =>
    "UnityEngine"."UnhandledExceptionHandler"
);
#[cfg(feature = "UnityEngine+UnhandledExceptionHandler")]
impl std::ops::Deref for crate::UnityEngine::UnhandledExceptionHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UnhandledExceptionHandler")]
impl std::ops::DerefMut for crate::UnityEngine::UnhandledExceptionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UnhandledExceptionHandler")]
impl crate::UnityEngine::UnhandledExceptionHandler {
    pub fn RegisterUECatcher() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterUECatcher", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UnhandledExceptionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UnhandledExceptionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
