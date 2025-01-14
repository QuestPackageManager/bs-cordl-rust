#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeEncryptionKeyPreferenceAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SmimeEncryptionKeyPreferenceAttribute {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X509::AttributeX509,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeEncryptionKeyPreferenceAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::Smime::SmimeEncryptionKeyPreferenceAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.Smime";
    const CLASS_NAME: &'static str = "SmimeEncryptionKeyPreferenceAttribute";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeEncryptionKeyPreferenceAttribute")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::Smime::SmimeEncryptionKeyPreferenceAttribute {
    type Target = crate::Org::BouncyCastle::Asn1::X509::AttributeX509;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeEncryptionKeyPreferenceAttribute")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Smime::SmimeEncryptionKeyPreferenceAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeEncryptionKeyPreferenceAttribute")]
impl crate::Org::BouncyCastle::Asn1::Smime::SmimeEncryptionKeyPreferenceAttribute {
    pub fn New_Asn1OctetString2(
        sKeyID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sKeyID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IssuerAndSerialNumber0(
        issAndSer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issAndSer))?;
        Ok(__cordl_object.into())
    }
    pub fn New_RecipientKeyIdentifier1(
        rKeyID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rKeyID))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Asn1OctetString2(
        &mut self,
        sKeyID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Asn1OctetString,
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
            method.invoke_unchecked(self, (sKeyID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IssuerAndSerialNumber0(
        &mut self,
        issAndSer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
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
            method.invoke_unchecked(self, (issAndSer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RecipientKeyIdentifier1(
        &mut self,
        rKeyID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cms::RecipientKeyIdentifier,
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
            method.invoke_unchecked(self, (rKeyID))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Smime+SmimeEncryptionKeyPreferenceAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Smime::SmimeEncryptionKeyPreferenceAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
