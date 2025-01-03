#[cfg(feature = "UnityEngine+Rendering+StencilState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StencilState {
    pub m_Enabled: u8,
    pub m_ReadMask: u8,
    pub m_WriteMask: u8,
    pub m_Padding: u8,
    pub m_CompareFunctionFront: u8,
    pub m_PassOperationFront: u8,
    pub m_FailOperationFront: u8,
    pub m_ZFailOperationFront: u8,
    pub m_CompareFunctionBack: u8,
    pub m_PassOperationBack: u8,
    pub m_FailOperationBack: u8,
    pub m_ZFailOperationBack: u8,
}
#[cfg(feature = "UnityEngine+Rendering+StencilState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::StencilState =>
    "UnityEngine.Rendering"."StencilState"
);
#[cfg(feature = "UnityEngine+Rendering+StencilState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::StencilState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+StencilState")]
impl crate::UnityEngine::Rendering::StencilState {
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
    pub fn Equals_StencilState0(
        &mut self,
        other: crate::UnityEngine::Rendering::StencilState,
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
    pub fn set_compareFunctionBack(
        &mut self,
        value: crate::UnityEngine::Rendering::CompareFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_compareFunctionBack",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compareFunctionFront(
        &mut self,
        value: crate::UnityEngine::Rendering::CompareFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_compareFunctionFront",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_enabled",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_failOperationBack(
        &mut self,
        value: crate::UnityEngine::Rendering::StencilOp,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_failOperationBack",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_failOperationFront(
        &mut self,
        value: crate::UnityEngine::Rendering::StencilOp,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_failOperationFront",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_passOperationBack(
        &mut self,
        value: crate::UnityEngine::Rendering::StencilOp,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_passOperationBack",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_passOperationFront(
        &mut self,
        value: crate::UnityEngine::Rendering::StencilOp,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_passOperationFront",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_readMask(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_readMask",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_writeMask(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_writeMask",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zFailOperationBack(
        &mut self,
        value: crate::UnityEngine::Rendering::StencilOp,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zFailOperationBack",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zFailOperationFront(
        &mut self,
        value: crate::UnityEngine::Rendering::StencilOp,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zFailOperationFront",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+StencilState")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::StencilState>>
for crate::UnityEngine::Rendering::StencilState {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::StencilState> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+StencilState")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::StencilState>>
for crate::UnityEngine::Rendering::StencilState {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Rendering::StencilState> {
        todo!()
    }
}
