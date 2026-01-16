#[cfg(feature = "cordl_class_System+Security+AccessControl+SystemAcl")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemAcl {
    __cordl_parent: crate::System::Security::AccessControl::CommonAcl,
}
#[cfg(feature = "cordl_class_System+Security+AccessControl+SystemAcl")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Security::AccessControl::SystemAcl {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "SystemAcl";
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
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl std::ops::Deref for crate::System::Security::AccessControl::SystemAcl {
    type Target = crate::System::Security::AccessControl::CommonAcl;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl std::ops::DerefMut for crate::System::Security::AccessControl::SystemAcl {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+AccessControl+SystemAcl")]
impl crate::System::Security::AccessControl::SystemAcl {
    pub fn ApplyCanonicalSortToExplicitAces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ApplyCanonicalSortToExplicitAces",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ApplyCanonicalSortToExplicitAces",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAceInsertPosition(
        &mut self,
        aceQualifier: crate::System::Security::AccessControl::AceQualifier,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Security::AccessControl::AceQualifier),
                        i32,
                        1usize,
                    >("GetAceInsertPosition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAceInsertPosition", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (aceQualifier))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Security+AccessControl+SystemAcl")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Security::AccessControl::SystemAcl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
