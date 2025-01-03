#[cfg(feature = "UnityEngine+BoneWeight1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BoneWeight1 {
    pub m_Weight: f32,
    pub m_BoneIndex: i32,
}
#[cfg(feature = "UnityEngine+BoneWeight1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BoneWeight1 => "UnityEngine"
    ."BoneWeight1"
);
#[cfg(feature = "UnityEngine+BoneWeight1")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::BoneWeight1 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BoneWeight1")]
impl crate::UnityEngine::BoneWeight1 {
    pub fn Equals_BoneWeight1_1(
        &mut self,
        other: crate::UnityEngine::BoneWeight1,
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
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    pub fn get_boneIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_boneIndex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_weight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_weight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+BoneWeight1")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::BoneWeight1>>
for crate::UnityEngine::BoneWeight1 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::BoneWeight1> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+BoneWeight1")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::BoneWeight1>>
for crate::UnityEngine::BoneWeight1 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::BoneWeight1> {
        todo!()
    }
}
