#[cfg(feature = "UnityEngine+InputSystem+Utilities+CallbackArray_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CallbackArray_1<TDelegate: quest_hook::libil2cpp::Type> {
    pub m_CannotMutateCallbacksArray: bool,
    pub m_Callbacks: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        TDelegate,
    >,
    pub m_CallbacksToAdd: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        TDelegate,
    >,
    pub m_CallbacksToRemove: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        TDelegate,
    >,
    __cordl_phantom_TDelegate: std::marker::PhantomData<TDelegate>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+CallbackArray_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::CallbackArray_1 < TDelegate > =>
    "UnityEngine.InputSystem.Utilities"."CallbackArray`1<TDelegate>" < TDelegate >
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+CallbackArray_1")]
unsafe impl<TDelegate: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<TDelegate> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+CallbackArray_1")]
impl<
    TDelegate: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputSystem::Utilities::CallbackArray_1<TDelegate> {
    pub fn AddCallback(
        &mut self,
        dlg: TDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AddCallback",
            (dlg),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn LockForChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LockForChanges",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn RemoveCallback(
        &mut self,
        dlg: TDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveCallback",
            (dlg),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UnlockForChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnlockForChanges",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<TDelegate>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDelegate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TDelegate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_length",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
