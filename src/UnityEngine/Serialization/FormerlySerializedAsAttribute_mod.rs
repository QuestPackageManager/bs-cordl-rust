#[cfg(feature = "UnityEngine+Serialization+FormerlySerializedAsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct FormerlySerializedAsAttribute {
    __cordl_parent: crate::System::Attribute,
    pub m_oldName: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "UnityEngine+Serialization+FormerlySerializedAsAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Serialization::FormerlySerializedAsAttribute =>
    "UnityEngine.Serialization"."FormerlySerializedAsAttribute"
);
#[cfg(feature = "UnityEngine+Serialization+FormerlySerializedAsAttribute")]
impl std::ops::Deref
for crate::UnityEngine::Serialization::FormerlySerializedAsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Serialization+FormerlySerializedAsAttribute")]
impl std::ops::DerefMut
for crate::UnityEngine::Serialization::FormerlySerializedAsAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Serialization+FormerlySerializedAsAttribute")]
impl crate::UnityEngine::Serialization::FormerlySerializedAsAttribute {
    pub fn New(
        oldName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oldName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        oldName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oldName))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Serialization+FormerlySerializedAsAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Serialization::FormerlySerializedAsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
