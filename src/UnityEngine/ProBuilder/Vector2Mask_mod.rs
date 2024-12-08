#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vector2Mask {
    pub m_Mask: u8,
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Vector2Mask =>
    "UnityEngine.ProBuilder"."Vector2Mask"
);
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Vector2Mask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vector2Mask")]
impl crate::UnityEngine::ProBuilder::Vector2Mask {
    pub const X: u8 = 1u8;
    pub const Y: u8 = 2u8;
    pub fn get_x(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_x",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_y(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_y",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_f32_0(
        &mut self,
        v: crate::UnityEngine::Vector3,
        epsilon: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v, epsilon),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_1(
        &mut self,
        mask: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mask),
        )?;
        Ok(__cordl_ret)
    }
}
