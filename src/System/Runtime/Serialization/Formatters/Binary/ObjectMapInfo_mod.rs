#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMapInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectMapInfo {
    __cordl_parent: crate::System::Object,
    pub objectId: i32,
    pub numMembers: i32,
    pub memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMapInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ObjectMapInfo =>
    "System.Runtime.Serialization.Formatters.Binary"."ObjectMapInfo"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMapInfo")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMapInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMapInfo")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMapInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMapInfo")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectMapInfo {
    pub fn _ctor(
        &mut self,
        objectId: i32,
        numMembers: i32,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (objectId, numMembers, memberNames, memberTypes))?;
        Ok(__cordl_ret)
    }
    pub fn isCompatible(
        &mut self,
        numMembers: i32,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("isCompatible", (numMembers, memberNames, memberTypes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        objectId: i32,
        numMembers: i32,
        memberNames: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        memberTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectId, numMembers, memberNames, memberTypes))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMapInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMapInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
