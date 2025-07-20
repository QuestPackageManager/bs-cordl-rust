#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct NaccacheSternPrivateKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters,
    pub phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Parameters";
    const CLASS_NAME: &'static str = "NaccacheSternPrivateKeyParameters";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    pub fn New_ArrayList0(
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList1(
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ArrayList0(
        &mut self,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::ArrayList,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (g, n, lowerSigmaBound, smallPrimes, phiN))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList1(
        &mut self,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (g, n, lowerSigmaBound, smallPrimes, phiN))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PhiN(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_PhiN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PhiN", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SmallPrimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
                        0usize,
                    >("get_SmallPrimes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SmallPrimes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SmallPrimesList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
                        0usize,
                    >("get_SmallPrimesList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_SmallPrimesList", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
