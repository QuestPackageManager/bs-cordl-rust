#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
#[repr(C)]
#[derive(Debug)]
pub struct CompiledIdentityConstraint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    pub role: crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole,
    pub selector: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
    pub fields: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
        >,
    >,
    pub refer: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::CompiledIdentityConstraint {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "CompiledIdentityConstraint";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlSchemaIdentityConstraint_XmlNamespaceManager1(
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constraint, nsmgr))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::CompiledIdentityConstraint as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::CompiledIdentityConstraint as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSchemaIdentityConstraint_XmlNamespaceManager1(
        &mut self,
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
        nsmgr: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::CompiledIdentityConstraint as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNamespaceManager>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::CompiledIdentityConstraint as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (constraint, nsmgr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Fields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::CompiledIdentityConstraint as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
                    >,
                >,
                0usize,
            >("get_Fields")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::CompiledIdentityConstraint as
                    quest_hook::libil2cpp::Type > ::class(), "get_Fields", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Role(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::CompiledIdentityConstraint as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole,
                0usize,
            >("get_Role")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::CompiledIdentityConstraint as
                    quest_hook::libil2cpp::Type > ::class(), "get_Role", 0usize
                )
            });
        let __cordl_ret: crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Selector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::CompiledIdentityConstraint as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
                0usize,
            >("get_Selector")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::CompiledIdentityConstraint as
                    quest_hook::libil2cpp::Type > ::class(), "get_Selector", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::Asttree,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompiledIdentityConstraint_ConstraintRole {
    #[default]
    Key = 1i32,
    Keyref = 2i32,
    Unique = 0i32,
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "CompiledIdentityConstraint/ConstraintRole";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Xml+Schema+CompiledIdentityConstraint+ConstraintRole")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::CompiledIdentityConstraint_ConstraintRole {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
