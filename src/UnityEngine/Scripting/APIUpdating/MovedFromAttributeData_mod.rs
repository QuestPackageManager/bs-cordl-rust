#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttributeData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovedFromAttributeData {
    pub className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub nameSpace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub assembly: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub classHasChanged: bool,
    pub nameSpaceHasChanged: bool,
    pub assemblyHasChanged: bool,
    pub autoUdpateAPI: bool,
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttributeData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Scripting::APIUpdating::MovedFromAttributeData =>
    "UnityEngine.Scripting.APIUpdating"."MovedFromAttributeData"
);
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttributeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Scripting::APIUpdating::MovedFromAttributeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Scripting+APIUpdating+MovedFromAttributeData")]
impl crate::UnityEngine::Scripting::APIUpdating::MovedFromAttributeData {
    pub fn Set(
        &mut self,
        autoUpdateAPI: bool,
        sourceNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceAssembly: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceClassName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (autoUpdateAPI, sourceNamespace, sourceAssembly, sourceClassName),
        )?;
        Ok(__cordl_ret.into())
    }
}
