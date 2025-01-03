#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SimulatedTlsSrpIdentityManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mGroup: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
    >,
    pub mVerifierGenerator: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator,
    >,
    pub mMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager =>
    "Org.BouncyCastle.Crypto.Tls"."SimulatedTlsSrpIdentityManager"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    pub fn GetLoginParameters(
        &mut self,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
        > = __cordl_object.invoke("GetLoginParameters", (identity))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRfc5054Default(
        group: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
        seedKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRfc5054Default", (group, seedKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        group: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
        verifierGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator,
        >,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (group, verifierGenerator, mac))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        group: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
        verifierGenerator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator,
        >,
        mac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (group, verifierGenerator, mac))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager>
for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager>
for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
        unsafe { std::mem::transmute(self) }
    }
}
