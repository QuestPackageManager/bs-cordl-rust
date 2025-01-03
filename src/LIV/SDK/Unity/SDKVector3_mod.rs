#[cfg(feature = "LIV+SDK+Unity+SDKVector3")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SDKVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKVector3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKVector3 => "LIV.SDK.Unity"
    ."SDKVector3"
);
#[cfg(feature = "LIV+SDK+Unity+SDKVector3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LIV::SDK::Unity::SDKVector3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKVector3")]
impl crate::LIV::SDK::Unity::SDKVector3 {
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
    pub fn get_forward() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKVector3,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_forward", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_one() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKVector3,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_one", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_right() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKVector3,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_right", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_up() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKVector3,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_up", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zero() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::SDKVector3,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        lhs: crate::LIV::SDK::Unity::SDKVector3,
        rhs: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_SDKVector3_0(
        v: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector3_1(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_SDKVector3_0(
        lhs: crate::LIV::SDK::Unity::SDKVector3,
        rhs: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_1(
        lhs: crate::LIV::SDK::Unity::SDKVector3,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        lhs: crate::LIV::SDK::Unity::SDKVector3,
        rhs: crate::LIV::SDK::Unity::SDKVector3,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKVector3> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKVector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
