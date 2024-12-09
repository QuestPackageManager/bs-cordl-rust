#[cfg(feature = "QuantizedMathf")]
#[repr(C)]
#[derive(Debug)]
pub struct QuantizedMathf {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "QuantizedMathf")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::QuantizedMathf => ""
    ."QuantizedMathf"
);
#[cfg(feature = "QuantizedMathf")]
impl std::ops::Deref for crate::GlobalNamespace::QuantizedMathf {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuantizedMathf")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuantizedMathf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuantizedMathf")]
impl crate::GlobalNamespace::QuantizedMathf {
    pub const kQuaternionSerializableEpsilon: f32 = 0.00006103888f32;
    pub const kQuaternionSerializableScaleFactor: i32 = 16383i32;
    pub const kVectorSerializableEpsilon: f32 = 0.001f32;
    pub const kVectorSerializableScale: f32 = 1000f32;
    pub const kVectorSerializableScaleInt: i32 = 1000i32;
}
#[cfg(feature = "QuantizedMathf")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::QuantizedMathf {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
