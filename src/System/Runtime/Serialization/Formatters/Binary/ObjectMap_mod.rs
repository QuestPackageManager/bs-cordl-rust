#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMap")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub objectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub binaryTypeEnumA: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
        >,
    >,
    pub typeInformationA: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub memberTypes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
    pub memberNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub objectInfo: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    >,
    pub isInitObjectInfo: bool,
    pub objectReader: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
    >,
    pub objectId: i32,
    pub assemblyInfo: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMap")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "ObjectMap";
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMap")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMap")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMap")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap {
    pub fn CreateObjectInfo(
        &mut self,
        si: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Serialization::SerializationInfo,
            >,
        >,
        memberData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
        > = __cordl_object.invoke("CreateObjectInfo", (si, memberData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Il2CppArray_Il2CppArray_Il2CppArray_ObjectReader_i32_BinaryAssemblyInfo_SizedArray1(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        binaryTypeEnumA: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
            >,
        >,
        typeInformationA: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        memberAssemIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        objectId: i32,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
        assemIdToAssemblyTable: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (
                    name,
                    memberNames,
                    binaryTypeEnumA,
                    typeInformationA,
                    memberAssemIds,
                    objectReader,
                    objectId,
                    assemblyInfo,
                    assemIdToAssemblyTable,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Type_ObjectReader_i32_BinaryAssemblyInfo0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        objectId: i32,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Create",
                (name, objectType, memberNames, objectReader, objectId, assemblyInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray_Il2CppArray_Il2CppArray_ObjectReader_i32_BinaryAssemblyInfo_SizedArray1(
        objectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        binaryTypeEnumA: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
            >,
        >,
        typeInformationA: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        memberAssemIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        objectId: i32,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
        assemIdToAssemblyTable: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    objectName,
                    memberNames,
                    binaryTypeEnumA,
                    typeInformationA,
                    memberAssemIds,
                    objectReader,
                    objectId,
                    assemblyInfo,
                    assemIdToAssemblyTable,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_ObjectReader_i32_BinaryAssemblyInfo0(
        objectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        objectId: i32,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    objectName,
                    objectType,
                    memberNames,
                    objectReader,
                    objectId,
                    assemblyInfo,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray_Il2CppArray_Il2CppArray_ObjectReader_i32_BinaryAssemblyInfo_SizedArray1(
        &mut self,
        objectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        binaryTypeEnumA: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Runtime::Serialization::Formatters::Binary::BinaryTypeEnum,
            >,
        >,
        typeInformationA: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        memberAssemIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        objectId: i32,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
        assemIdToAssemblyTable: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::SizedArray,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    objectName,
                    memberNames,
                    binaryTypeEnumA,
                    typeInformationA,
                    memberAssemIds,
                    objectReader,
                    objectId,
                    assemblyInfo,
                    assemIdToAssemblyTable,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_ObjectReader_i32_BinaryAssemblyInfo0(
        &mut self,
        objectName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        objectReader: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::ObjectReader,
        >,
        objectId: i32,
        assemblyInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::BinaryAssemblyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    objectName,
                    objectType,
                    memberNames,
                    objectReader,
                    objectId,
                    assemblyInfo,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
