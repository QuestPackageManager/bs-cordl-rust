#[cfg(feature = "UnityEngine+ScriptingUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptingUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ScriptingUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ScriptingUtility => "UnityEngine"
    ."ScriptingUtility"
);
#[cfg(feature = "UnityEngine+ScriptingUtility")]
impl std::ops::Deref for crate::UnityEngine::ScriptingUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ScriptingUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ScriptingUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ScriptingUtility")]
impl crate::UnityEngine::ScriptingUtility {
    #[cfg(feature = "UnityEngine+ScriptingUtility+TestClass")]
    pub type TestClass = crate::UnityEngine::ScriptingUtility_TestClass;
}
#[cfg(feature = "UnityEngine+ScriptingUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ScriptingUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ScriptingUtility+TestClass")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScriptingUtility_TestClass {
    pub value: i32,
}
#[cfg(feature = "UnityEngine+ScriptingUtility+TestClass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ScriptingUtility_TestClass =>
    "UnityEngine"."ScriptingUtility/TestClass"
);
#[cfg(feature = "UnityEngine+ScriptingUtility+TestClass")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ScriptingUtility_TestClass {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ScriptingUtility+TestClass")]
impl crate::UnityEngine::ScriptingUtility_TestClass {}
