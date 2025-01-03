#[cfg(feature = "TMPro+Mesh_Extents")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Mesh_Extents {
    pub min: crate::UnityEngine::Vector2,
    pub max: crate::UnityEngine::Vector2,
}
#[cfg(feature = "TMPro+Mesh_Extents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::Mesh_Extents => "TMPro"."Mesh_Extents"
);
#[cfg(feature = "TMPro+Mesh_Extents")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::TMPro::Mesh_Extents {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TMPro+Mesh_Extents")]
impl crate::TMPro::Mesh_Extents {
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
    pub fn _ctor(
        &mut self,
        min: crate::UnityEngine::Vector2,
        max: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
}
