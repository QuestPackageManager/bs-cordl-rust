#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaPrivateCrtKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    pub e: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub dP: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub dQ: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub qInv: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Parameters";
    const CLASS_NAME: &'static str = "RsaPrivateCrtKeyParameters";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl std::ops::DerefMut
    for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger0(
        modulus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        publicExponent: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        privateExponent: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        dP: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        dQ: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        qInv: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (modulus, publicExponent, privateExponent, p, q, dP, dQ, qInv),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn New_RsaPrivateKeyStructure1(
        rsaPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rsaPrivateKey))?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateValue(
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        desc: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 3usize>("ValidateValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (x, name, desc))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger0(
        &mut self,
        modulus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        publicExponent: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        privateExponent: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        dP: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        dQ: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        qInv: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                    ), quest_hook::libil2cpp::Void, 8usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (modulus, publicExponent, privateExponent, p, q, dP, dQ, qInv),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RsaPrivateKeyStructure1(
        &mut self,
        rsaPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rsaPrivateKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DP(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_DP")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_DP",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DQ(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_DQ")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_DQ",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_P(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_P")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_P",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicExponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_PublicExponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PublicExponent", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_Q")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_Q",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_QInv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_QInv")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_QInv", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
