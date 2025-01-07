#[cfg(feature = "System+Buffers+Utilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Utilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Buffers+Utilities")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Buffers::Utilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Buffers";
    const CLASS_NAME: &'static str = "Utilities";
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
#[cfg(feature = "System+Buffers+Utilities")]
impl std::ops::Deref for crate::System::Buffers::Utilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Utilities")]
impl std::ops::DerefMut for crate::System::Buffers::Utilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Utilities")]
impl crate::System::Buffers::Utilities {
    pub fn GetMaxSizeForBucket(binIndex: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaxSizeForBucket", (binIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectBucketIndex(bufferSize: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SelectBucketIndex", (bufferSize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+Utilities")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Buffers::Utilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
