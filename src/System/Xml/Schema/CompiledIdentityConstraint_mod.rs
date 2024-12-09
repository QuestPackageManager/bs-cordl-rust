#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
#[repr(C)]
#[derive(Debug)]
pub struct CompiledIdentityConstraint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: *mut crate::System::Xml::XmlQualifiedName,
    pub role: crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole,
    pub selector: *mut crate::System::Xml::Schema::Asttree,
    pub fields: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Schema::Asttree,
    >,
    pub refer: *mut crate::System::Xml::XmlQualifiedName,
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::CompiledIdentityConstraint
    => "System.Xml.Schema"."CompiledIdentityConstraint"
);
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
impl std::ops::Deref for crate::System::Xml::Schema::CompiledIdentityConstraint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
impl std::ops::DerefMut for crate::System::Xml::Schema::CompiledIdentityConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
impl crate::System::Xml::Schema::CompiledIdentityConstraint {
    #[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
    pub type ConstraintRole = crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole;
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_XmlSchemaIdentityConstraint_XmlNamespaceManager1(
        constraint: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constraint, nsmgr))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_XmlSchemaIdentityConstraint_XmlNamespaceManager1(
        &mut self,
        constraint: *mut crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        nsmgr: *mut crate::System::Xml::XmlNamespaceManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (constraint, nsmgr))?;
        Ok(__cordl_ret)
    }
    pub fn get_Fields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::Schema::Asttree>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::Asttree,
        > = __cordl_object.invoke("get_Fields", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Role(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole = __cordl_object
            .invoke("get_Role", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Selector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Xml::Schema::Asttree> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::Asttree = __cordl_object
            .invoke("get_Selector", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::CompiledIdentityConstraint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompiledIdentityConstraint_ConstraintRole {
    Key = 1i32,
    Keyref = 2i32,
    Unique = 0i32,
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole =>
    "System.Xml.Schema"."CompiledIdentityConstraint/ConstraintRole"
);
