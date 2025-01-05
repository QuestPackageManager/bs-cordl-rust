#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
#[repr(C)]
#[derive(Debug)]
pub struct VectorHash {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::VectorHash =>
    "UnityEngine.ProBuilder"."VectorHash"
);
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::VectorHash {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::VectorHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl crate::UnityEngine::ProBuilder::VectorHash {
    pub const FltCompareResolution: f32 = 1000f32;
    pub fn GetHashCode_Vector2_0(
        v: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Vector3_1(
        v: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode_Vector4_2(
        v: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashFloat(f: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashFloat", (f))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VectorHash")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::VectorHash {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
