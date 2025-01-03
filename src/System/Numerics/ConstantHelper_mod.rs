#[cfg(feature = "System+Numerics+ConstantHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstantHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+ConstantHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::ConstantHelper =>
    "System.Numerics"."ConstantHelper"
);
#[cfg(feature = "System+Numerics+ConstantHelper")]
impl std::ops::Deref for crate::System::Numerics::ConstantHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+ConstantHelper")]
impl std::ops::DerefMut for crate::System::Numerics::ConstantHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+ConstantHelper")]
impl crate::System::Numerics::ConstantHelper {
    pub fn GetByteWithAllBitsSet() -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetByteWithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDoubleWithAllBitsSet() -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDoubleWithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInt16WithAllBitsSet() -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInt16WithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInt32WithAllBitsSet() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInt32WithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInt64WithAllBitsSet() -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInt64WithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSByteWithAllBitsSet() -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSByteWithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSingleWithAllBitsSet() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSingleWithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUInt16WithAllBitsSet() -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUInt16WithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUInt32WithAllBitsSet() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUInt32WithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUInt64WithAllBitsSet() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUInt64WithAllBitsSet", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+ConstantHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::ConstantHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
