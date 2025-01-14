#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct V2TbsCertListGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
    pub signature: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub issuer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Name,
    >,
    pub thisUpdate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::Time,
    >,
    pub nextUpdate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::Time,
    >,
    pub extensions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    >,
    pub crlEntries: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.X509";
    const CLASS_NAME: &'static str = "V2TbsCertListGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    pub fn AddCrlEntry_Asn1Sequence0(
        &mut self,
        crlEntry: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Asn1Sequence,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddCrlEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddCrlEntry", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (crlEntry))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCrlEntry_DerInteger_DerUtcTime_i32_1(
        &mut self,
        userCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        >,
        revocationDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerUtcTime,
        >,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerInteger,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerUtcTime,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddCrlEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddCrlEntry", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userCertificate, revocationDate, reason))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCrlEntry_DerInteger_Time_X509Extensions4(
        &mut self,
        userCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        >,
        revocationDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::Time,
        >,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerInteger,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::Time,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddCrlEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddCrlEntry", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userCertificate, revocationDate, extensions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCrlEntry_DerInteger_Time_i32_2(
        &mut self,
        userCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        >,
        revocationDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::Time,
        >,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerInteger,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::Time,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AddCrlEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddCrlEntry", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userCertificate, revocationDate, reason))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddCrlEntry_DerInteger_Time_i32_DerGeneralizedTime3(
        &mut self,
        userCertificate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        >,
        revocationDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::Time,
        >,
        reason: i32,
        invalidityDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerInteger,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::X509::Time,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("AddCrlEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddCrlEntry", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userCertificate, revocationDate, reason, invalidityDate),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTbsCertList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList,
                >,
                0usize,
            >("GenerateTbsCertList")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateTbsCertList", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetExtensions(
        &mut self,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetExtensions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetExtensions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (extensions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetIssuer(
        &mut self,
        issuer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::X509Name>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::X509Name,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetIssuer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetIssuer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (issuer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNextUpdate_DerUtcTime0(
        &mut self,
        nextUpdate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtcTime>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtcTime>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetNextUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetNextUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nextUpdate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNextUpdate_Time1(
        &mut self,
        nextUpdate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::Time>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::Time>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetNextUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetNextUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nextUpdate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSignature(
        &mut self,
        signature: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetSignature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSignature", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (signature))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetThisUpdate_DerUtcTime0(
        &mut self,
        thisUpdate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtcTime>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerUtcTime>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetThisUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetThisUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (thisUpdate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetThisUpdate_Time1(
        &mut self,
        thisUpdate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::Time>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::Time>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetThisUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetThisUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (thisUpdate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2TbsCertListGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
