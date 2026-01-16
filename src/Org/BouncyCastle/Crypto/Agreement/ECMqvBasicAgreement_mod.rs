#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct ECMqvBasicAgreement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub privParams: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters,
    >,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Agreement";
    const CLASS_NAME: &'static str = "ECMqvBasicAgreement";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    pub fn CalculateAgreement(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ICipherParameters,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        1usize,
                    >("CalculateAgreement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateAgreement", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger> =
            unsafe { cordl_method_info.invoke_unchecked(self, (pubKey))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalculateMqvAgreement(
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
        d1U: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        d2U: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        Q2U: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
        Q1V: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
        Q2V: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        6usize,
                    >("CalculateMqvAgreement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalculateMqvAgreement", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint> = unsafe {
            cordl_method_info.invoke_unchecked((), (parameters, d1U, d2U, Q2U, Q1V, Q2V))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetFieldSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFieldSize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::ICipherParameters,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parameters))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IBasicAgreement>
    for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IBasicAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IBasicAgreement>
    for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IBasicAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
