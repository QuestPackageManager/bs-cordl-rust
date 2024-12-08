#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ExpressionVisitorUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Dynamic::Utils::ExpressionVisitorUtils
    => "System.Dynamic.Utils"."ExpressionVisitorUtils"
);
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl std::ops::Deref for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl std::ops::DerefMut for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl crate::System::Dynamic::Utils::ExpressionVisitorUtils {}
#[cfg(feature = "System+Dynamic+Utils+ExpressionVisitorUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Dynamic::Utils::ExpressionVisitorUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
