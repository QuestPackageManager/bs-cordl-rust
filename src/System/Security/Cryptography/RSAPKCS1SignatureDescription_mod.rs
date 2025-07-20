#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct RSAPKCS1SignatureDescription {
    __cordl_parent: crate::System::Security::Cryptography::SignatureDescription,
    pub _hashAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography";
    const CLASS_NAME: &'static str = "RSAPKCS1SignatureDescription";
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
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl std::ops::Deref
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    type Target = crate::System::Security::Cryptography::SignatureDescription;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    pub fn New(
        hashAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        digestAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashAlgorithm, digestAlgorithm))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        hashAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        digestAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (hashAlgorithm, digestAlgorithm))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
