#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEvent")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub m_Graph: *mut crate::System::String,
    pub m_Dependencies: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_ObjectId: i32,
    pub m_DisplayName: *mut crate::System::String,
    pub m_Stream: i32,
    pub m_Frame: i32,
    pub m_Value: i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent =>
    "UnityEngine.ResourceManagement.Diagnostics"."DiagnosticEvent"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEvent")]
impl crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent {
    pub fn get_Value(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Stream(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Stream",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        graph: *mut crate::System::String,
        name: *mut crate::System::String,
        id: i32,
        stream: i32,
        frame: i32,
        value: i32,
        deps: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (graph, name, id, stream, frame, value, deps),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ObjectId",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Frame(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Frame",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Graph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Graph",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_DisplayName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_DisplayName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Dependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Dependencies",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
