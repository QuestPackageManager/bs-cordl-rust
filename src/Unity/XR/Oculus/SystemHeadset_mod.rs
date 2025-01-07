#[cfg(feature = "Unity+XR+Oculus+SystemHeadset")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemHeadset {
    #[default]
    Meta_Link_Quest_Pro = 4103i32,
    Meta_Quest_Pro = 10i32,
    None = 0i32,
    Oculus_Link_Quest = 4101i32,
    Oculus_Link_Quest_2 = 4102i32,
    Oculus_Quest = 8i32,
    Oculus_Quest_2 = 9i32,
    PC_Placeholder_4104 = 4104i32,
    PC_Placeholder_4105 = 4105i32,
    PC_Placeholder_4106 = 4106i32,
    PC_Placeholder_4107 = 4107i32,
    Placeholder_11 = 11i32,
    Placeholder_12 = 12i32,
    Placeholder_13 = 13i32,
    Placeholder_14 = 14i32,
    Rift_CB = 4099i32,
    Rift_CV1 = 4098i32,
    Rift_DK1 = 4096i32,
    Rift_DK2 = 4097i32,
    Rift_S = 4100i32,
}
#[cfg(feature = "Unity+XR+Oculus+SystemHeadset")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::SystemHeadset {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "SystemHeadset";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::XR::Oculus::SystemHeadset {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::XR::Oculus::SystemHeadset {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::XR::Oculus::SystemHeadset {
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
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::XR::Oculus::SystemHeadset {
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
