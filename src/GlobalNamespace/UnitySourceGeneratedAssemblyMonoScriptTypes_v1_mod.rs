#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
#[repr(C)]
#[derive(Debug)]
pub struct UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 => ""
    ."UnitySourceGeneratedAssemblyMonoScriptTypes_v1"
);
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl std::ops::Deref
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl std::ops::DerefMut
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    #[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
    pub type MonoScriptData = crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    pub FilePathsData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub TypesData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub TotalTypes: i32,
    pub TotalFiles: i32,
    pub IsEditorOnly: bool,
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData =>
    ""."UnitySourceGeneratedAssemblyMonoScriptTypes_v1/MonoScriptData"
);
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnitySourceGeneratedAssemblyMonoScriptTypes_v1+MonoScriptData")]
impl crate::GlobalNamespace::UnitySourceGeneratedAssemblyMonoScriptTypes_v1_MonoScriptData {}
