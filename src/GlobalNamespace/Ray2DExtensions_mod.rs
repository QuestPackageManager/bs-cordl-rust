#[cfg(feature = "Ray2DExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Ray2DExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Ray2DExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for Ray2DExtensions => ""."Ray2DExtensions"
);
#[cfg(feature = "Ray2DExtensions")]
impl std::ops::Deref for Ray2DExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Ray2DExtensions")]
impl std::ops::DerefMut for Ray2DExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Ray2DExtensions")]
impl Ray2DExtensions {}
#[cfg(feature = "Ray2DExtensions")]
impl quest_hook::libil2cpp::ObjectType for Ray2DExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}