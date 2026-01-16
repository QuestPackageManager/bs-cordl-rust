#[cfg(feature = "cordl_class_System+Security+AccessControl+CommonAce")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonAce {
    __cordl_parent: crate::System::Security::AccessControl::QualifiedAce,
}
#[cfg(feature = "cordl_class_System+Security+AccessControl+CommonAce")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Security::AccessControl::CommonAce {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "CommonAce";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl std::ops::Deref for crate::System::Security::AccessControl::CommonAce {
    type Target = crate::System::Security::AccessControl::QualifiedAce;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::CommonAce {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+CommonAce")]
impl crate::System::Security::AccessControl::CommonAce {
    pub fn ConvertType(
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        isCallback: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::Security::AccessControl::AceType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Security::AccessControl::AceQualifier, bool),
                        crate::System::Security::AccessControl::AceType,
                        2usize,
                    >("ConvertType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertType", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Security::AccessControl::AceType =
            unsafe { cordl_method_info.invoke_unchecked((), (qualifier, isCallback))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBinaryForm(
        &mut self,
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("GetBinaryForm")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBinaryForm",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (binaryForm, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_AceFlags_AceQualifier_i32_SecurityIdentifier__cordl_bool_Il2CppArray0(
        flags: crate::System::Security::AccessControl::AceFlags,
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        accessMask: i32,
        sid: quest_hook::libil2cpp::Gc<crate::System::Security::Principal::SecurityIdentifier>,
        isCallback: bool,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (flags, qualifier, accessMask, sid, isCallback, opaque),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_i32_1(
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binaryForm, offset))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_AceFlags_AceQualifier_i32_SecurityIdentifier__cordl_bool_Il2CppArray0(
        &mut self,
        flags: crate::System::Security::AccessControl::AceFlags,
        qualifier: crate::System::Security::AccessControl::AceQualifier,
        accessMask: i32,
        sid: quest_hook::libil2cpp::Gc<crate::System::Security::Principal::SecurityIdentifier>,
        isCallback: bool,
        opaque: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::System::Security::AccessControl::AceFlags,
                        crate::System::Security::AccessControl::AceQualifier,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Security::Principal::SecurityIdentifier,
                        >,
                        bool,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    ), quest_hook::libil2cpp::Void, 6usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (flags, qualifier, accessMask, sid, isCallback, opaque),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_i32_1(
        &mut self,
        binaryForm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (binaryForm, offset))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_BinaryLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_BinaryLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_BinaryLength",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Security+AccessControl+CommonAce")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::AccessControl::CommonAce {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
