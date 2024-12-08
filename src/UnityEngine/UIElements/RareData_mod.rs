#[cfg(feature = "UnityEngine+UIElements+RareData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RareData {
    pub cursor: crate::UnityEngine::UIElements::Cursor,
    pub textOverflow: crate::UnityEngine::UIElements::TextOverflow,
    pub unityBackgroundImageTintColor: crate::UnityEngine::Color,
    pub unityOverflowClipBox: crate::UnityEngine::UIElements::OverflowClipBox,
    pub unitySliceBottom: i32,
    pub unitySliceLeft: i32,
    pub unitySliceRight: i32,
    pub unitySliceScale: f32,
    pub unitySliceTop: i32,
    pub unityTextOverflowPosition: crate::UnityEngine::UIElements::TextOverflowPosition,
}
#[cfg(feature = "UnityEngine+UIElements+RareData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RareData =>
    "UnityEngine.UIElements"."RareData"
);
#[cfg(feature = "UnityEngine+UIElements+RareData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::RareData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RareData")]
impl crate::UnityEngine::UIElements::RareData {
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::RareData> {
        let __cordl_ret: crate::UnityEngine::UIElements::RareData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Copy",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::RareData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
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
    pub fn Equals_RareData0(
        &mut self,
        other: crate::UnityEngine::UIElements::RareData,
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
}