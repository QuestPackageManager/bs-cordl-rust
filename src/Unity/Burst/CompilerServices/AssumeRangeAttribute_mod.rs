#[cfg(feature = "Unity+Burst+CompilerServices+AssumeRangeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssumeRangeAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Burst+CompilerServices+AssumeRangeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::CompilerServices::AssumeRangeAttribute =>
    "Unity.Burst.CompilerServices"."AssumeRangeAttribute"
);
#[cfg(feature = "Unity+Burst+CompilerServices+AssumeRangeAttribute")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::AssumeRangeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+AssumeRangeAttribute")]
impl std::ops::DerefMut for crate::Unity::Burst::CompilerServices::AssumeRangeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+AssumeRangeAttribute")]
impl crate::Unity::Burst::CompilerServices::AssumeRangeAttribute {
    pub fn New_i64_i64_0(
        min: i64,
        max: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (min, max))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u64_u64_1(
        min: u64,
        max: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (min, max))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_i64_i64_0(
        &mut self,
        min: i64,
        max: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_u64_1(
        &mut self,
        min: u64,
        max: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (min, max))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+AssumeRangeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::CompilerServices::AssumeRangeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
