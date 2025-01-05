#[cfg(feature = "System+Marvin")]
#[repr(C)]
#[derive(Debug)]
pub struct Marvin {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Marvin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Marvin => "System"."Marvin"
);
#[cfg(feature = "System+Marvin")]
impl std::ops::Deref for crate::System::Marvin {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Marvin")]
impl std::ops::DerefMut for crate::System::Marvin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Marvin")]
impl crate::System::Marvin {
    pub fn Block(
        rp0: quest_hook::libil2cpp::ByRefMut<u32>,
        rp1: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Block", (rp0, rp1))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeHash32_ByRefMut_i32_u64_1(
        data: quest_hook::libil2cpp::ByRefMut<u8>,
        count: i32,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeHash32", (data, count, seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeHash32_ReadOnlySpan_1_u64_0(
        data: crate::System::ReadOnlySpan_1<u8>,
        seed: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeHash32", (data, seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSeed() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateSeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _rotl(value: u32, shift: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_rotl", (value, shift))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultSeed() -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DefaultSeed", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Marvin")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Marvin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
