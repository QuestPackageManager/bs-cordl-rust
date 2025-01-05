#[cfg(feature = "UnityEngine+Rendering+BatchPackedCullingViewID")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchPackedCullingViewID {
    pub handle: u64,
}
#[cfg(feature = "UnityEngine+Rendering+BatchPackedCullingViewID")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchPackedCullingViewID
    => "UnityEngine.Rendering"."BatchPackedCullingViewID"
);
#[cfg(feature = "UnityEngine+Rendering+BatchPackedCullingViewID")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchPackedCullingViewID {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchPackedCullingViewID")]
impl crate::UnityEngine::Rendering::BatchPackedCullingViewID {
    pub fn Equals_BatchPackedCullingViewID0(
        &mut self,
        other: crate::UnityEngine::Rendering::BatchPackedCullingViewID,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc1(
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
#[cfg(feature = "UnityEngine+Rendering+BatchPackedCullingViewID")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::BatchPackedCullingViewID>,
> for crate::UnityEngine::Rendering::BatchPackedCullingViewID {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::BatchPackedCullingViewID,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchPackedCullingViewID")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::BatchPackedCullingViewID>,
> for crate::UnityEngine::Rendering::BatchPackedCullingViewID {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::BatchPackedCullingViewID,
    > {
        todo!()
    }
}
