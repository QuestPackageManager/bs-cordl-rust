#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StylePropertyNameCollection {
    pub propertiesList: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyNameCollection => "UnityEngine.UIElements"
    ."StylePropertyNameCollection"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyNameCollection {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection")]
impl crate::UnityEngine::UIElements::StylePropertyNameCollection {
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
    pub type Enumerator = crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IEnumerable_UnityEngine_UIElements_StylePropertyName__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.Generic.IEnumerable<UnityEngine.UIElements.StylePropertyName>.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerable.GetEnumerator",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        list: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (list),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StylePropertyNameCollection_Enumerator {
    pub m_Enumerator: crate::System::Collections::Generic::List_1_Enumerator<
        crate::UnityEngine::UIElements::StylePropertyName,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator =>
    "UnityEngine.UIElements"."StylePropertyNameCollection/Enumerator"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyNameCollection+Enumerator")]
impl crate::UnityEngine::UIElements::StylePropertyNameCollection_Enumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        enumerator: crate::System::Collections::Generic::List_1_Enumerator<
            crate::UnityEngine::UIElements::StylePropertyName,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (enumerator),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StylePropertyName,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::StylePropertyName = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
