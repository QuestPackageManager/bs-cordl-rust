#[cfg(feature = "TMPro+HighlightState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HighlightState {
    pub color: crate::UnityEngine::Color32,
    pub padding: crate::TMPro::TMP_Offset,
}
#[cfg(feature = "TMPro+HighlightState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::HighlightState => "TMPro"
    ."HighlightState"
);
#[cfg(feature = "TMPro+HighlightState")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::HighlightState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+HighlightState")]
impl crate::TMPro::HighlightState {
    pub fn Equals_HighlightState1(
        &mut self,
        other: crate::TMPro::HighlightState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
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
    pub fn _ctor(
        &mut self,
        color: crate::UnityEngine::Color32,
        padding: crate::TMPro::TMP_Offset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (color, padding),
        )?;
        Ok(__cordl_ret)
    }
}
