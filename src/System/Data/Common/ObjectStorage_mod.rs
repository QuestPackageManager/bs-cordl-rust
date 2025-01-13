#[cfg(feature = "System+Data+Common+ObjectStorage")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectStorage {
    __cordl_parent: crate::System::Data::Common::DataStorage,
    pub _values: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub _implementsIXmlSerializable: bool,
}
#[cfg(feature = "System+Data+Common+ObjectStorage")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Common::ObjectStorage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data.Common";
    const CLASS_NAME: &'static str = "ObjectStorage";
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
#[cfg(feature = "System+Data+Common+ObjectStorage")]
impl std::ops::Deref for crate::System::Data::Common::ObjectStorage {
    type Target = crate::System::Data::Common::DataStorage;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage")]
impl std::ops::DerefMut for crate::System::Data::Common::ObjectStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage")]
impl crate::System::Data::Common::ObjectStorage {
    #[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
    pub type Families = crate::System::Data::Common::ObjectStorage_Families;
    #[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
    pub type TempAssemblyComparer = crate::System::Data::Common::ObjectStorage_TempAssemblyComparer;
    pub fn Aggregate(
        &mut self,
        records: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        kind: crate::System::Data::AggregateType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Aggregate", (records, kind))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        &mut self,
        recordNo1: i32,
        recordNo2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (recordNo1, recordNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo(
        &mut self,
        valueNo1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        valueNo2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (valueNo1, valueNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareValueTo(
        &mut self,
        recordNo1: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareValueTo", (recordNo1, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareWithFamilies(
        &mut self,
        valueNo1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        valueNo2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareWithFamilies", (valueNo1, valueNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertObjectToXml_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ConvertObjectToXml", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertObjectToXml_XmlWriter_XmlRootAttribute1(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        xmlWriter: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlWriter>,
        xmlAttrib: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertObjectToXml", (value, xmlWriter, xmlAttrib))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertXmlToObject_Il2CppString0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ConvertXmlToObject", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertXmlToObject_XmlReader_XmlRootAttribute1(
        &mut self,
        xmlReader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
        xmlAttrib: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ConvertXmlToObject", (xmlReader, xmlAttrib))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy(
        &mut self,
        recordNo1: i32,
        recordNo2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Copy", (recordNo1, recordNo2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyValue(
        &mut self,
        record: i32,
        store: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
        storeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValue", (record, store, nullbits, storeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        &mut self,
        recordNo: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Get", (recordNo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEmptyStorage(
        &mut self,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetEmptyStorage", (recordCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFamily(
        &mut self,
        dataType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Data::Common::ObjectStorage_Families,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::Common::ObjectStorage_Families = __cordl_object
            .invoke("GetFamily", (dataType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlSerializer_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlSerializer>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetXmlSerializer", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetXmlSerializer_XmlRootAttribute1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        attribute: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlRootAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Serialization::XmlSerializer>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Serialization::XmlSerializer,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetXmlSerializer", (_cordl_type, attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull(&mut self, record: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", (record))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (column, _cordl_type))?;
        Ok(__cordl_object.into())
    }
    pub fn Set(
        &mut self,
        recordNo: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (recordNo, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCapacity", (capacity))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStorage(
        &mut self,
        store: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nullbits: quest_hook::libil2cpp::Gc<crate::System::Collections::BitArray>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStorage", (store, nullbits))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyIDynamicMetaObjectProvider(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyIDynamicMetaObjectProvider", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        column: quest_hook::libil2cpp::Gc<crate::System::Data::DataColumn>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (column, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Common::ObjectStorage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectStorage_Families {
    #[default]
    ARRAY = 4i32,
    BOOLEAN = 3i32,
    DATETIME = 0i32,
    NUMBER = 1i32,
    STRING = 2i32,
}
#[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Data::Common::ObjectStorage_Families {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data.Common";
    const CLASS_NAME: &'static str = "ObjectStorage/Families";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Data::Common::ObjectStorage_Families {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Data::Common::ObjectStorage_Families {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Data::Common::ObjectStorage_Families {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+Families")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Data::Common::ObjectStorage_Families {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectStorage_TempAssemblyComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data.Common";
    const CLASS_NAME: &'static str = "ObjectStorage/TempAssemblyComparer";
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
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
impl std::ops::Deref
for crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
impl std::ops::DerefMut
for crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
impl crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    pub fn Equals(
        &mut self,
        x: crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
        y: crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
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
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
    >,
> for crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+Common+ObjectStorage+TempAssemblyComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
    >,
> for crate::System::Data::Common::ObjectStorage_TempAssemblyComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<
                crate::System::Xml::Serialization::XmlRootAttribute,
            >,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
