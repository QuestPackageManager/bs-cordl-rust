#[cfg(feature = "ModestTree+ReflectionUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ModestTree+ReflectionUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::ReflectionUtil => "ModestTree"
    ."ReflectionUtil"
);
#[cfg(feature = "ModestTree+ReflectionUtil")]
impl std::ops::Deref for crate::ModestTree::ReflectionUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+ReflectionUtil")]
impl std::ops::DerefMut for crate::ModestTree::ReflectionUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+ReflectionUtil")]
impl crate::ModestTree::ReflectionUtil {}
#[cfg(feature = "ModestTree+ReflectionUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::ReflectionUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
