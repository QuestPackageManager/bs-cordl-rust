#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
#[repr(C)]
#[derive(Debug)]
pub struct Arm {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::Arm {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "Arm";
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
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::Arm {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::Arm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl crate::Unity::Burst::Intrinsics::Arm {
    #[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
    pub type Neon = crate::Unity::Burst::Intrinsics::Arm_Neon;
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::Arm {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
#[repr(C)]
#[derive(Debug)]
pub struct Arm_Neon {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::Arm_Neon {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "Arm/Neon";
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
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::Arm_Neon {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::Arm_Neon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl crate::Unity::Burst::Intrinsics::Arm_Neon {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __crc32b(a0: u32, a1: u8) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32b", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32cb(a0: u32, a1: u8) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32cb", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32cd(a0: u32, a1: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32cd", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32ch(a0: u32, a1: u16) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32ch", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32cw(a0: u32, a1: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32cw", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32d(a0: u32, a1: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32d", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32h(a0: u32, a1: u16) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32h", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __crc32w(a0: u32, a1: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__crc32w", (a0, a1))?;
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
    pub fn get_IsNeonArmv82FeaturesSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsNeonArmv82FeaturesSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNeonCryptoSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsNeonCryptoSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNeonDotProdSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsNeonDotProdSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNeonRDMASupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsNeonRDMASupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNeonSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsNeonSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn vaba_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaba_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaba_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaba_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaba_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaba_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaba_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaba_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaba_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaba_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaba_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaba_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_high_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_high_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabal_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabal_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabaq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabaq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabaq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabaq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabaq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabaq_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabaq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabaq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabaq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabaq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabaq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabaq_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_high_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_high_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabdq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabdq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabds_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabds_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabs_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabs_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabs_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabs_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabs_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabs_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabs_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabs_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabs_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabs_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabs_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsd_s64(a0: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vabsq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vabsq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vadd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vadd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_high_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_high_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddhn_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddhn_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_high_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_high_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlv_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlv_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlv_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlv_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlv_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlv_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlv_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlv_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlv_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlv_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlv_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlv_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlvq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlvq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlvq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlvq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlvq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlvq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlvq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlvq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlvq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlvq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddlvq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddlvq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddv_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddv_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddvq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddvq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_high_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_high_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaddw_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaddw_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaesdq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaesdq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaeseq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaeseq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaesimcq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaesimcq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vaesmcq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vaesmcq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vand_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vand_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vandq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vandq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbic_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbic_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbicq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbicq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbsl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbsl_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vbslq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vbslq_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcage_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcage_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcage_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcage_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaged_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaged_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcageq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcageq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcageq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcageq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcages_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcages_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcagt_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcagt_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcagt_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcagt_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcagtd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcagtd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcagtq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcagtq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcagtq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcagtq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcagts_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcagts_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcale_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcale_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcale_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcale_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaled_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaled_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaleq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaleq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaleq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaleq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcales_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcales_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcalt_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcalt_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcalt_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcalt_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaltd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaltd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaltq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaltq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcaltq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcaltq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcalts_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcalts_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceq_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqs_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqs_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqz_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqz_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzd_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzd_s64(a0: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzd_u64(a0: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzd_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vceqzs_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vceqzs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcge_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcge_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcged_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcged_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcged_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcged_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcged_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcged_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgeq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgeq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcges_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcges_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgez_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgez_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgez_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgez_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgez_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgez_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgez_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgez_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgez_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgez_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgez_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgez_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezd_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezd_s64(a0: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgezs_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgezs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgt_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgt_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgts_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgts_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtz_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtz_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtz_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtz_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtz_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtz_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtz_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtz_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtz_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtz_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtz_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtz_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzd_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzd_s64(a0: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcgtzs_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcgtzs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcle_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcle_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcled_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcled_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcled_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcled_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcled_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcled_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcleq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcleq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcles_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcles_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclez_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclez_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclez_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclez_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclez_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclez_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclez_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclez_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclez_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclez_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclez_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclez_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezd_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezd_s64(a0: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclezs_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclezs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcls_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcls_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcls_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcls_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcls_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcls_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclsq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclsq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclsq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclsq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclsq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclsq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclt_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclt_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclts_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclts_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltz_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltz_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltz_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltz_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltz_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltz_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltz_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltz_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltz_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltz_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltz_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltz_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzd_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzd_s64(a0: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcltzs_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcltzs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclz_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclz_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclz_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclz_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclz_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclz_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclz_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclz_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclz_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclz_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclz_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclz_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclzq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclzq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclzq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclzq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclzq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclzq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclzq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclzq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclzq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclzq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vclzq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vclzq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcnt_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcnt_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcnt_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcnt_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcntq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcntq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcntq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcntq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_f16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_f16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcombine_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcombine_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_s64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_s8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_u64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_lane_u8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_s64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_s8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_u64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopy_laneq_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopy_laneq_u8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_s64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_s8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_u64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_lane_u8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_s64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_s8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_u64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcopyq_laneq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcopyq_laneq_u8", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_f16(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_f16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_f32(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_f64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_s16(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_s32(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_s64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_s8(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_u16(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_u32(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_u64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcreate_u8(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcreate_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_f32_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_f32_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_f32_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_f32_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_f32_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_f32_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_f64_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_f64_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_f64_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_f64_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_f64_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_f64_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_high_f32_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_high_f32_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_high_f64_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_high_f64_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_f32_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_f32_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_f32_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_f32_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_f64_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_f64_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_f64_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_f64_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_s32_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_s64_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_u32_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_n_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_n_u64_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvt_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvt_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvta_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvta_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvta_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvta_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvta_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvta_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvta_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvta_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtad_s64_f64(a0: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtad_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtad_u64_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtad_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtaq_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtaq_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtaq_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtaq_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtaq_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtaq_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtaq_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtaq_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtas_s32_f32(a0: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtas_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtas_u32_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtas_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_f64_s64(a0: i64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_f64_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_f64_u64(a0: u64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_f64_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_n_f64_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_n_f64_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_n_f64_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_n_f64_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_n_s64_f64(a0: f64, a1: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_n_s64_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_n_u64_f64(a0: f64, a1: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_n_u64_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_s64_f64(a0: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtd_u64_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtd_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtm_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtm_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtm_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtm_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtm_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtm_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtm_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtm_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtmd_s64_f64(a0: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtmd_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtmd_u64_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtmd_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtmq_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtmq_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtmq_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtmq_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtmq_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtmq_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtmq_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtmq_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtms_s32_f32(a0: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtms_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtms_u32_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtms_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtn_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtn_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtn_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtn_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtn_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtn_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtn_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtn_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtnd_s64_f64(a0: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtnd_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtnd_u64_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtnd_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtnq_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtnq_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtnq_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtnq_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtnq_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtnq_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtnq_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtnq_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtns_s32_f32(a0: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtns_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtns_u32_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtns_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtp_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtp_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtp_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtp_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtp_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtp_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtp_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtp_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtpd_s64_f64(a0: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtpd_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtpd_u64_f64(a0: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtpd_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtpq_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtpq_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtpq_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtpq_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtpq_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtpq_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtpq_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtpq_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtps_s32_f32(a0: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtps_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtps_u32_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtps_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_f32_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_f32_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_f32_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_f32_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_f64_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_f64_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_f64_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_f64_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_f32_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_f32_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_f32_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_f32_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_f64_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_f64_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_f64_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_f64_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_s32_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_s64_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_u32_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_n_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_n_u64_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_s32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_s64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_s64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_u32_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtq_u64_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtq_u64_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_f32_s32(a0: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_f32_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_f32_u32(a0: u32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_f32_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_n_f32_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_n_f32_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_n_f32_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_n_f32_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_n_s32_f32(a0: f32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_n_s32_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_n_u32_f32(a0: f32, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_n_u32_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_s32_f32(a0: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_s32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvts_u32_f32(a0: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvts_u32_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtx_f32_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtx_f32_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtx_high_f32_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtx_high_f32_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vcvtxd_f32_f64(a0: f64) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vcvtxd_f32_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdiv_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdiv_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdiv_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdiv_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdivq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdivq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdivq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdivq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdot_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdot_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdot_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdot_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdot_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdot_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdot_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdot_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdot_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdot_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdot_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdot_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdotq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdotq_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdotq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdotq_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdotq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdotq_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdotq_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdotq_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdotq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdotq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdotq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdotq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_lane_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_laneq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_laneq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_f32(
        a0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_f64(
        a0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_s16(
        a0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_s32(
        a0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_s64(
        a0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_s8(
        a0: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_u16(
        a0: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_u32(
        a0: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_u64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdup_n_u8(
        a0: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdup_n_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupb_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupb_lane_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupb_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupb_lane_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupb_laneq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupb_laneq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupb_laneq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupb_laneq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupd_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupd_lane_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupd_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupd_lane_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupd_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupd_lane_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupd_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupd_laneq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupd_laneq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupd_laneq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupd_laneq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupd_laneq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vduph_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vduph_lane_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vduph_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vduph_lane_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vduph_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vduph_laneq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vduph_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vduph_laneq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_lane_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_laneq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_laneq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_f32(
        a0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_f64(
        a0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_s16(
        a0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_s32(
        a0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_s64(
        a0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_s8(
        a0: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_u16(
        a0: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_u32(
        a0: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_u64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdupq_n_u8(
        a0: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdupq_n_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdups_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdups_lane_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdups_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdups_lane_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdups_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdups_lane_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdups_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdups_laneq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdups_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdups_laneq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vdups_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vdups_laneq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veor_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veor_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn veorq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("veorq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vext_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vext_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vextq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vextq_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfma_n_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfma_n_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmad_lane_f64(
        a0: f64,
        a1: f64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmad_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmad_laneq_f64(
        a0: f64,
        a1: f64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmad_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmaq_n_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmaq_n_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmas_lane_f32(
        a0: f32,
        a1: f32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmas_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmas_laneq_f32(
        a0: f32,
        a1: f32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmas_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfms_n_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfms_n_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsd_lane_f64(
        a0: f64,
        a1: f64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsd_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsd_laneq_f64(
        a0: f64,
        a1: f64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsd_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_lane_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_laneq_f64", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmsq_n_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmsq_n_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmss_lane_f32(
        a0: f32,
        a1: f32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmss_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vfmss_laneq_f32(
        a0: f32,
        a1: f32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vfmss_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_high_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_lane_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vget_low_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vget_low_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vgetq_lane_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vgetq_lane_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhadd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhadd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhadd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhadd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhadd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhadd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhadd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhadd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhadd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhadd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhadd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhadd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhaddq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhaddq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhaddq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhaddq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhaddq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhaddq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhaddq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhaddq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhaddq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhaddq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhaddq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhaddq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsub_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsub_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsub_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsub_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsub_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsub_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsub_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsub_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsub_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsub_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsub_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsub_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsubq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsubq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsubq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsubq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsubq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsubq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsubq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsubq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsubq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsubq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vhsubq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vhsubq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_f32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_f64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_s16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_s32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_s64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_s8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_u16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_u32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_u64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1_u8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_f32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_f64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_s16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_s32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_s64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_s8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_u16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_u32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_u64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vld1q_u8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vld1q_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmax_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmax_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnm_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnm_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnm_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnm_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnmq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnmq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnmq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnmq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnmv_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnmv_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnmvq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnmvq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxnmvq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxnmvq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxv_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxv_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmaxvq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmaxvq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmin_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmin_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnm_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnm_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnm_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnm_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnmq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnmq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnmq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnmq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnmv_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnmv_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnmvq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnmvq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminnmvq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminnmvq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminv_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminv_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vminvq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vminvq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmla_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmla_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_high_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlal_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlal_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlaq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlaq_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmls_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmls_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_high_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsl_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsl_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_lane_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_lane_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_lane_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_laneq_f32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_laneq_u16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_laneq_u32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_n_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmlsq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmlsq_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_f32(
        a0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_f64(
        a0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_s16(
        a0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_s32(
        a0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_s64(
        a0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_s8(
        a0: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_u16(
        a0: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_u32(
        a0: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_u64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmov_n_u8(
        a0: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmov_n_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_high_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_high_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_high_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_high_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_high_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_high_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovl_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_high_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_high_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovn_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovn_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_f32(
        a0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_f64(
        a0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_s16(
        a0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_s32(
        a0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_s64(
        a0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_s8(
        a0: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_u16(
        a0: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_u32(
        a0: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_u64(
        a0: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmovq_n_u8(
        a0: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmovq_n_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_lane_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_lane_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_laneq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_laneq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_laneq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_laneq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_n_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_n_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_n_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmul_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmul_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmuld_lane_f64(
        a0: f64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmuld_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmuld_laneq_f64(
        a0: f64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmuld_laneq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_lane_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_lane_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_laneq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_laneq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_high_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_lane_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_lane_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_laneq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_laneq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmull_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmull_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_lane_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_lane_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_lane_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_lane_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_laneq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_laneq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_laneq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_laneq_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_laneq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_laneq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_n_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_n_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_n_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_n_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmuls_lane_f32(
        a0: f32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmuls_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmuls_laneq_f32(
        a0: f32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmuls_laneq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulx_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulx_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulx_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulx_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulx_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulx_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulx_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulx_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulx_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulx_laneq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulx_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulx_laneq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxd_lane_f64(
        a0: f64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxd_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxd_laneq_f64(
        a0: f64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxd_laneq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxq_lane_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxq_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxq_lane_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxq_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxq_laneq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxq_laneq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxq_laneq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxq_laneq_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxs_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxs_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxs_lane_f32(
        a0: f32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxs_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmulxs_laneq_f32(
        a0: f32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmulxs_laneq_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvn_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvn_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvn_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvn_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvn_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvn_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvn_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvn_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvn_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvn_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvn_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvn_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvnq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvnq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvnq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvnq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvnq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvnq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvnq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvnq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvnq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvnq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vmvnq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vmvnq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vneg_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vneg_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vneg_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vneg_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vneg_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vneg_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vneg_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vneg_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vneg_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vneg_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vneg_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vneg_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegd_s64(a0: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vnegq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vnegq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorn_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorn_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vornq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vornq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorr_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorr_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vorrq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vorrq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadal_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadal_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadal_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadal_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadal_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadal_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadal_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadal_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadal_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadal_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadal_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadal_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadalq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadalq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadalq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadalq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadalq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadalq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadalq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadalq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadalq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadalq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadalq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadalq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddd_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddd_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddd_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddd_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddl_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddl_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddl_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddl_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddl_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddl_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddlq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddlq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddlq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddlq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddlq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddlq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddlq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddlq_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddlq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddlq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddlq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddlq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpaddq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpaddq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpadds_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpadds_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmax_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmax_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxnm_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxnm_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxnmq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxnmq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxnmq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxnmq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxnmqd_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxnmqd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxnms_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxnms_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxqd_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxqd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmaxs_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmaxs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmin_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmin_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminnm_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminnm_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminnmq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminnmq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminnmq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminnmq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminnmqd_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminnmqd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminnms_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminnms_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpminqd_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpminqd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vpmins_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vpmins_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabs_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabs_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabs_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabs_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabs_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabs_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabs_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabs_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsb_s8(a0: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsb_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsd_s64(a0: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsh_s16(a0: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsh_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabsq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabsq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqabss_s32(a0: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqabss_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddb_s8(a0: i8, a1: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddb_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddb_u8(a0: u8, a1: u8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddb_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddh_u16(a0: u16, a1: u16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddh_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqaddq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqaddq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadds_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadds_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqadds_u32(a0: u32, a1: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqadds_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlal_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlal_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlalh_lane_s16(
        a0: i32,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlalh_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlalh_laneq_s16(
        a0: i32,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlalh_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlalh_s16(
        a0: i32,
        a1: i16,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlalh_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlals_lane_s32(
        a0: i64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlals_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlals_laneq_s32(
        a0: i64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlals_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlals_s32(
        a0: i64,
        a1: i32,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlals_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsl_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsl_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlslh_lane_s16(
        a0: i32,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlslh_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlslh_laneq_s16(
        a0: i32,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlslh_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlslh_s16(
        a0: i32,
        a1: i16,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlslh_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsls_lane_s32(
        a0: i64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsls_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsls_laneq_s32(
        a0: i64,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsls_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmlsls_s32(
        a0: i64,
        a1: i32,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmlsls_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulh_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulh_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhh_lane_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhh_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhh_laneq_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhh_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhs_lane_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhs_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhs_laneq_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhs_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulhs_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulhs_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmull_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmull_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmullh_lane_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmullh_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmullh_laneq_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmullh_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmullh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmullh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulls_lane_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulls_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulls_laneq_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulls_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqdmulls_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqdmulls_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_high_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_high_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovn_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovn_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovnd_s64(a0: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovnd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovnd_u64(a0: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovnd_u64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovnh_s16(a0: i16) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovnh_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovnh_u16(a0: u16) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovnh_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovns_s32(a0: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovns_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovns_u32(a0: u32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovns_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovun_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovun_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovun_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovun_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovun_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovun_high_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovun_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovun_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovun_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovun_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovun_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovun_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovund_s64(a0: i64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovund_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovunh_s16(a0: i16) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovunh_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqmovuns_s32(a0: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqmovuns_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqneg_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqneg_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqneg_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqneg_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqneg_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqneg_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqneg_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqneg_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegb_s8(a0: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegb_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegd_s64(a0: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegd_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegh_s16(a0: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegh_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegq_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegq_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegq_s64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqnegs_s32(a0: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqnegs_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlah_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlah_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlah_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlah_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlah_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlah_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlah_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlah_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlah_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlah_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlah_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlah_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahh_lane_s16(
        a0: i16,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahh_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahh_laneq_s16(
        a0: i16,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahh_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahh_s16(
        a0: i16,
        a1: i16,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahh_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahq_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahq_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahq_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahq_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahs_lane_s32(
        a0: i32,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahs_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlahs_s32(
        a0: i32,
        a1: i32,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlahs_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlsh_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlsh_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlsh_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlsh_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlsh_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlsh_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlsh_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlsh_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlsh_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlsh_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlsh_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlsh_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshh_lane_s16(
        a0: i16,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshh_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshh_laneq_s16(
        a0: i16,
        a1: i16,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshh_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshh_s16(
        a0: i16,
        a1: i16,
        a2: i16,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshh_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshq_lane_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshq_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshq_laneq_s16", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshq_laneq_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshs_lane_s32(
        a0: i32,
        a1: i32,
        a2: crate::Unity::Burst::Intrinsics::v64,
        a3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshs_lane_s32", (a0, a1, a2, a3))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmlshs_s32(
        a0: i32,
        a1: i32,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmlshs_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulh_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulh_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhh_lane_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhh_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhh_laneq_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhh_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_lane_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_lane_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_laneq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_laneq_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_laneq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhs_lane_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhs_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhs_laneq_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhs_laneq_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrdmulhs_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrdmulhs_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlb_s8(a0: i8, a1: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlb_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlb_u8(a0: u8, a1: i8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlb_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshld_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshld_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshld_u64(a0: u64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshld_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlh_u16(a0: u16, a1: i16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlh_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshlq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshlq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshls_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshls_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshls_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshls_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_high_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_high_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_high_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_high_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_high_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_high_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrn_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrn_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrnd_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrnd_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrnd_n_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrnd_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrnh_n_s16(a0: i16, a1: i32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrnh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrnh_n_u16(a0: u16, a1: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrnh_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrns_n_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrns_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrns_n_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrns_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrun_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrun_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrun_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrun_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrun_high_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrun_high_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrun_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrun_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrun_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrun_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrun_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrun_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrund_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrund_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshrunh_n_s16(a0: i16, a1: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshrunh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqrshruns_n_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqrshruns_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlb_n_s8(a0: i8, a1: i32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlb_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlb_n_u8(a0: u8, a1: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlb_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlb_s8(a0: i8, a1: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlb_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlb_u8(a0: u8, a1: i8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlb_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshld_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshld_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshld_n_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshld_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshld_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshld_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshld_u64(a0: u64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshld_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlh_n_s16(a0: i16, a1: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlh_n_u16(a0: u16, a1: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlh_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlh_u16(a0: u16, a1: i16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlh_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshls_n_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshls_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshls_n_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshls_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshls_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshls_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshls_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshls_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlu_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlu_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlu_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlu_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlu_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlu_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlu_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlu_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlub_n_s8(a0: i8, a1: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlub_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlud_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlud_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshluh_n_s16(a0: i16, a1: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshluh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshluq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshluq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshluq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshluq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshluq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshluq_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshluq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshluq_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshlus_n_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshlus_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_high_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_high_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_high_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_high_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_high_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_high_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrn_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrn_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrnd_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrnd_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrnd_n_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrnd_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrnh_n_s16(a0: i16, a1: i32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrnh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrnh_n_u16(a0: u16, a1: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrnh_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrns_n_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrns_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrns_n_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrns_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrun_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrun_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrun_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrun_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrun_high_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrun_high_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrun_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrun_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrun_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrun_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrun_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrun_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrund_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrund_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshrunh_n_s16(a0: i16, a1: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshrunh_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqshruns_n_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqshruns_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsub_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsub_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubb_s8(a0: i8, a1: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubb_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubb_u8(a0: u8, a1: u8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubb_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubh_s16(a0: i16, a1: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubh_u16(a0: u16, a1: u16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubh_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubs_s32(a0: i32, a1: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubs_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqsubs_u32(a0: u32, a1: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqsubs_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbl1_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbl1_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbl1_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbl1_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbl1q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbl1q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbl1q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbl1q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbx1_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbx1_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbx1_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbx1_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbx1q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbx1q_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vqtbx1q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vqtbx1q_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_high_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_high_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vraddhn_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vraddhn_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrbit_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrbit_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrbit_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrbit_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrbitq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrbitq_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrbitq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrbitq_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpe_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpe_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpe_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpe_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpe_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpe_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecped_f64(a0: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecped_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpeq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpeq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpeq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpeq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpeq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpeq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpes_f32(a0: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpes_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecps_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecps_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecps_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecps_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpsd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpsd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpsq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpsq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpsq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpsq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpss_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpss_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpxd_f64(a0: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpxd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrecpxs_f32(a0: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrecpxs_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev16_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev16_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev16_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev16_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev16q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev16q_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev16q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev16q_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32q_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32q_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32q_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev32q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev32q_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_s16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_s32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_s8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_u16", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrev64q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrev64q_u8", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhadd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhadd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhadd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhadd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhadd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhadd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhadd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhadd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhadd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhadd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhadd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhadd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhaddq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhaddq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhaddq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhaddq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhaddq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhaddq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhaddq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhaddq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhaddq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhaddq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrhaddq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrhaddq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrnd_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrnd_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrnd_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrnd_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrnda_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrnda_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrnda_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrnda_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndaq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndaq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndaq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndaq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndi_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndi_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndi_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndi_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndiq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndiq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndiq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndiq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndm_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndm_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndm_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndm_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndmq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndmq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndmq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndmq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndn_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndn_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndn_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndn_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndnq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndnq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndnq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndnq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndns_f32(a0: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndns_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndp_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndp_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndp_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndp_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndpq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndpq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndpq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndpq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndx_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndx_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndx_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndx_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndxq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndxq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrndxq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrndxq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshld_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshld_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshld_u64(a0: u64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshld_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshlq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshlq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshr_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshr_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrd_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrd_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrd_n_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrd_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_high_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_high_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_high_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_high_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_high_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_high_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrn_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrn_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrshrq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrshrq_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrte_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrte_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrte_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrte_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrte_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrte_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrted_f64(a0: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrted_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrteq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrteq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrteq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrteq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrteq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrteq_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrtes_f32(a0: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrtes_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrts_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrts_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrts_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrts_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrtsd_f64(a0: f64, a1: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrtsd_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrtsq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrtsq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrtsq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrtsq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsqrtss_f32(a0: f32, a1: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsqrtss_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsra_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsra_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsrad_n_s64(
        a0: i64,
        a1: i64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsrad_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsrad_n_u64(
        a0: u64,
        a1: u64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsrad_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsraq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsraq_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_high_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_high_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vrsubhn_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vrsubhn_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_f32(
        a0: f32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_f64(
        a0: f64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_s64(
        a0: i64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_s8(
        a0: i8,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_u16(
        a0: u16,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_u32(
        a0: u32,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_u64(
        a0: u64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vset_lane_u8(
        a0: u8,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vset_lane_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_f32(
        a0: f32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_f32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_f64(
        a0: f64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_f64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_s16(
        a0: i16,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_s32(
        a0: i32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_s64(
        a0: i64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_s8(
        a0: i8,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_u16(
        a0: u16,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_u32(
        a0: u32,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_u64(
        a0: u64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsetq_lane_u8(
        a0: u8,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsetq_lane_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha1cq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u32,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha1cq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha1h_u32(a0: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha1h_u32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha1mq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u32,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha1mq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha1pq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: u32,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha1pq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha1su0q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha1su0q_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha1su1q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha1su1q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha256h2q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha256h2q_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha256hq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha256hq_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha256su0q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha256su0q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsha256su1q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsha256su1q_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshld_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshld_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshld_n_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshld_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshld_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshld_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshld_u64(a0: u64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshld_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_high_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_high_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_high_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_high_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_high_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_high_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_high_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_high_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshll_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshll_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshlq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshlq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshr_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshr_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrd_n_s64(a0: i64, a1: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrd_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrd_n_u64(a0: u64, a1: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrd_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_high_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_high_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_high_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_high_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_high_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_high_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_high_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_high_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_high_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_high_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_high_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_high_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrn_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrn_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vshrq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vshrq_n_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsli_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsli_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vslid_n_s64(a0: i64, a1: i64, a2: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vslid_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vslid_n_u64(a0: u64, a1: u64, a2: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vslid_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsliq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsliq_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqadd_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqadd_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqadd_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqadd_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqadd_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqadd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqadd_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqadd_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddb_u8(a0: u8, a1: i8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddb_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddd_u64(a0: u64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddh_u16(a0: u16, a1: i16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddh_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqaddq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqaddq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqadds_u32(a0: u32, a1: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqadds_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqrt_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqrt_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqrt_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqrt_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqrtq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqrtq_f32", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsqrtq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsqrtq_f64", (a0))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsra_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsra_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsrad_n_s64(a0: i64, a1: i64, a2: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsrad_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsrad_n_u64(a0: u64, a1: u64, a2: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsrad_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsraq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsraq_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsri_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsri_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsrid_n_s64(a0: i64, a1: i64, a2: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsrid_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsrid_n_u64(a0: u64, a1: u64, a2: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsrid_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsriq_n_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsriq_n_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_f32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_f64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_s16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_s32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_s64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_s8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_u16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_u32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_u64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1_u8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_f32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_f64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_s16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_s32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_s64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_s8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_u16(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_u32(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_u64(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vst1q_u8(
        a0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vst1q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_f64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsub_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsub_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_high_s16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_high_s32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_high_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_high_s64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_high_u16", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_high_u32", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_high_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v128,
        a2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_high_u64", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubhn_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubhn_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_high_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_high_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubl_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubl_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_high_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_high_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_high_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_high_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_high_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_high_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_high_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_high_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_high_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_high_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_high_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_high_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vsubw_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vsubw_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtbl1_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtbl1_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtbl1_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtbl1_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtbx1_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtbx1_s8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtbx1_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
        a2: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtbx1_u8", (a0, a1, a2))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn1q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn1q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtrn2q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtrn2q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_u64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtst_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtst_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstd_s64(a0: i64, a1: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstd_u64(a0: u64, a1: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstd_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vtstq_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vtstq_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqadd_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqadd_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqadd_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqadd_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqadd_s64(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqadd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqadd_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqadd_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddb_s8(a0: i8, a1: u8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddb_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddd_s64(a0: i64, a1: u64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddd_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddh_s16(a0: i16, a1: u16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddh_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddq_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddq_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddq_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddq_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddq_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddq_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqaddq_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqaddq_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuqadds_s32(a0: i32, a1: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuqadds_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp1q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp1q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vuzp2q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vuzp2q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip1q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip1q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_f32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_s16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_s32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_s8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_u16(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_u32(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2_u8(
        a0: crate::Unity::Burst::Intrinsics::v64,
        a1: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v64> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_f32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_f32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_f64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_f64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_s16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_s16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_s32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_s32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_s64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_s64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_s8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_s8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_u16(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_u16", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_u32(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_u32", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_u64(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_u64", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
    pub fn vzip2q_u8(
        a0: crate::Unity::Burst::Intrinsics::v128,
        a1: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("vzip2q_u8", (a0, a1))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+Arm+Neon")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::Arm_Neon {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
