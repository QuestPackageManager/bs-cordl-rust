#[cfg(feature = "System+Runtime+CompilerServices+TypeForwardedFromAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeForwardedFromAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _AssemblyFullName_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeForwardedFromAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::TypeForwardedFromAttribute =>
    "System.Runtime.CompilerServices"."TypeForwardedFromAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+TypeForwardedFromAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::TypeForwardedFromAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeForwardedFromAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::TypeForwardedFromAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeForwardedFromAttribute")]
impl crate::System::Runtime::CompilerServices::TypeForwardedFromAttribute {
    pub fn get_AssemblyFullName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssemblyFullName", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        assemblyFullName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assemblyFullName))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        assemblyFullName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assemblyFullName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TypeForwardedFromAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::TypeForwardedFromAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
