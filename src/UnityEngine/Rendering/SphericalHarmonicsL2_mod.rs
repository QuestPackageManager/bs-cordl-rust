#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SphericalHarmonicsL2 {
    pub shr0: f32,
    pub shr1: f32,
    pub shr2: f32,
    pub shr3: f32,
    pub shr4: f32,
    pub shr5: f32,
    pub shr6: f32,
    pub shr7: f32,
    pub shr8: f32,
    pub shg0: f32,
    pub shg1: f32,
    pub shg2: f32,
    pub shg3: f32,
    pub shg4: f32,
    pub shg5: f32,
    pub shg6: f32,
    pub shg7: f32,
    pub shg8: f32,
    pub shb0: f32,
    pub shb1: f32,
    pub shb2: f32,
    pub shb3: f32,
    pub shb4: f32,
    pub shb5: f32,
    pub shb6: f32,
    pub shb7: f32,
    pub shb8: f32,
}
#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::SphericalHarmonicsL2 =>
    "UnityEngine.Rendering"."SphericalHarmonicsL2"
);
#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::SphericalHarmonicsL2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SphericalHarmonicsL2")]
impl crate::UnityEngine::Rendering::SphericalHarmonicsL2 {
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_SphericalHarmonicsL2_1(
        &mut self,
        other: crate::UnityEngine::Rendering::SphericalHarmonicsL2,
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
}
