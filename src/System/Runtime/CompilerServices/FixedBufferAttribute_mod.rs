#[cfg(feature = "System+Runtime+CompilerServices+FixedBufferAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct FixedBufferAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ElementType_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _Length_k__BackingField: i32,
}
#[cfg(feature = "System+Runtime+CompilerServices+FixedBufferAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::FixedBufferAttribute =>
    "System.Runtime.CompilerServices"."FixedBufferAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+FixedBufferAttribute")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::FixedBufferAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+FixedBufferAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::FixedBufferAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+FixedBufferAttribute")]
impl crate::System::Runtime::CompilerServices::FixedBufferAttribute {
    pub fn New(
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (elementType, length))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        elementType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (elementType, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ElementType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ElementType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+FixedBufferAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::FixedBufferAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
