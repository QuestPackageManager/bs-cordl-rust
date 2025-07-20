#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub isDefault: bool,
    pub isNil: bool,
    pub schemaElement: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaElement,
    >,
    pub schemaAttribute: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaAttribute,
    >,
    pub schemaType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    pub memberType: quest_hook::libil2cpp::Gc<
        crate::System::Xml::Schema::XmlSchemaSimpleType,
    >,
    pub validity: crate::System::Xml::Schema::XmlSchemaValidity,
    pub contentType: crate::System::Xml::Schema::XmlSchemaContentType,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::XmlSchemaInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XmlSchemaInfo";
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
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
impl crate::System::Xml::Schema::XmlSchemaInfo {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_XmlSchemaValidity1(
        validity: crate::System::Xml::Schema::XmlSchemaValidity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (validity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XmlSchemaValidity1(
        &mut self,
        validity: crate::System::Xml::Schema::XmlSchemaValidity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Xml::Schema::XmlSchemaValidity),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (validity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XmlSchemaContentType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::Schema::XmlSchemaContentType,
                0usize,
            >("get_ContentType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_ContentType", 0usize
                )
            });
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaContentType = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_HasDefaultValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_HasDefaultValue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_HasDefaultValue",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDefault(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsDefault", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNil(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsNil")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsNil", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUnionType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsUnionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_IsUnionType", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XmlSchemaSimpleType,
                >,
                0usize,
            >("get_MemberType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_MemberType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaSimpleType,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemaAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XmlSchemaAttribute,
                >,
                0usize,
            >("get_SchemaAttribute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_SchemaAttribute",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttribute,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemaElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
                0usize,
            >("get_SchemaElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_SchemaElement", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaElement,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemaType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
                0usize,
            >("get_SchemaType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_SchemaType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Validity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::Schema::XmlSchemaValidity> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Xml::Schema::XmlSchemaValidity,
                0usize,
            >("get_Validity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_Validity", 0usize
                )
            });
        let __cordl_ret: crate::System::Xml::Schema::XmlSchemaValidity = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
                0usize,
            >("get_XmlType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "get_XmlType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaType,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_IsDefault(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_IsDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_IsDefault", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_IsNil(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_IsNil")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_IsNil", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MemberType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XmlSchemaSimpleType,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MemberType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_MemberType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_SchemaAttribute(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XmlSchemaAttribute,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_SchemaAttribute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_SchemaAttribute",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_SchemaElement(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Xml::Schema::XmlSchemaElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_SchemaElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_SchemaElement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_SchemaType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaType>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_SchemaType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_SchemaType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Validity(
        &mut self,
        value: crate::System::Xml::Schema::XmlSchemaValidity,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::Schema::XmlSchemaInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Xml::Schema::XmlSchemaValidity),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Validity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::Schema::XmlSchemaInfo as
                    quest_hook::libil2cpp::Type > ::class(), "set_Validity", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XmlSchemaInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
impl AsRef<crate::System::Xml::Schema::IXmlSchemaInfo>
for crate::System::Xml::Schema::XmlSchemaInfo {
    fn as_ref(&self) -> &crate::System::Xml::Schema::IXmlSchemaInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaInfo")]
impl AsMut<crate::System::Xml::Schema::IXmlSchemaInfo>
for crate::System::Xml::Schema::XmlSchemaInfo {
    fn as_mut(&mut self) -> &mut crate::System::Xml::Schema::IXmlSchemaInfo {
        unsafe { std::mem::transmute(self) }
    }
}
