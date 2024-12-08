#[cfg(feature = "QuantizedMathf")]
#[repr(C)]
#[derive(Debug)]
pub struct QuantizedMathf {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "QuantizedMathf")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for QuantizedMathf => ""."QuantizedMathf"
);
#[cfg(feature = "QuantizedMathf")]
impl std::ops::Deref for QuantizedMathf {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuantizedMathf")]
impl std::ops::DerefMut for QuantizedMathf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuantizedMathf")]
impl QuantizedMathf {
    pub const kQuaternionSerializableEpsilon: f32 = 0.00006103888f32;
    pub const kQuaternionSerializableScaleFactor: i32 = 16383i32;
    pub const kVectorSerializableEpsilon: f32 = 0.001f32;
    pub const kVectorSerializableScale: f32 = 1000f32;
    pub const kVectorSerializableScaleInt: i32 = 1000i32;
}
#[cfg(feature = "QuantizedMathf")]
impl quest_hook::libil2cpp::ObjectType for QuantizedMathf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
