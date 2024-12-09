#[cfg(feature = "UnityEngine+HeaderAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub header: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+HeaderAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::HeaderAttribute => "UnityEngine"
    ."HeaderAttribute"
);
#[cfg(feature = "UnityEngine+HeaderAttribute")]
impl std::ops::Deref for crate::UnityEngine::HeaderAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HeaderAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::HeaderAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+HeaderAttribute")]
impl crate::UnityEngine::HeaderAttribute {
    pub fn New(
        header: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (header))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        header: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (header))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+HeaderAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::HeaderAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
