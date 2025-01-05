#[cfg(feature = "TMPro+TMP_Math")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMP_Math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Math => "TMPro"."TMP_Math"
);
#[cfg(feature = "TMPro+TMP_Math")]
impl std::ops::Deref for crate::TMPro::TMP_Math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Math")]
impl std::ops::DerefMut for crate::TMPro::TMP_Math {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Math")]
impl crate::TMPro::TMP_Math {
    pub const FLOAT_MAX: f32 = 32767f32;
    pub const FLOAT_MIN: f32 = -32767f32;
    pub const FLOAT_UNSET: f32 = -32767f32;
    pub const INT_UNSET: i32 = -32767i32;
    pub const _cordl_INT_MAX: i32 = 2147483647i32;
    pub const _cordl_INT_MIN: i32 = -2147483647i32;
    pub fn Approximately(a: f32, b: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn _cordl_Mod(a: i32, b: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mod", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_Math")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
