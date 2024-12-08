#[cfg(feature = "UnityEngine+UIElements+StartDragArgs")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StartDragArgs {
    pub _title_k__BackingField: *mut crate::System::String,
    pub _visualMode_k__BackingField: crate::UnityEngine::UIElements::DragVisualMode,
    pub _genericData_k__BackingField: *mut crate::System::Collections::Hashtable,
    pub _unityObjectReferences_k__BackingField: *mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::Object,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StartDragArgs")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StartDragArgs =>
    "UnityEngine.UIElements"."StartDragArgs"
);
#[cfg(feature = "UnityEngine+UIElements+StartDragArgs")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StartDragArgs {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StartDragArgs")]
impl crate::UnityEngine::UIElements::StartDragArgs {
    pub fn get_genericData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::Hashtable> {
        let __cordl_ret: *mut crate::System::Collections::Hashtable = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_genericData",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        title: *mut crate::System::String,
        visualMode: crate::UnityEngine::UIElements::DragVisualMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (title, visualMode),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_visualMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DragVisualMode> {
        let __cordl_ret: crate::UnityEngine::UIElements::DragVisualMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_visualMode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_genericData(
        &mut self,
        value: *mut crate::System::Collections::Hashtable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_genericData",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_unityObjectReferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Object,
        >,
    > {
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Object,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityObjectReferences",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_unityObjectReferences(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_unityObjectReferences",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_title(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_title",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetGenericData(
        &mut self,
        key: *mut crate::System::String,
        data: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetGenericData",
            (key, data),
        )?;
        Ok(__cordl_ret)
    }
}
