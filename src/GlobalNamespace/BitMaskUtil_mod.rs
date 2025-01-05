#[cfg(feature = "BitMaskUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BitMaskUtil {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BitMaskUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BitMaskUtil => ""."BitMaskUtil"
);
#[cfg(feature = "BitMaskUtil")]
impl std::ops::Deref for crate::GlobalNamespace::BitMaskUtil {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BitMaskUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::BitMaskUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BitMaskUtil")]
impl crate::GlobalNamespace::BitMaskUtil {
    pub fn NumberOfSetBits_u32_1(i: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberOfSetBits", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberOfSetBits_u64_0(i: u64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberOfSetBits", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftLeft(
        value: quest_hook::libil2cpp::ByRefMut<u64>,
        shift: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftLeft", (value, shift))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShiftRight(
        value: quest_hook::libil2cpp::ByRefMut<u64>,
        shift: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShiftRight", (value, shift))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BitMaskUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BitMaskUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
