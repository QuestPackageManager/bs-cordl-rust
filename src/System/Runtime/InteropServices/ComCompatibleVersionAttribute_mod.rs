#[cfg(feature = "System+Runtime+InteropServices+ComCompatibleVersionAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ComCompatibleVersionAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _major: i32,
    pub _minor: i32,
    pub _build: i32,
    pub _revision: i32,
}
#[cfg(feature = "System+Runtime+InteropServices+ComCompatibleVersionAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::ComCompatibleVersionAttribute =>
    "System.Runtime.InteropServices"."ComCompatibleVersionAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+ComCompatibleVersionAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::ComCompatibleVersionAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComCompatibleVersionAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::ComCompatibleVersionAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComCompatibleVersionAttribute")]
impl crate::System::Runtime::InteropServices::ComCompatibleVersionAttribute {
    pub fn _ctor(
        &mut self,
        major: i32,
        minor: i32,
        build: i32,
        revision: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (major, minor, build, revision))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        major: i32,
        minor: i32,
        build: i32,
        revision: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (major, minor, build, revision))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComCompatibleVersionAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::ComCompatibleVersionAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
