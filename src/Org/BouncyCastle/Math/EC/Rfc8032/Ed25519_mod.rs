#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    pub const L0: i32 = -50998291i32;
    pub const L1: i32 = 19280294i32;
    pub const L2: i32 = 127719000i32;
    pub const L3: i32 = -6428113i32;
    pub const L4: i32 = 5343i32;
    pub const M28L: i64 = 268435455i64;
    pub const M32L: i64 = 4294967295i64;
    pub const PointBytes: i32 = 32i32;
    pub const PrecompBlocks: i32 = 8i32;
    pub const PrecompMask: i32 = 7i32;
    pub const PrecompPoints: i32 = 8i32;
    pub const PrecompSpacing: i32 = 8i32;
    pub const PrecompTeeth: i32 = 4i32;
    pub const ScalarBytes: i32 = 32i32;
    pub const ScalarUints: i32 = 8i32;
    pub const WnafWidthBase: i32 = 7i32;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+Algorithm")]
    pub type Algorithm = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_Algorithm;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
    pub type PointAccum = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
    pub type PointExt = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
    pub type PointPrecomp = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+Algorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ed25519_Algorithm {
    Ed25519 = 0i32,
    Ed25519ctx = 1i32,
    Ed25519ph = 2i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+Algorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_Algorithm =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/Algorithm"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519_PointAccum {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub y: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub z: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub u: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub v: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/PointAccum"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointAccum")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointAccum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519_PointExt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub y: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub z: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub t: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/PointExt"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointExt")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointExt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed25519_PointPrecomp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ypx_h: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub ymx_h: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub xyd: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed25519/PointPrecomp"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed25519+PointPrecomp")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed25519_PointPrecomp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
