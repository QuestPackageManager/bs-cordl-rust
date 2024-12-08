#[cfg(feature = "UnityEngine+ArticulationJacobian")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ArticulationJacobian {
    pub rowsCount: i32,
    pub colsCount: i32,
    pub matrixData: *mut crate::System::Collections::Generic::List_1<f32>,
}
#[cfg(feature = "UnityEngine+ArticulationJacobian")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationJacobian =>
    "UnityEngine"."ArticulationJacobian"
);
#[cfg(feature = "UnityEngine+ArticulationJacobian")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ArticulationJacobian {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ArticulationJacobian")]
impl crate::UnityEngine::ArticulationJacobian {
    pub fn _ctor(
        &mut self,
        rows: i32,
        cols: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rows, cols),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(
        &mut self,
        row: i32,
        col: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (row, col),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_columns(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_columns",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_elements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<f32>,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<f32> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_elements",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rows(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rows",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        row: i32,
        col: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (row, col, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_columns(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_columns",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_elements(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_elements",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rows(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rows",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
