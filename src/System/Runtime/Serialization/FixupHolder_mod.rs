#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
#[repr(C)]
#[derive(Debug)]
pub struct FixupHolder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_id: i64,
    pub m_fixupInfo: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_fixupType: i32,
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Serialization::FixupHolder =>
    "System.Runtime.Serialization"."FixupHolder"
);
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl std::ops::Deref for crate::System::Runtime::Serialization::FixupHolder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::FixupHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl crate::System::Runtime::Serialization::FixupHolder {
    pub fn New(
        id: i64,
        fixupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fixupType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, fixupInfo, fixupType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        id: i64,
        fixupInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        fixupType: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, fixupInfo, fixupType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+FixupHolder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::FixupHolder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
