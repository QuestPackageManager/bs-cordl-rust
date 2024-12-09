#[cfg(feature = "System+Data+ExceptionBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct ExceptionBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+ExceptionBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ExceptionBuilder => "System.Data"
    ."ExceptionBuilder"
);
#[cfg(feature = "System+Data+ExceptionBuilder")]
impl std::ops::Deref for crate::System::Data::ExceptionBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExceptionBuilder")]
impl std::ops::DerefMut for crate::System::Data::ExceptionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ExceptionBuilder")]
impl crate::System::Data::ExceptionBuilder {}
#[cfg(feature = "System+Data+ExceptionBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ExceptionBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
