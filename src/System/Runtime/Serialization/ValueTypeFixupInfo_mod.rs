#[cfg(feature = "System+Runtime+Serialization+ValueTypeFixupInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ValueTypeFixupInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _containerID: i64,
    pub _parentField: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    pub _parentIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "System+Runtime+Serialization+ValueTypeFixupInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::ValueTypeFixupInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "ValueTypeFixupInfo";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+Serialization+ValueTypeFixupInfo")]
impl std::ops::Deref for crate::System::Runtime::Serialization::ValueTypeFixupInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ValueTypeFixupInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::ValueTypeFixupInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+ValueTypeFixupInfo")]
impl crate::System::Runtime::Serialization::ValueTypeFixupInfo {
    pub fn New(
        containerID: i64,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        parentIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (containerID, member, parentIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        containerID: i64,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        parentIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (containerID, member, parentIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContainerID(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContainerID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::FieldInfo,
        > = __cordl_object.invoke("get_ParentField", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("get_ParentIndex", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+ValueTypeFixupInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::ValueTypeFixupInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
