#[cfg(feature = "UnityEngine+Yoga+YogaValue")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct YogaValue {
    pub value: f32,
    pub unit: crate::UnityEngine::Yoga::YogaUnit,
}
#[cfg(feature = "UnityEngine+Yoga+YogaValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaValue =>
    "UnityEngine.Yoga"."YogaValue"
);
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
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Unit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaUnit> {
        let __cordl_ret: crate::UnityEngine::Yoga::YogaUnit = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Unit",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
