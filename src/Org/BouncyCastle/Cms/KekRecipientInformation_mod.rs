#[cfg(feature = "Org+BouncyCastle+Cms+KekRecipientInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct KekRecipientInformation {
    __cordl_parent: crate::Org::BouncyCastle::Cms::RecipientInformation,
    pub info: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+KekRecipientInformation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::KekRecipientInformation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "KekRecipientInformation";
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
#[cfg(feature = "Org+BouncyCastle+Cms+KekRecipientInformation")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::KekRecipientInformation {
    type Target = crate::Org::BouncyCastle::Cms::RecipientInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KekRecipientInformation")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::KekRecipientInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KekRecipientInformation")]
impl crate::Org::BouncyCastle::Cms::KekRecipientInformation {
    pub fn GetContentStream(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsTypedStream>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::ICipherParameters,
                >),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::CmsTypedStream>,
                1usize,
            >("GetContentStream")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetContentStream", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsTypedStream,
        > = unsafe { method.invoke_unchecked(self, (key)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, secureReadable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
        >,
        secureReadable: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::CmsSecureReadable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Cms::KekRecipientInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Cms::CmsSecureReadable,
                    >,
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
            method.invoke_unchecked(self, (info, secureReadable))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KekRecipientInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::KekRecipientInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
