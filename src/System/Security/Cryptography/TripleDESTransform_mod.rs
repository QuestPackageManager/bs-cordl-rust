#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct TripleDESTransform {
    __cordl_parent: crate::Mono::Security::Cryptography::SymmetricTransform,
    pub E1: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub D2: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub E3: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub D1: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub E2: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
    pub D3: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::DESTransform,
    >,
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::Cryptography::TripleDESTransform {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Security.Cryptography";
    const CLASS_NAME: &'static str = "TripleDESTransform";
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
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl std::ops::Deref for crate::System::Security::Cryptography::TripleDESTransform {
    type Target = crate::Mono::Security::Cryptography::SymmetricTransform;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl std::ops::DerefMut for crate::System::Security::Cryptography::TripleDESTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl crate::System::Security::Cryptography::TripleDESTransform {
    pub fn ECB(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ECB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ECB", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input, output))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetStrongKey() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GetStrongKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetStrongKey", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algo: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::TripleDES,
        >,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algo, encryption, key, iv))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algo: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::TripleDES,
        >,
        encryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::TripleDES,
                    >,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (algo, encryption, key, iv))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+TripleDESTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::TripleDESTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
