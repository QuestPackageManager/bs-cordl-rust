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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP160r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP160t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP160t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP192r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP192t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP192t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP224r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP224t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP224t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP256r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP256t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP256t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP320r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP320t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP320t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP384r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP384t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP384t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512r1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP512r1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves_BrainpoolP512t1Holder =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves/BrainpoolP512t1Holder"
);
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
        *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X9::X9ECParameters = __cordl_object
            .invoke("CreateParameters", ())?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct TeleTrusTNamedCurves {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves =>
    "Org.BouncyCastle.Asn1.TeleTrust"."TeleTrusTNamedCurves"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+TeleTrust+TeleTrusTNamedCurves")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::TeleTrust::TeleTrusTNamedCurves {
    type Target = crate::System::Object;
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
