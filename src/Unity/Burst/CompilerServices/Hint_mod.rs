#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
#[repr(C)]
#[derive(Debug)]
pub struct Hint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::CompilerServices::Hint =>
    "Unity.Burst.CompilerServices"."Hint"
);
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::Hint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl std::ops::DerefMut for crate::Unity::Burst::CompilerServices::Hint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl crate::Unity::Burst::CompilerServices::Hint {
    pub fn Assume(
        condition: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Assume", (condition))?;
        Ok(__cordl_ret.into())
    }
    pub fn Likely(condition: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Likely", (condition))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unlikely(condition: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unlikely", (condition))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Hint")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::CompilerServices::Hint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
