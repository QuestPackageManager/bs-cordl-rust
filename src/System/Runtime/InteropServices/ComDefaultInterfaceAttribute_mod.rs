#[cfg(feature = "System+Runtime+InteropServices+ComDefaultInterfaceAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ComDefaultInterfaceAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _val: *mut crate::System::Type,
}
#[cfg(feature = "System+Runtime+InteropServices+ComDefaultInterfaceAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::ComDefaultInterfaceAttribute =>
    "System.Runtime.InteropServices"."ComDefaultInterfaceAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+ComDefaultInterfaceAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::ComDefaultInterfaceAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComDefaultInterfaceAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::ComDefaultInterfaceAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComDefaultInterfaceAttribute")]
impl crate::System::Runtime::InteropServices::ComDefaultInterfaceAttribute {
    pub fn New(
        defaultInterface: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (defaultInterface))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        defaultInterface: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (defaultInterface))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+InteropServices+ComDefaultInterfaceAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::ComDefaultInterfaceAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
