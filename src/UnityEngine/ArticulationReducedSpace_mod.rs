#[cfg(feature = "UnityEngine+ArticulationReducedSpace")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArticulationReducedSpace {
    pub x: crate::UnityEngine::ArticulationReducedSpace__x_e__FixedBuffer,
    pub dofCount: i32,
}
#[cfg(feature = "UnityEngine+ArticulationReducedSpace")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ArticulationReducedSpace =>
    "UnityEngine"."ArticulationReducedSpace"
);
#[cfg(feature = "UnityEngine+ArticulationReducedSpace")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ArticulationReducedSpace {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ArticulationReducedSpace")]
impl crate::UnityEngine::ArticulationReducedSpace {
    #[cfg(feature = "UnityEngine+ArticulationReducedSpace+_x_e__FixedBuffer")]
    pub type _x_e__FixedBuffer = crate::UnityEngine::ArticulationReducedSpace__x_e__FixedBuffer;
    pub fn _ctor_f32_0(
        &mut self,
        a: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_1(
        &mut self,
        a: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_2(
        &mut self,
        a: f32,
        b: f32,
        c: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, i: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (i),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        i: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (i, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ArticulationReducedSpace+_x_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArticulationReducedSpace__x_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(feature = "UnityEngine+ArticulationReducedSpace+_x_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ArticulationReducedSpace__x_e__FixedBuffer => "UnityEngine"
    ."ArticulationReducedSpace/<x>e__FixedBuffer"
);
#[cfg(feature = "UnityEngine+ArticulationReducedSpace+_x_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ArticulationReducedSpace__x_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ArticulationReducedSpace+_x_e__FixedBuffer")]
impl crate::UnityEngine::ArticulationReducedSpace__x_e__FixedBuffer {}
