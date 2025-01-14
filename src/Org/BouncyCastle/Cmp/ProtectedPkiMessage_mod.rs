#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct ProtectedPkiMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pkiMessage: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cmp";
    const CLASS_NAME: &'static str = "ProtectedPkiMessage";
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
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    pub fn GetCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::X509::X509Certificate,
                        >,
                    >,
                >,
                0usize,
            >("GetCertificates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCertificates", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::X509::X509Certificate,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New_GeneralPkiMessage0(
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::GeneralPkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkiMessage))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PkiMessage1(
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pkiMessage))?;
        Ok(__cordl_object.into())
    }
    pub fn Process(
        &mut self,
        streamCalculator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::IStreamCalculator,
                >),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("Process")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Process", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (streamCalculator)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToAsn1Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
                >,
                0usize,
            >("ToAsn1Message")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToAsn1Message", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Verify_IVerifierFactory0(
        &mut self,
        verifierFactory: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IVerifierFactory,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::IVerifierFactory,
                >),
                bool,
                1usize,
            >("Verify")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Verify", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (verifierFactory))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Verify_PKMacBuilder_Il2CppArray1(
        &mut self,
        pkMacBuilder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crmf::PKMacBuilder,
        >,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crmf::PKMacBuilder,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                ),
                bool,
                2usize,
            >("Verify")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Verify", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pkMacBuilder, password))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GeneralPkiMessage0(
        &mut self,
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cmp::GeneralPkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Cmp::GeneralPkiMessage,
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
            method.invoke_unchecked(self, (pkiMessage))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PkiMessage1(
        &mut self,
        pkiMessage: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::PkiMessage,
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
            method.invoke_unchecked(self, (pkiMessage))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Body(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiBody>,
                0usize,
            >("get_Body")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Body", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiBody,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_HasPasswordBasedMacProtected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_HasPasswordBasedMacProtected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_HasPasswordBasedMacProtected", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Header(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
                >,
                0usize,
            >("get_Header")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Header", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiHeader,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+ProtectedPkiMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::ProtectedPkiMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
