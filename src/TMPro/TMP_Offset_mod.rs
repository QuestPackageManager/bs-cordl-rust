#[cfg(feature = "TMPro+TMP_Offset")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TMP_Offset {
    pub m_Left: f32,
    pub m_Right: f32,
    pub m_Top: f32,
    pub m_Bottom: f32,
}
#[cfg(feature = "TMPro+TMP_Offset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Offset => "TMPro"."TMP_Offset"
);
#[cfg(feature = "TMPro+TMP_Offset")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::TMP_Offset {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+TMP_Offset")]
impl crate::TMPro::TMP_Offset {
    pub fn Equals_Il2CppObject0(
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
    pub fn Equals_TMP_Offset1(
        &mut self,
        other: crate::TMPro::TMP_Offset,
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
    pub fn _ctor_f32_f32_0(
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
    pub fn _ctor_f32_f32_1(
        &mut self,
        horizontal: f32,
        vertical: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (horizontal, vertical),
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
    pub fn get_horizontal(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_horizontal",
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
    pub fn get_vertical(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vertical",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zero() -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_Offset> {
        let __cordl_ret: crate::TMPro::TMP_Offset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::TMPro::TMP_Offset,
        rhs: crate::TMPro::TMP_Offset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::TMPro::TMP_Offset,
        rhs: crate::TMPro::TMP_Offset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        a: crate::TMPro::TMP_Offset,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<crate::TMPro::TMP_Offset> {
        let __cordl_ret: crate::TMPro::TMP_Offset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bottom(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bottom",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_horizontal(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_horizontal",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_left(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_left",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_right(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_right",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_top(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_top",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vertical(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_vertical",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
