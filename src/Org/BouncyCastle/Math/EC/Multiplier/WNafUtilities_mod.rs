#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_ConfigureBasepointCallback {
    __cordl_parent: crate::System::Object,
    pub m_curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    pub m_confWidth: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback
    => "Org.BouncyCastle.Math.EC.Multiplier"."WNafUtilities/ConfigureBasepointCallback"
);
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+ConfigureBasepointCallback"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_ConfigureBasepointCallback {
    type Target = crate::System::Object;
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
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        confWidth: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (curve, confWidth))?;
        Ok(__cordl_object)
    }
    pub fn Precompute(
        &mut self,
        existing: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo = __cordl_object
            .invoke("Precompute", (existing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        curve: *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
        confWidth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (curve, confWidth))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_MapPointCallback {
    __cordl_parent: crate::System::Object,
    pub m_infoP: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
    pub m_includeNegated: bool,
    pub m_pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback =>
    "Org.BouncyCastle.Math.EC.Multiplier"."WNafUtilities/MapPointCallback"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+MapPointCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_MapPointCallback {
    type Target = crate::System::Object;
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
        infoP: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        includeNegated: bool,
        pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (infoP, includeNegated, pointMap))?;
        Ok(__cordl_object)
    }
    pub fn Precompute(
        &mut self,
        existing: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo = __cordl_object
            .invoke("Precompute", (existing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        infoP: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        includeNegated: bool,
        pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (infoP, includeNegated, pointMap))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_PrecomputeCallback {
    __cordl_parent: crate::System::Object,
    pub m_p: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    pub m_minWidth: i32,
    pub m_includeNegated: bool,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback =>
    "Org.BouncyCastle.Math.EC.Multiplier"."WNafUtilities/PrecomputeCallback"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback {
    type Target = crate::System::Object;
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
        existingWNaf: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
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
        Ok(__cordl_ret)
    }
    pub fn CheckTable(
        &mut self,
        table: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        >,
        reqLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckTable", (table, reqLen))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        p: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        minWidth: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (p, minWidth, includeNegated))?;
        Ok(__cordl_object)
    }
    pub fn Precompute(
        &mut self,
        existing: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo = __cordl_object
            .invoke("Precompute", (existing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        minWidth: i32,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (p, minWidth, includeNegated))?;
        Ok(__cordl_ret)
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
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities_PrecomputeWithPointMapCallback {
    __cordl_parent: crate::System::Object,
    pub m_point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    pub m_pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
    pub m_fromWNaf: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
    pub m_includeNegated: bool,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback
    => "Org.BouncyCastle.Math.EC.Multiplier"
    ."WNafUtilities/PrecomputeWithPointMapCallback"
);
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback {
    type Target = crate::System::Object;
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
        existingWNaf: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
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
        Ok(__cordl_ret)
    }
    pub fn CheckTable(
        &mut self,
        table: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        >,
        reqLen: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckTable", (table, reqLen))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
        fromWNaf: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (point, pointMap, fromWNaf, includeNegated))?;
        Ok(__cordl_object)
    }
    pub fn Precompute(
        &mut self,
        existing: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo = __cordl_object
            .invoke("Precompute", (existing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        pointMap: *mut crate::Org::BouncyCastle::Math::EC::ECPointMap,
        fromWNaf: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::WNafPreCompInfo,
        includeNegated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (point, pointMap, fromWNaf, includeNegated))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct WNafUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities =>
    "Org.BouncyCastle.Math.EC.Multiplier"."WNafUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities {
    type Target = crate::System::Object;
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
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeWithPointMapCallback"
    )]
    pub type PrecomputeWithPointMapCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeWithPointMapCallback;
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Multiplier+WNafUtilities+PrecomputeCallback"
    )]
    pub type PrecomputeCallback = crate::Org::BouncyCastle::Math::EC::Multiplier::WNafUtilities_PrecomputeCallback;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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