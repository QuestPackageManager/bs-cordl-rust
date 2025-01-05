#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HashUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::HashUtility =>
    "UnityEngine.Timeline"."HashUtility"
);
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl std::ops::Deref for crate::UnityEngine::Timeline::HashUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::HashUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl crate::UnityEngine::Timeline::HashUtility {
    pub fn CombineHash_Il2CppArray6(
        hashes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (hashes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_0(
        h1: i32,
        h2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (h1, h2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_1(
        h1: i32,
        h2: i32,
        h3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (h1, h2, h3))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_2(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (h1, h2, h3, h4))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_i32_3(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (h1, h2, h3, h4, h5))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_i32_i32_4(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (h1, h2, h3, h4, h5, h6))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHash_i32_i32_i32_i32_i32_i32_i32_5(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
        h5: i32,
        h6: i32,
        h7: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHash", (h1, h2, h3, h4, h5, h6, h7))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::HashUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
