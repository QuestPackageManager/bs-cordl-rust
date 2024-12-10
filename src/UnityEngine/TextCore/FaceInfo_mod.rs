#[cfg(feature = "UnityEngine+TextCore+FaceInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FaceInfo {
    pub m_FaceIndex: i32,
    pub m_FamilyName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_StyleName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_PointSize: i32,
    pub m_Scale: f32,
    pub m_UnitsPerEM: i32,
    pub m_LineHeight: f32,
    pub m_AscentLine: f32,
    pub m_CapLine: f32,
    pub m_MeanLine: f32,
    pub m_Baseline: f32,
    pub m_DescentLine: f32,
    pub m_SuperscriptOffset: f32,
    pub m_SuperscriptSize: f32,
    pub m_SubscriptOffset: f32,
    pub m_SubscriptSize: f32,
    pub m_UnderlineOffset: f32,
    pub m_UnderlineThickness: f32,
    pub m_StrikethroughOffset: f32,
    pub m_StrikethroughThickness: f32,
    pub m_TabWidth: f32,
}
#[cfg(feature = "UnityEngine+TextCore+FaceInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::FaceInfo =>
    "UnityEngine.TextCore"."FaceInfo"
);
#[cfg(feature = "UnityEngine+TextCore+FaceInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::FaceInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+FaceInfo")]
impl crate::UnityEngine::TextCore::FaceInfo {
    pub fn get_ascentLine(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ascentLine",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_baseline(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_baseline",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_capLine(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_capLine",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_descentLine(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_descentLine",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_faceIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_faceIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_familyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_familyName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lineHeight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meanLine(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_meanLine",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_pointSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_strikethroughOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_strikethroughOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_styleName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subscriptOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_subscriptOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subscriptSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_subscriptSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_superscriptOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_superscriptOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_superscriptSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_superscriptSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tabWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_tabWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_underlineOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_underlineOffset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_underlineThickness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_underlineThickness",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ascentLine(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ascentLine",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_baseline(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_baseline",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_capLine(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_capLine",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_descentLine(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_descentLine",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_familyName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_familyName",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lineHeight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_lineHeight",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_meanLine(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_meanLine",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pointSize(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_pointSize",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_scale",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_strikethroughOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_strikethroughOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_strikethroughThickness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_strikethroughThickness",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_styleName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_styleName",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subscriptOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_subscriptOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subscriptSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_subscriptSize",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_superscriptOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_superscriptOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_superscriptSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_superscriptSize",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tabWidth(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_tabWidth",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_underlineOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_underlineOffset",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_underlineThickness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_underlineThickness",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
