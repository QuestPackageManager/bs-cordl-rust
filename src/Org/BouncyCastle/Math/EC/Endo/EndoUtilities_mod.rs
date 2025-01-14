#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct EndoUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Endo";
    const CLASS_NAME: &'static str = "EndoUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
    pub type MapPointCallback = crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback;
    pub fn CalculateB(
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        t: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::BigInteger,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::BigInteger,
                    >,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                3usize,
            >("CalculateB")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CalculateB", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { method.invoke_unchecked((), (k, g, t)) };
        Ok(__cordl_ret.into())
    }
    pub fn DecomposeScalar(
        p: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
        >,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::Endo::ScalarSplitParameters,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                    >,
                >,
                2usize,
            >("DecomposeScalar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecomposeScalar", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
            >,
        > = unsafe { method.invoke_unchecked((), (p, k)) };
        Ok(__cordl_ret.into())
    }
    pub fn MapPoint(
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::ECPoint,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                2usize,
            >("MapPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MapPoint", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked((), (endomorphism, p)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct EndoUtilities_MapPointCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_endomorphism: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
    >,
    pub m_point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Endo";
    const CLASS_NAME: &'static str = "EndoUtilities/MapPointCallback";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    pub fn CheckExisting(
        &mut self,
        existingEndo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::EndoPreCompInfo,
        >,
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::Endo::EndoPreCompInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
                    >,
                ),
                bool,
                2usize,
            >("CheckExisting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckExisting", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (existingEndo, endomorphism))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (endomorphism, point))?;
        Ok(__cordl_object.into())
    }
    pub fn Precompute(
        &mut self,
        existing: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
                >,
                1usize,
            >("Precompute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Precompute", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = unsafe { method.invoke_unchecked(self, (existing)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::ECPoint,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (endomorphism, point))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Endo+EndoUtilities+MapPointCallback")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Endo::EndoUtilities_MapPointCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
