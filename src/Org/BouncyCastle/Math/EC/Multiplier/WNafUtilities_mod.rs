#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WNafUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities {
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
    )]
    pub type ConfigureBasepointCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback;
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback"
    )]
    pub type MapPointCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback;
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback"
    )]
    pub type PrecomputeCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback;
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
    )]
    pub type PrecomputeWithPointMapCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback;
    pub fn ConfigureBasepoint(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConfigureBasepoint", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCompactNaf(
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateCompactNaf", (k))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateCompactWindowNaf(
        width: i32,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateCompactWindowNaf", (width, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateJsf(
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        h: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateJsf", (g, h))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateNaf(
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GenerateNaf", (k))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateWindowNaf(
        width: i32,
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateWindowNaf", (width, k))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNafWeight(
        k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNafWeight", (k))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWNafPreCompInfo_ECPoint0(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWNafPreCompInfo", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWNafPreCompInfo_PreCompInfo1(
        preCompInfo: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWNafPreCompInfo", (preCompInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindowSize_Il2CppArray2(
        bits: i32,
        windowSizeCutoffs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindowSize", (bits, windowSizeCutoffs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindowSize_Il2CppArray_i32_3(
        bits: i32,
        windowSizeCutoffs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        maxWidth: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindowSize", (bits, windowSizeCutoffs, maxWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindowSize_i32_0(bits: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindowSize", (bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindowSize_i32_1(
        bits: i32,
        maxWidth: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindowSize", (bits, maxWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn MapPointWithPrecomp(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        minWidth: i32,
        includeNegated: bool,
        pointMap: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MapPointWithPrecomp", (p, minWidth, includeNegated, pointMap))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Precompute(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        minWidth: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Precompute", (p, minWidth, includeNegated))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrecomputeWithPointMap(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        pointMap: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        >,
        fromWNaf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrecomputeWithPointMap", (p, pointMap, fromWNaf, includeNegated))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeTable(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResizeTable", (a, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim_Il2CppArray_i32_0(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Trim", (a, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim_Il2CppArray_i32_1(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Trim", (a, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_ConfigureBasepointCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    pub m_confWidth: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WNafUtilities/ConfigureBasepointCallback";
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
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    pub fn New(
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        confWidth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, confWidth))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = __cordl_object.invoke("Precompute", (existing))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        curve: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        confWidth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, confWidth))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_MapPointCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_infoP: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
    >,
    pub m_includeNegated: bool,
    pub m_pointMap: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECPointMap,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WNafUtilities/MapPointCallback";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    pub fn New(
        infoP: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        includeNegated: bool,
        pointMap: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (infoP, includeNegated, pointMap))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = __cordl_object.invoke("Precompute", (existing))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        infoP: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        includeNegated: bool,
        pointMap: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (infoP, includeNegated, pointMap))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_PrecomputeCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    pub m_minWidth: i32,
    pub m_includeNegated: bool,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WNafUtilities/PrecomputeCallback";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    pub fn CheckExisting(
        &mut self,
        existingWNaf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        width: i32,
        reqPreCompLen: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CheckExisting",
                (existingWNaf, width, reqPreCompLen, includeNegated),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
        reqLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckTable", (table, reqLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        minWidth: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, minWidth, includeNegated))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = __cordl_object.invoke("Precompute", (existing))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        minWidth: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, minWidth, includeNegated))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_PrecomputeWithPointMapCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    pub m_pointMap: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECPointMap,
    >,
    pub m_fromWNaf: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
    >,
    pub m_includeNegated: bool,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Multiplier";
    const CLASS_NAME: &'static str = "WNafUtilities/PrecomputeWithPointMapCallback";
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
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    pub fn CheckExisting(
        &mut self,
        existingWNaf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        width: i32,
        reqPreCompLen: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CheckExisting",
                (existingWNaf, width, reqPreCompLen, includeNegated),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckTable(
        &mut self,
        table: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
        reqLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckTable", (table, reqLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        pointMap: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        >,
        fromWNaf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (point, pointMap, fromWNaf, includeNegated))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = __cordl_object.invoke("Precompute", (existing))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        pointMap: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPointMap,
        >,
        fromWNaf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        >,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (point, pointMap, fromWNaf, includeNegated))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl AsRef<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl AsMut<crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback>
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback {
        unsafe { std::mem::transmute(self) }
    }
}
