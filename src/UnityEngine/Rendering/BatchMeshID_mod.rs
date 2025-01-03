#[cfg(feature = "UnityEngine+Rendering+BatchMeshID")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchMeshID {
    pub value: u32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchMeshID")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchMeshID =>
    "UnityEngine.Rendering"."BatchMeshID"
);
#[cfg(feature = "UnityEngine+Rendering+BatchMeshID")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchMeshID {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchMeshID")]
impl crate::UnityEngine::Rendering::BatchMeshID {
    pub fn Equals_BatchMeshID1(
        &mut self,
        other: crate::UnityEngine::Rendering::BatchMeshID,
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
#[cfg(feature = "UnityEngine+Rendering+BatchMeshID")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMeshID>>
for crate::UnityEngine::Rendering::BatchMeshID {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMeshID> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchMeshID")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMeshID>>
for crate::UnityEngine::Rendering::BatchMeshID {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Rendering::BatchMeshID> {
        todo!()
    }
}
