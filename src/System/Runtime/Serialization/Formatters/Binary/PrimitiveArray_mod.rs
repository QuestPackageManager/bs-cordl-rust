#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+PrimitiveArray")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimitiveArray {
    __cordl_parent: crate::System::Object,
    pub code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub booleanA: *mut quest_hook::libil2cpp::Il2CppArray<bool>,
    pub charA: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub doubleA: *mut quest_hook::libil2cpp::Il2CppArray<f64>,
    pub int16A: *mut quest_hook::libil2cpp::Il2CppArray<i16>,
    pub int32A: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub int64A: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
    pub sbyteA: *mut quest_hook::libil2cpp::Il2CppArray<i8>,
    pub singleA: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub uint16A: *mut quest_hook::libil2cpp::Il2CppArray<u16>,
    pub uint32A: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub uint64A: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+PrimitiveArray")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray =>
    "System.Runtime.Serialization.Formatters.Binary"."PrimitiveArray"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+PrimitiveArray")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+PrimitiveArray")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+PrimitiveArray")]
impl crate::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray {
    pub fn Init(
        &mut self,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        array: *mut crate::System::Array,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (code, array))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        array: *mut crate::System::Array,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (code, array))?;
        Ok(__cordl_object)
    }
    pub fn SetValue(
        &mut self,
        value: *mut crate::System::String,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, index))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        array: *mut crate::System::Array,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (code, array))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+PrimitiveArray")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::PrimitiveArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
