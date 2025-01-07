#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
#[repr(C)]
#[derive(Debug)]
pub struct Converter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "Converter";
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    pub fn CreatePrimitiveArray(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePrimitiveArray", (code, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromString", (value, code))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitArrayTypeA() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitArrayTypeA", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitCodeA() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitCodeA", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitTypeA() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitTypeA", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitTypeCodeA() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitTypeCodeA", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitValueA() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitValueA", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitiveArray(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeInformation: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrimitiveArray", (_cordl_type, typeInformation))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWriteAsByteArray(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWriteAsByteArray", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToArrayType(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToArrayType", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToCode(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    > {
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToCode", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToComType(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToComType", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToPrimitiveTypeEnum(
        typeCode: crate::System::TypeCode,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    > {
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToPrimitiveTypeEnum", (typeCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToType(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToType", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTypeCode(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToTypeCode", (code))?;
        Ok(__cordl_ret.into())
    }
    pub fn TypeLength(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TypeLength", (code))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
