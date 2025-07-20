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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Authentication.ExtendedProtection";
    const CLASS_NAME: &'static str = "ExtendedProtectionPolicyTypeConverter";
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
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::ITypeDescriptorContext,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                bool,
                2usize,
            >("CanConvertTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter
                    as quest_hook::libil2cpp::Type > ::class(), "CanConvertTo", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (context, destinationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTo(
        &mut self,
        context: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::ITypeDescriptorContext,
        >,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::ComponentModel::ITypeDescriptorContext,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                4usize,
            >("ConvertTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter
                    as quest_hook::libil2cpp::Type > ::class(), "ConvertTo", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method.invoke_unchecked(self, (context, culture, value, destinationType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Security::Authentication::ExtendedProtection::ExtendedProtectionPolicyTypeConverter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
