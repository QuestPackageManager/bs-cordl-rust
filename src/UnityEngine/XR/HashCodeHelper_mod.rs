#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct HashCodeHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::HashCodeHelper =>
    "UnityEngine.XR"."HashCodeHelper"
);
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl std::ops::Deref for crate::UnityEngine::XR::HashCodeHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl std::ops::DerefMut for crate::UnityEngine::XR::HashCodeHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl crate::UnityEngine::XR::HashCodeHelper {
    pub fn Combine_i32_1(
        hash1: i32,
        hash2: i32,
        hash3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (hash1, hash2, hash3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_i32_i32_0(
        hash1: i32,
        hash2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (hash1, hash2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_i32_i32_2(
        hash1: i32,
        hash2: i32,
        hash3: i32,
        hash4: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (hash1, hash2, hash3, hash4))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_i32_i32_i32_3(
        hash1: i32,
        hash2: i32,
        hash3: i32,
        hash4: i32,
        hash5: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (hash1, hash2, hash3, hash4, hash5))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_i32_i32_i32_i32_4(
        hash1: i32,
        hash2: i32,
        hash3: i32,
        hash4: i32,
        hash5: i32,
        hash6: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (hash1, hash2, hash3, hash4, hash5, hash6))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_i32_i32_i32_i32_i32_5(
        hash1: i32,
        hash2: i32,
        hash3: i32,
        hash4: i32,
        hash5: i32,
        hash6: i32,
        hash7: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (hash1, hash2, hash3, hash4, hash5, hash6, hash7))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_i32_i32_i32_i32_i32_i32_6(
        hash1: i32,
        hash2: i32,
        hash3: i32,
        hash4: i32,
        hash5: i32,
        hash6: i32,
        hash7: i32,
        hash8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Combine",
                (hash1, hash2, hash3, hash4, hash5, hash6, hash7, hash8),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+HashCodeHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::HashCodeHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
