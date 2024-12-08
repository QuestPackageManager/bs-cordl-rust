#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicsFormatUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::GraphicsFormatUtility =>
    "UnityEngine.Experimental.Rendering"."GraphicsFormatUtility"
);
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {}
#[cfg(feature = "UnityEngine+Experimental+Rendering+GraphicsFormatUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::GraphicsFormatUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
