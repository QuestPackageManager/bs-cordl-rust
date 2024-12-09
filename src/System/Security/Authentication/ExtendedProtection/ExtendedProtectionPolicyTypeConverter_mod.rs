#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ExtendedProtectionPolicyTypeConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ExtendedProtectionPolicyTypeConverter {
    __cordl_parent: crate::System::ComponentModel::TypeConverter,
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ExtendedProtectionPolicyTypeConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter
    => "System.Security.Authentication.ExtendedProtection"
    ."ExtendedProtectionPolicyTypeConverter"
);
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ExtendedProtectionPolicyTypeConverter"
)]
impl std::ops::Deref
for crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter {
    type Target = crate::System::ComponentModel::TypeConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ExtendedProtectionPolicyTypeConverter"
)]
impl std::ops::DerefMut
for crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ExtendedProtectionPolicyTypeConverter"
)]
impl crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter {
    pub fn CanConvertTo(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanConvertTo", (context, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertTo(
        &mut self,
        context: *mut crate::System::ComponentModel::ITypeDescriptorContext,
        culture: *mut crate::System::Globalization::CultureInfo,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        destinationType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("ConvertTo", (context, culture, value, destinationType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Security+Authentication+ExtendedProtection+ExtendedProtectionPolicyTypeConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
