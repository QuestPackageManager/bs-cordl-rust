#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed448 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed448"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    pub const C_d: i32 = -39081i32;
    pub const L4_0: i32 = 43969588i32;
    pub const L4_1: i32 = 30366549i32;
    pub const L4_2: i32 = 163752818i32;
    pub const L4_3: i32 = 258169998i32;
    pub const L4_4: i32 = 96434764i32;
    pub const L4_5: i32 = 227822194i32;
    pub const L4_6: i32 = 149865618i32;
    pub const L4_7: i32 = 550336261i32;
    pub const L_0: i32 = 78101261i32;
    pub const L_1: i32 = 141809365i32;
    pub const L_2: i32 = 175155932i32;
    pub const L_3: i32 = 64542499i32;
    pub const L_4: i32 = 158326419i32;
    pub const L_5: i32 = 191173276i32;
    pub const L_6: i32 = 104575268i32;
    pub const L_7: i32 = 137584065i32;
    pub const M26UL: u64 = 67108863u64;
    pub const M28UL: u64 = 268435455u64;
    pub const PointBytes: i32 = 57i32;
    pub const PrecompBlocks: i32 = 5i32;
    pub const PrecompMask: i32 = 15i32;
    pub const PrecompPoints: i32 = 16i32;
    pub const PrecompSpacing: i32 = 18i32;
    pub const PrecompTeeth: i32 = 5i32;
    pub const ScalarBytes: i32 = 57i32;
    pub const ScalarUints: i32 = 14i32;
    pub const WnafWidthBase: i32 = 7i32;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
    pub type Algorithm = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
    pub type PointExt = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
    pub type PointPrecomp = crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp;
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ed448_Algorithm {
    Ed448 = 0i32,
    Ed448ph = 1i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+Algorithm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_Algorithm =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed448/Algorithm"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed448_PointExt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub y: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub z: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed448/PointExt"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointExt")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointExt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
#[repr(C)]
#[derive(Debug)]
pub struct Ed448_PointPrecomp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub y: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp =>
    "Org.BouncyCastle.Math.EC.Rfc8032"."Ed448/PointPrecomp"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc8032+Ed448+PointPrecomp")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc8032::Ed448_PointPrecomp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
