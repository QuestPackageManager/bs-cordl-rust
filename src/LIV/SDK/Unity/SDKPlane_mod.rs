#[cfg(feature = "LIV+SDK+Unity+SDKPlane")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKPlane {
    pub distance: f32,
    pub normal: crate::LIV::SDK::Unity::SDKVector3,
}
#[cfg(feature = "LIV+SDK+Unity+SDKPlane")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKPlane => "LIV.SDK.Unity"
    ."SDKPlane"
);
#[cfg(feature = "LIV+SDK+Unity+SDKPlane")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LIV::SDK::Unity::SDKPlane {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKPlane")]
impl crate::LIV::SDK::Unity::SDKPlane {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_empty() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKPlane,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKPlane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_empty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: crate::UnityEngine::Plane,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKPlane> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKPlane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
}
