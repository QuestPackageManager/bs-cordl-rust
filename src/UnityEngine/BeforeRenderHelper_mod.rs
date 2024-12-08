#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeforeRenderHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BeforeRenderHelper => "UnityEngine"
    ."BeforeRenderHelper"
);
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl std::ops::Deref for crate::UnityEngine::BeforeRenderHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl std::ops::DerefMut for crate::UnityEngine::BeforeRenderHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl crate::UnityEngine::BeforeRenderHelper {
    #[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
    pub type OrderBlock = crate::UnityEngine::BeforeRenderHelper_OrderBlock;
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::BeforeRenderHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeforeRenderHelper_OrderBlock {
    pub order: i32,
    pub callback: *mut crate::UnityEngine::Events::UnityAction,
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BeforeRenderHelper_OrderBlock =>
    "UnityEngine"."BeforeRenderHelper/OrderBlock"
);
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::BeforeRenderHelper_OrderBlock {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BeforeRenderHelper+OrderBlock")]
impl crate::UnityEngine::BeforeRenderHelper_OrderBlock {}
