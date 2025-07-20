#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultVerifierResult {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSigner: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "DefaultVerifierResult";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    pub fn IsVerified_Il2CppArray0(
        &mut self,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        bool,
                        1usize,
                    >("IsVerified")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsVerified", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (signature))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsVerified_i32_i32_1(
        &mut self,
        sig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        sigOff: i32,
        sigLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                            i32,
                        ),
                        bool,
                        3usize,
                    >("IsVerified")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsVerified", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (sig, sigOff, sigLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        signer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ISigner,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (signer))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IVerifier>
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IVerifier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+DefaultVerifierResult")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IVerifier>
for crate::Org::BouncyCastle::Crypto::Operators::DefaultVerifierResult {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IVerifier {
        unsafe { std::mem::transmute(self) }
    }
}
