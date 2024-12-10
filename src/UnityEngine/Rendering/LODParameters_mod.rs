#[cfg(feature = "UnityEngine+Rendering+LODParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LODParameters {
    pub m_IsOrthographic: i32,
    pub m_CameraPosition: crate::UnityEngine::Vector3,
    pub m_FieldOfView: f32,
    pub m_OrthoSize: f32,
    pub m_CameraPixelHeight: i32,
}
#[cfg(feature = "UnityEngine+Rendering+LODParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::LODParameters =>
    "UnityEngine.Rendering"."LODParameters"
);
#[cfg(feature = "UnityEngine+Rendering+LODParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::LODParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODParameters")]
impl crate::UnityEngine::Rendering::LODParameters {
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_LODParameters0(
        &mut self,
        other: crate::UnityEngine::Rendering::LODParameters,
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
}
#[cfg(feature = "UnityEngine+Rendering+LODParameters")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::LODParameters>>
for crate::UnityEngine::Rendering::LODParameters {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::LODParameters> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+LODParameters")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::LODParameters>>
for crate::UnityEngine::Rendering::LODParameters {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Rendering::LODParameters> {
        todo!()
    }
}
