#[cfg(feature = "System+Xml+Schema+KSStruct")]
#[repr(C)]
#[derive(Debug)]
pub struct KSStruct {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub depth: i32,
    pub ks: *mut crate::System::Xml::Schema::KeySequence,
    pub fields: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Schema::LocatedActiveAxis,
    >,
}
#[cfg(feature = "System+Xml+Schema+KSStruct")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::KSStruct =>
    "System.Xml.Schema"."KSStruct"
);
#[cfg(feature = "System+Xml+Schema+KSStruct")]
impl std::ops::Deref for crate::System::Xml::Schema::KSStruct {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+KSStruct")]
impl std::ops::DerefMut for crate::System::Xml::Schema::KSStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+KSStruct")]
impl crate::System::Xml::Schema::KSStruct {
    pub fn New(
        ks: *mut crate::System::Xml::Schema::KeySequence,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ks, dim))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        ks: *mut crate::System::Xml::Schema::KeySequence,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ks, dim))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+KSStruct")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::KSStruct {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
