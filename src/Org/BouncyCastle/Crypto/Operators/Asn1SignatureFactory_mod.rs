#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1SignatureFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algID: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier>,
    pub algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub privateKey:
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter>,
    pub random: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Operators";
    const CLASS_NAME: &'static str = "Asn1SignatureFactory";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory {
    pub fn CreateCalculator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IStreamCalculator>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
                        >,
                        0usize,
                    >("CreateCalculator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateCalculator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_AsymmetricKeyParameter0(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, privateKey))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        random: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, privateKey, random))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString_AsymmetricKeyParameter0(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (algorithm, privateKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom1(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        random: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (algorithm, privateKey, random))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_AlgorithmDetails")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AlgorithmDetails", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureAlgNames() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >,
                        0usize,
                    >("get_SignatureAlgNames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SignatureAlgNames", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ISignatureFactory>
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ISignatureFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1SignatureFactory")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ISignatureFactory>
    for crate::Org::BouncyCastle::Crypto::Operators::Asn1SignatureFactory
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ISignatureFactory {
        unsafe { std::mem::transmute(self) }
    }
}
