#[cfg(feature = "UnityEngine+InputSystem+DynamicBitfield")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DynamicBitfield {
    pub array: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<u64>,
    pub length: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+DynamicBitfield")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::DynamicBitfield =>
    "UnityEngine.InputSystem"."DynamicBitfield"
);
#[cfg(feature = "UnityEngine+InputSystem+DynamicBitfield")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::DynamicBitfield {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DynamicBitfield")]
impl crate::UnityEngine::InputSystem::DynamicBitfield {
    pub fn BitCountToULongCount(bitCount: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BitCountToULongCount", (bitCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearBit(
        &mut self,
        bitIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClearBit",
            (bitIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBit(
        &mut self,
        bitIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBit",
            (bitIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLength(
        &mut self,
        newLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLength",
            (newLength),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TestBit(&mut self, bitIndex: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TestBit",
            (bitIndex),
        )?;
        Ok(__cordl_ret.into())
    }
}
