#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpGroupVerifier")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSrpGroupVerifier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpGroupVerifier")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "TlsSrpGroupVerifier";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpGroupVerifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpGroupVerifier")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpGroupVerifier")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier {
    pub fn Accept(
        &mut self,
        group: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
                >),
                bool,
                1usize,
            >("Accept")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier as
                    quest_hook::libil2cpp::Type > ::class(), "Accept", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (group))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpGroupVerifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpGroupVerifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
