#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+FixedBitArray3")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FixedBitArray3 {
    pub _0: bool,
    pub _1: bool,
    pub _2: bool,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+FixedBitArray3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::Poly2Tri::FixedBitArray3 =>
    "UnityEngine.ProBuilder.Poly2Tri"."FixedBitArray3"
);
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+FixedBitArray3")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Poly2Tri::FixedBitArray3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+FixedBitArray3")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::FixedBitArray3 {
    #[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+FixedBitArray3+_Enumerate_d__10")]
    pub type _Enumerate_d__10 = crate::UnityEngine::ProBuilder::Poly2Tri::FixedBitArray3__Enumerate_d__10;
    pub fn Clear_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear__cordl_bool1(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Contains(&mut self, value: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Enumerate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<bool>,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Enumerate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<bool>,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<bool> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(&mut self, value: bool) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret)
    }
}
