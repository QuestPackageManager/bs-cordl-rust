#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
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
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "SimulatedTlsSrpIdentityManager";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager {
    pub fn GetLoginParameters(
        &mut self,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
                        >,
                        1usize,
                    >("GetLoginParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLoginParameters", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (identity))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    ), quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager,
                    >, 2usize>("GetRfc5054Default")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetRfc5054Default",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager,
        > = unsafe { cordl_method_info.invoke_unchecked((), (group, seedKey))? };
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
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6VerifierGenerator,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (group, verifierGenerator, mac))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager>
    for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+SimulatedTlsSrpIdentityManager")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager>
    for crate::Org::BouncyCastle::Crypto::Tls::SimulatedTlsSrpIdentityManager
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Tls::TlsSrpIdentityManager {
        unsafe { std::mem::transmute(self) }
    }
}
