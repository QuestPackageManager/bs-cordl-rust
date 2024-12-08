#[cfg(feature = "Unity+Burst+CompilerServices+IgnoreWarningAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoreWarningAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Burst+CompilerServices+IgnoreWarningAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::CompilerServices::IgnoreWarningAttribute =>
    "Unity.Burst.CompilerServices"."IgnoreWarningAttribute"
);
#[cfg(feature = "Unity+Burst+CompilerServices+IgnoreWarningAttribute")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::IgnoreWarningAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+IgnoreWarningAttribute")]
impl std::ops::DerefMut
for crate::Unity::Burst::CompilerServices::IgnoreWarningAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+IgnoreWarningAttribute")]
impl crate::Unity::Burst::CompilerServices::IgnoreWarningAttribute {
    pub fn _ctor(
        &mut self,
        warning: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (warning))?;
        Ok(__cordl_ret)
    }
    pub fn New(warning: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (warning))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+IgnoreWarningAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::CompilerServices::IgnoreWarningAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
