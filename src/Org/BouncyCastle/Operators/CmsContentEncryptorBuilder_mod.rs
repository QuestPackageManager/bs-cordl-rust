#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsContentEncryptorBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub encryptionOID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    >,
    pub keySize: i32,
    pub helper: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::EnvelopedDataHelper,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Operators";
    const CLASS_NAME: &'static str = "CmsContentEncryptorBuilder";
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
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
                >,
                0usize,
            >("Build")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Build", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetKeySize(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                >),
                i32,
                1usize,
            >("GetKeySize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetKeySize", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (oid)) };
        Ok(__cordl_ret.into())
    }
    pub fn New_DerObjectIdentifier0(
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptionOID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptionOID, keySize))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DerObjectIdentifier0(
        &mut self,
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encryptionOID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (encryptionOID, keySize))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
