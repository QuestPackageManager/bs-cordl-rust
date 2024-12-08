#[cfg(feature = "System+Data+ConstraintTable")]
#[repr(C)]
#[derive(Debug)]
pub struct ConstraintTable {
    __cordl_parent: crate::System::Object,
    pub table: *mut crate::System::Data::DataTable,
    pub constraint: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
}
#[cfg(feature = "System+Data+ConstraintTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::ConstraintTable => "System.Data"
    ."ConstraintTable"
);
#[cfg(feature = "System+Data+ConstraintTable")]
impl std::ops::Deref for crate::System::Data::ConstraintTable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ConstraintTable")]
impl std::ops::DerefMut for crate::System::Data::ConstraintTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+ConstraintTable")]
impl crate::System::Data::ConstraintTable {
    pub fn _ctor(
        &mut self,
        t: *mut crate::System::Data::DataTable,
        c: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (t, c))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        t: *mut crate::System::Data::DataTable,
        c: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (t, c))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Data+ConstraintTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::ConstraintTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
