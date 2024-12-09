#[cfg(feature = "UnityEngine+EnumData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct EnumData {
    pub values: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Enum>,
    pub flagValues: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub displayNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub names: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub tooltip: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub flags: bool,
    pub underlyingType: *mut crate::System::Type,
    pub _cordl_unsigned: bool,
    pub serializable: bool,
}
#[cfg(feature = "UnityEngine+EnumData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EnumData => "UnityEngine"
    ."EnumData"
);
#[cfg(feature = "UnityEngine+EnumData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::EnumData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+EnumData")]
impl crate::UnityEngine::EnumData {}
