#[cfg(feature = "UnityEngine+Yoga+YogaValue")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct YogaValue {
    pub value: f32,
    pub unit: crate::UnityEngine::Yoga::YogaUnit,
}
#[cfg(feature = "UnityEngine+Yoga+YogaValue")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Yoga::YogaValue {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Yoga";
    const CLASS_NAME: &'static str = "YogaValue";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Yoga::YogaValue {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Yoga::YogaValue {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Yoga::YogaValue {
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
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Yoga::YogaValue {
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
#[cfg(feature = "UnityEngine+Yoga+YogaValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Yoga::YogaValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Yoga+YogaValue")]
impl crate::UnityEngine::Yoga::YogaValue {
    pub fn Auto() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaValue> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Auto", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_YogaValue0(
        &mut self,
        other: crate::UnityEngine::Yoga::YogaValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Percent(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaValue> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Percent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Point(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaValue> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Point", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Unit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaUnit> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaUnit = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Unit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        pointValue: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaValue> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (pointValue))?;
        Ok(__cordl_ret.into())
    }
}
