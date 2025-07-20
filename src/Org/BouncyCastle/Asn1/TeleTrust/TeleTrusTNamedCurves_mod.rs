#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves";
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
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves {
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
    )]
    pub type BrainpoolP160r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
    )]
    pub type BrainpoolP160t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
    )]
    pub type BrainpoolP192r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
    )]
    pub type BrainpoolP192t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
    )]
    pub type BrainpoolP224r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
    )]
    pub type BrainpoolP224t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
    )]
    pub type BrainpoolP256r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
    )]
    pub type BrainpoolP256t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
    )]
    pub type BrainpoolP320r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
    )]
    pub type BrainpoolP320t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
    )]
    pub type BrainpoolP384r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
    )]
    pub type BrainpoolP384t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
    )]
    pub type BrainpoolP512r1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder;
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
    )]
    pub type BrainpoolP512t1Holder = crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder;
    pub fn ConfigureBasepoint(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        encoding: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECPoint,
                        >,
                        2usize,
                    >("ConfigureBasepoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConfigureBasepoint", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECPoint,
        > = unsafe { method.invoke_unchecked((), (curve, encoding))? };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureCurve(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve,
                        >,
                        1usize,
                    >("ConfigureCurve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConfigureCurve", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = unsafe { method.invoke_unchecked((), (curve))? };
        Ok(__cordl_ret.into())
    }
    pub fn DefineCurve(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        holder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DefineCurve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "DefineCurve", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (name, oid, holder))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromHex(
        hex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        1usize,
                    >("FromHex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FromHex", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { method.invoke_unchecked((), (hex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetByName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        1usize,
                    >("GetByName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetByName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetByOid(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        1usize,
                    >("GetByOid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetByOid", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked((), (oid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetName(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (oid))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOid_Il2CppString0(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >,
                        1usize,
                    >("GetOid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetOid", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = unsafe { method.invoke_unchecked((), (name))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOid_i16__cordl_bool1(
        curvesize: i16,
        twisted: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerObjectIdentifier>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i16, bool),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
                        >,
                        2usize,
                    >("GetOid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetOid", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        > = unsafe { method.invoke_unchecked((), (curvesize, twisted))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Names() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerable,
                        >,
                        0usize,
                    >("get_Names")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Names", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP160r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP160r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP160t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP160t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP160t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP192r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP192r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP192t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP192t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP192t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP224r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP224r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP224t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP224t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP224t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP256r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP256r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP256t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP256t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP256t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP320r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP320r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP320t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP320t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP320t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP384r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP384r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP384t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP384t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP384t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP512r1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP512r1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512r1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves_BrainpoolP512t1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Asn1.TeleTrust";
    const CLASS_NAME: &'static str = "TeleTrusTNamedCurves/BrainpoolP512t1Holder";
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
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
)]
impl crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder {
    pub fn CreateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X9::X9ECParameters>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
                        >,
                        0usize,
                    >("CreateParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves+BrainpoolP512t1Holder"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
