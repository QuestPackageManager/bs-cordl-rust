#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_Curve25519Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_Curve25519Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/Curve25519Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_Curve25519Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_Curve25519Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_Curve25519Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_Curve25519Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves
    => "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves {
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+Curve25519Holder")]
    pub type Curve25519Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_Curve25519Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
    pub type SM2P256V1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SM2P256V1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
    pub type SecP128R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP128R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
    pub type SecP160K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
    pub type SecP160R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
    pub type SecP160R2Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R2Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
    pub type SecP192K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
    pub type SecP192R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
    pub type SecP224K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
    pub type SecP224R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
    pub type SecP256K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
    pub type SecP256R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
    pub type SecP384R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP384R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
    pub type SecP521R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP521R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
    pub type SecT113R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
    pub type SecT113R2Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R2Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
    pub type SecT131R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
    pub type SecT131R2Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R2Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
    pub type SecT163K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
    pub type SecT163R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
    pub type SecT163R2Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R2Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
    pub type SecT193R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
    pub type SecT193R2Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R2Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
    pub type SecT233K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
    pub type SecT233R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
    pub type SecT239K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT239K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
    pub type SecT283K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
    pub type SecT283R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
    pub type SecT409K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
    pub type SecT409R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409R1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
    pub type SecT571K1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571K1Holder;
    #[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
    pub type SecT571R1Holder = crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571R1Holder;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SM2P256V1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SM2P256V1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SM2P256V1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SM2P256V1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SM2P256V1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SM2P256V1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SM2P256V1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SM2P256V1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP128R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP128R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP128R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP128R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP128R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP128R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP128R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP128R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP160K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP160K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP160R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP160R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP160R2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R2Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP160R2Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R2Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP160R2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP160R2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP192K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP192K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP192R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP192R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP192R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP192R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP224K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP224K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP224R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP224R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP224R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP224R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP256K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP256K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP256R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP256R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP256R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP256R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP384R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP384R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP384R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP384R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP384R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP384R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP384R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP384R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecP521R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP521R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecP521R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP521R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP521R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP521R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecP521R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecP521R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT113R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT113R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT113R2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R2Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT113R2Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R2Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT113R2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT113R2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT131R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT131R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT131R2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R2Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT131R2Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R2Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT131R2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT131R2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT163K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT163K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT163R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT163R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT163R2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R2Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT163R2Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R2Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT163R2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT163R2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT193R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT193R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT193R2Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R2Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT193R2Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R2Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R2Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R2Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT193R2Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT193R2Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT233K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT233K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT233R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT233R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT233R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT233R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT239K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT239K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT239K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT239K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT239K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT239K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT239K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT239K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT283K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT283K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT283R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT283R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT283R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT283R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT409K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT409K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT409R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT409R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT409R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT409R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT571K1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571K1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT571K1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571K1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571K1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571K1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571K1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571K1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
#[repr(C)]
#[derive(Debug)]
pub struct CustomNamedCurves_SecT571R1Holder {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571R1Holder =>
    "Org.BouncyCastle.Crypto.EC"."CustomNamedCurves/SecT571R1Holder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571R1Holder {
    type Target = crate::Org::BouncyCastle::Asn1::X9::X9ECParametersHolder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571R1Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
impl crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571R1Holder {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+EC+CustomNamedCurves+SecT571R1Holder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::EC::CustomNamedCurves_SecT571R1Holder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
