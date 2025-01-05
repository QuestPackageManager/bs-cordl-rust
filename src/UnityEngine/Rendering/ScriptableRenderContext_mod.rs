#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ScriptableRenderContext {
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ScriptableRenderContext
    => "UnityEngine.Rendering"."ScriptableRenderContext"
);
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ScriptableRenderContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
impl crate::UnityEngine::Rendering::ScriptableRenderContext {
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
    pub fn Equals_ScriptableRenderContext0(
        &mut self,
        other: crate::UnityEngine::Rendering::ScriptableRenderContext,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameras(
        &mut self,
        results: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCameras",
            (results),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameras_Internal(
        &mut self,
        listType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resultList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCameras_Internal",
            (listType, resultList),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameras_Internal_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ScriptableRenderContext,
        >,
        listType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        resultList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCameras_Internal_Injected",
                (_unity_self, listType, resultList),
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
    pub fn _ctor(
        &mut self,
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (ptr),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScriptableRenderContext>,
> for crate::UnityEngine::Rendering::ScriptableRenderContext {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ScriptableRenderContext,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableRenderContext")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScriptableRenderContext>,
> for crate::UnityEngine::Rendering::ScriptableRenderContext {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ScriptableRenderContext,
    > {
        todo!()
    }
}
