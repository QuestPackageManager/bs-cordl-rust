#[cfg(feature = "QuantizedMathf")]
#[repr(C)]
#[derive(Debug)]
pub struct QuantizedMathf {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "QuantizedMathf")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::QuantizedMathf {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuantizedMathf";
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
    pub fn Approximately_Pose_Pose4(
        a: crate::UnityEngine::Pose,
        b: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately_Quaternion_Quaternion0(
        a: crate::UnityEngine::Quaternion,
        b: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately_Vector3_Vector3_3(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately_f32_f32_f32_1(
        a: f32,
        b: f32,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b, epsilon))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately_i32_i32_i32_2(
        a: i32,
        b: i32,
        epsilon: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b, epsilon))?;
        Ok(__cordl_ret.into())
    }
    pub fn QuantizedVectorComponentToString(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("QuantizedVectorComponentToString", (v))?;
        Ok(__cordl_ret.into())
    }
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
