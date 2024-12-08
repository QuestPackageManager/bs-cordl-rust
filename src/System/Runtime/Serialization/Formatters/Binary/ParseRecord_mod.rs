#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct ParseRecord {
    __cordl_parent: crate::System::Object,
    pub PRparseTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalParseTypeE,
    pub PRobjectTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalObjectTypeE,
    pub PRarrayTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalArrayTypeE,
    pub PRmemberTypeEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalMemberTypeE,
    pub PRmemberValueEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalMemberValueE,
    pub PRobjectPositionEnum: crate::System::Runtime::Serialization::Formatters::Binary::InternalObjectPositionE,
    pub PRname: *mut crate::System::String,
    pub PRvalue: *mut crate::System::String,
    pub PRvarValue: *mut crate::System::Object,
    pub PRkeyDt: *mut crate::System::String,
    pub PRdtType: *mut crate::System::Type,
    pub PRdtTypeCode: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub PRisEnum: bool,
    pub PRobjectId: i64,
    pub PRidRef: i64,
    pub PRarrayElementTypeString: *mut crate::System::String,
    pub PRarrayElementType: *mut crate::System::Type,
    pub PRisArrayVariant: bool,
    pub PRarrayElementTypeCode: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub PRrank: i32,
    pub PRlengthA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub PRpositionA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub PRlowerBoundA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub PRupperBoundA: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub PRindexMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub PRmemberIndex: i32,
    pub PRlinearlength: i32,
    pub PRrectangularMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub PRisLowerBound: bool,
    pub PRtopId: i64,
    pub PRheaderId: i64,
    pub PRobjectInfo: *mut crate::System::Runtime::Serialization::Formatters::Binary::ReadObjectInfo,
    pub PRisValueTypeFixup: bool,
    pub PRnewObj: *mut crate::System::Object,
    pub PRobjectA: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub PRprimitiveArray: *mut crate::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray,
    pub PRisRegistered: bool,
    pub PRmemberData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Object,
    >,
    pub PRsi: *mut crate::System::Runtime::Serialization::SerializationInfo,
    pub PRnullCount: i32,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::ParseRecord =>
    "System.Runtime.Serialization.Formatters.Binary"."ParseRecord"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ParseRecord")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ParseRecord {
    type Target = crate::System::Object;
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
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
