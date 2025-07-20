#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
#[repr(C)]
#[derive(Debug)]
pub struct GlvTypeAEndomorphism {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_parameters: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters,
    >,
    pub m_pointMap: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECPointMap,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Endo";
    const CLASS_NAME: &'static str = "GlvTypeAEndomorphism";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    pub fn DecomposeScalar(
        &mut self,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Math::BigInteger,
                                >,
                            >,
                        >,
                        1usize,
                    >("DecomposeScalar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecomposeScalar", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (k))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAParameters,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (curve, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_HasEfficientPointMap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_HasEfficientPointMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_HasEfficientPointMap", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_PointMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPointMap>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPointMap,
                        >,
                        0usize,
                    >("get_PointMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PointMap", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism>
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism>
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism>
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+GlvTypeAEndomorphism")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism>
for crate::Org::BouncyCastle::Math::EC::Endo::GlvTypeAEndomorphism {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Endo::GlvEndomorphism {
        unsafe { std::mem::transmute(self) }
    }
}
