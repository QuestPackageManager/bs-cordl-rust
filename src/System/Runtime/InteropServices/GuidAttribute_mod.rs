#[cfg(feature = "System+Runtime+InteropServices+GuidAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct GuidAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _val: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Runtime+InteropServices+GuidAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::InteropServices::GuidAttribute
    => "System.Runtime.InteropServices"."GuidAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+GuidAttribute")]
impl std::ops::Deref for crate::System::Runtime::InteropServices::GuidAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+GuidAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::InteropServices::GuidAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+GuidAttribute")]
impl crate::System::Runtime::InteropServices::GuidAttribute {
    pub fn New(
        guid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (guid))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        guid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (guid))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+GuidAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::GuidAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
