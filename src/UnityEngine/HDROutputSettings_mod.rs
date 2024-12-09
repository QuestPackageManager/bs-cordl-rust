#[cfg(feature = "UnityEngine+HDROutputSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct HDROutputSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_DisplayIndex: i32,
}
#[cfg(feature = "UnityEngine+HDROutputSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HDROutputSettings => "UnityEngine"
    ."HDROutputSettings"
);
#[cfg(feature = "UnityEngine+HDROutputSettings")]
impl std::ops::Deref for crate::UnityEngine::HDROutputSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HDROutputSettings")]
impl std::ops::DerefMut for crate::UnityEngine::HDROutputSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HDROutputSettings")]
impl crate::UnityEngine::HDROutputSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "UnityEngine+HDROutputSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::HDROutputSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
