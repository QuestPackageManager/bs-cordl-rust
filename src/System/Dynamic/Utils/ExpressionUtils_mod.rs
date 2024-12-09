#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::ExpressionUtils =>
    "System.Dynamic.Utils"."ExpressionUtils"
);
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::ExpressionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::ExpressionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl crate::System::Dynamic::Utils::ExpressionUtils {}
#[cfg(feature = "System+Dynamic+Utils+ExpressionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::ExpressionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
