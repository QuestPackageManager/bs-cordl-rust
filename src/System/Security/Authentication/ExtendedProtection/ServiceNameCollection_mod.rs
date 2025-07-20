#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ServiceNameCollection {
    __cordl_parent: crate::System::Collections::ReadOnlyCollectionBase,
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Authentication.ExtendedProtection";
    const CLASS_NAME: &'static str = "ServiceNameCollection";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
impl std::ops::Deref
for crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {
    type Target = crate::System::Collections::ReadOnlyCollectionBase;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ServiceNameCollection"
)]
impl std::ops::DerefMut
for crate::System::Security::Authentication::ExtendedProtection::ServiceNameCollection {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
