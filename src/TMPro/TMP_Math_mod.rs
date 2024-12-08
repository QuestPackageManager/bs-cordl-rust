#[cfg(feature = "TMPro+TMP_Math")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Math {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "TMPro+TMP_Math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Math => "TMPro"."TMP_Math"
);
#[cfg(feature = "TMPro+TMP_Math")]
impl std::ops::Deref for crate::TMPro::TMP_Math {
    type Target = crate::System::Object;
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
