#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CountingBloomFilter__m_Counters_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer =>
    "UnityEngine.UIElements"."CountingBloomFilter/<m_Counters>e__FixedBuffer"
);
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer")]
impl crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer {}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CountingBloomFilter {
    pub m_Counters: crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::CountingBloomFilter =>
    "UnityEngine.UIElements"."CountingBloomFilter"
);
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::CountingBloomFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+CountingBloomFilter")]
impl crate::UnityEngine::UIElements::CountingBloomFilter {
    #[cfg(
        feature = "UnityEngine+UIElements+CountingBloomFilter+_m_Counters_e__FixedBuffer"
    )]
    pub type _m_Counters_e__FixedBuffer = crate::UnityEngine::UIElements::CountingBloomFilter__m_Counters_e__FixedBuffer;
    pub fn Hash2(&mut self, hash: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Hash2",
            (hash),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveHash(
        &mut self,
        hash: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveHash",
            (hash),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ContainsHash(&mut self, hash: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ContainsHash",
            (hash),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Hash1(&mut self, hash: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Hash1",
            (hash),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AdjustSlot(
        &mut self,
        index: u32,
        increment: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AdjustSlot",
            (index, increment),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsSlotEmpty(&mut self, index: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsSlotEmpty",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn InsertHash(
        &mut self,
        hash: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InsertHash",
            (hash),
        )?;
        Ok(__cordl_ret)
    }
}
