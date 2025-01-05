#[cfg(feature = "UnityEngine+Rendering+BatchMaterialID")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchMaterialID {
    pub value: u32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchMaterialID")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchMaterialID =>
    "UnityEngine.Rendering"."BatchMaterialID"
);
#[cfg(feature = "UnityEngine+Rendering+BatchMaterialID")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchMaterialID {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchMaterialID")]
impl crate::UnityEngine::Rendering::BatchMaterialID {
    pub fn Equals_BatchMaterialID1(
        &mut self,
        other: crate::UnityEngine::Rendering::BatchMaterialID,
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
}
#[cfg(feature = "UnityEngine+Rendering+BatchMaterialID")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMaterialID>>
for crate::UnityEngine::Rendering::BatchMaterialID {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMaterialID> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchMaterialID")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMaterialID>>
for crate::UnityEngine::Rendering::BatchMaterialID {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::BatchMaterialID,
    > {
        todo!()
    }
}
