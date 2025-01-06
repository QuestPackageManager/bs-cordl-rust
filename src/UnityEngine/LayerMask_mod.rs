#[cfg(feature = "UnityEngine+LayerMask")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LayerMask {
    pub m_Mask: i32,
}
#[cfg(feature = "UnityEngine+LayerMask")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LayerMask => "UnityEngine"
    ."LayerMask"
);
#[cfg(feature = "UnityEngine+LayerMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::LayerMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LayerMask")]
impl crate::UnityEngine::LayerMask {
    pub fn GetMask(
        layerNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMask", (layerNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn NameToLayer(
        layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NameToLayer", (layerName))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_LayerMask0(
        mask: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        intVal: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LayerMask> {
        let __cordl_ret: crate::UnityEngine::LayerMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (intVal))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_value",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
