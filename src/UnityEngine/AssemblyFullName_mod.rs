#[cfg(feature = "UnityEngine+AssemblyFullName")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AssemblyFullName {
    pub Name: *mut crate::System::String,
    pub Version: crate::UnityEngine::AssemblyVersion,
    pub PublicKeyToken: *mut crate::System::String,
    pub Culture: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+AssemblyFullName")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AssemblyFullName => "UnityEngine"
    ."AssemblyFullName"
);
#[cfg(feature = "UnityEngine+AssemblyFullName")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::AssemblyFullName {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+AssemblyFullName")]
impl crate::UnityEngine::AssemblyFullName {
    pub fn Equals(
        &mut self,
        other: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
