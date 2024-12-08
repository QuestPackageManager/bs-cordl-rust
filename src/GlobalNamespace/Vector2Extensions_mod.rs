#[cfg(feature = "Vector2Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2Extensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Vector2Extensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for Vector2Extensions => ""."Vector2Extensions"
);
#[cfg(feature = "Vector2Extensions")]
impl std::ops::Deref for Vector2Extensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Vector2Extensions")]
impl std::ops::DerefMut for Vector2Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Vector2Extensions")]
impl Vector2Extensions {}
#[cfg(feature = "Vector2Extensions")]
impl quest_hook::libil2cpp::ObjectType for Vector2Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
