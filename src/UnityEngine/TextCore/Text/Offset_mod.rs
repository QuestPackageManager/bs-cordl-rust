#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Offset {
    pub m_Left: f32,
    pub m_Right: f32,
    pub m_Top: f32,
    pub m_Bottom: f32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::Offset =>
    "UnityEngine.TextCore.Text"."Offset"
);
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::Offset {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+Offset")]
impl crate::UnityEngine::TextCore::Text::Offset {
    pub fn Equals(
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (left, right, top, bottom),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bottom(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bottom",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_left(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_left",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_right(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_right",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_top(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_top",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
