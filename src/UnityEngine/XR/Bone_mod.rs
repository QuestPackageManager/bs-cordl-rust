#[cfg(feature = "UnityEngine+XR+Bone")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bone {
    pub m_DeviceId: u64,
    pub m_FeatureIndex: u32,
}
#[cfg(feature = "UnityEngine+XR+Bone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::Bone => "UnityEngine.XR"."Bone"
);
#[cfg(feature = "UnityEngine+XR+Bone")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::XR::Bone {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+Bone")]
impl crate::UnityEngine::XR::Bone {
    pub fn Equals_Bone1(
        &mut self,
        other: crate::UnityEngine::XR::Bone,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceId(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deviceId",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_featureIndex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_featureIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+Bone")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::XR::Bone>>
for crate::UnityEngine::XR::Bone {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::XR::Bone> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+XR+Bone")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::XR::Bone>>
for crate::UnityEngine::XR::Bone {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::XR::Bone> {
        todo!()
    }
}
