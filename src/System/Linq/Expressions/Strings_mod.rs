#[cfg(feature = "System+Linq+Expressions+Strings")]
#[repr(C)]
#[derive(Debug)]
pub struct Strings {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Strings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::Strings =>
    "System.Linq.Expressions"."Strings"
);
#[cfg(feature = "System+Linq+Expressions+Strings")]
impl std::ops::Deref for crate::System::Linq::Expressions::Strings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Strings")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Strings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Strings")]
impl crate::System::Linq::Expressions::Strings {}
#[cfg(feature = "System+Linq+Expressions+Strings")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Strings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}