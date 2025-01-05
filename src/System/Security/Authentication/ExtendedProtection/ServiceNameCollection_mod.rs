#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ServiceNameCollection {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ReadOnlyCollectionBase,
    >,
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Authentication::ExtendedProtection::ServiceNameCollection =>
    "System.Security.Authentication.ExtendedProtection"."ServiceNameCollection"
);
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
impl std::ops::Deref
for crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Collections::ReadOnlyCollectionBase,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
impl std::ops::DerefMut
for crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
impl crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
