#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GraphicsBufferHandle {
    pub value: u32,
}
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GraphicsBufferHandle =>
    "UnityEngine"."GraphicsBufferHandle"
);
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::GraphicsBufferHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
impl crate::UnityEngine::GraphicsBufferHandle {
    pub fn Equals_Gc0(
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
    pub fn Equals_GraphicsBufferHandle1(
        &mut self,
        other: crate::UnityEngine::GraphicsBufferHandle,
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
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBufferHandle>>
for crate::UnityEngine::GraphicsBufferHandle {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBufferHandle> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBufferHandle>>
for crate::UnityEngine::GraphicsBufferHandle {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBufferHandle> {
        todo!()
    }
}
