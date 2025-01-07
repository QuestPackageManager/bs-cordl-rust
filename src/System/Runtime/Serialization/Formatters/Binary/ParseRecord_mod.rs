#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct ParseRecord {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub PRparseTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalParseTypeE,
    pub PRobjectTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalObjectTypeE,
    pub PRarrayTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalArrayTypeE,
    pub PRmemberTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalMemberTypeE,
    pub PRmemberValueEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalMemberValueE,
    pub PRobjectPositionEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalObjectPositionE,
    pub PRname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PRvalue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PRvarValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub PRkeyDt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub PRdtType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub PRdtTypeCode: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub PRisEnum: bool,
    pub PRobjectId: i64,
    pub PRidRef: i64,
    pub PRarrayElementTypeString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub PRarrayElementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub PRisArrayVariant: bool,
    pub PRarrayElementTypeCode: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub PRrank: i32,
    pub PRlengthA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub PRpositionA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub PRlowerBoundA: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub PRupperBoundA: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub PRindexMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub PRmemberIndex: i32,
    pub PRlinearlength: i32,
    pub PRrectangularMap: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub PRisLowerBound: bool,
    pub PRtopId: i64,
    pub PRheaderId: i64,
    pub PRobjectInfo: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    >,
    pub PRisValueTypeFixup: bool,
    pub PRnewObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub PRobjectA: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub PRprimitiveArray: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray,
    >,
    pub PRisRegistered: bool,
    pub PRmemberData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub PRsi: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationInfo,
    >,
    pub PRnullCount: i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "ParseRecord";
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
