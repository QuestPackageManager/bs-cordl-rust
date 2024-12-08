#[cfg(feature = "System+Reflection+AssemblyCopyrightAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyCopyrightAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Copyright_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "System+Reflection+AssemblyCopyrightAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::AssemblyCopyrightAttribute
    => "System.Reflection"."AssemblyCopyrightAttribute"
);
#[cfg(feature = "System+Reflection+AssemblyCopyrightAttribute")]
impl std::ops::Deref for crate::System::Reflection::AssemblyCopyrightAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyCopyrightAttribute")]
impl std::ops::DerefMut for crate::System::Reflection::AssemblyCopyrightAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+AssemblyCopyrightAttribute")]
impl crate::System::Reflection::AssemblyCopyrightAttribute {
    pub fn _ctor(
        &mut self,
        copyright: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (copyright))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        copyright: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (copyright))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Reflection+AssemblyCopyrightAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Reflection::AssemblyCopyrightAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
