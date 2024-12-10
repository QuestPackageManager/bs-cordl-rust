#[cfg(feature = "Unity+Burst+CompilerServices+SkipLocalsInitAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SkipLocalsInitAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Burst+CompilerServices+SkipLocalsInitAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::CompilerServices::SkipLocalsInitAttribute =>
    "Unity.Burst.CompilerServices"."SkipLocalsInitAttribute"
);
#[cfg(feature = "Unity+Burst+CompilerServices+SkipLocalsInitAttribute")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::SkipLocalsInitAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+SkipLocalsInitAttribute")]
impl std::ops::DerefMut
for crate::Unity::Burst::CompilerServices::SkipLocalsInitAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+SkipLocalsInitAttribute")]
impl crate::Unity::Burst::CompilerServices::SkipLocalsInitAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "Unity+Burst+CompilerServices+SkipLocalsInitAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::CompilerServices::SkipLocalsInitAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
