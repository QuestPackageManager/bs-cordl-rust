#[cfg(feature = "cordl_class_System+Xml+Schema+Preprocessor")]
#[repr(C)]
#[derive(Debug)]
pub struct Preprocessor {
    __cordl_parent: crate::System::Xml::Schema::BaseProcessor,
    pub Xmlns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub NsXsi: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub rootSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub currentSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub elementFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub attributeFormDefault: crate::System::Xml::Schema::XmlSchemaForm,
    pub blockDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub finalDefault: crate::System::Xml::Schema::XmlSchemaDerivationMethod,
    pub schemaLocations: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub chameleonSchemas: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub referenceNamespaces: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub processedExternals: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub lockList: quest_hook::libil2cpp::Gc<crate::System::Collections::SortedList>,
    pub readerSettings: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
    pub rootSchemaForRedefine: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    pub redefinedList: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    pub xmlResolver: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
}
#[cfg(feature = "cordl_class_System+Xml+Schema+Preprocessor")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::Preprocessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Preprocessor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl std::ops::Deref for crate::System::Xml::Schema::Preprocessor {
    type Target = crate::System::Xml::Schema::BaseProcessor;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl std::ops::DerefMut for crate::System::Xml::Schema::Preprocessor {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+Preprocessor")]
impl crate::System::Xml::Schema::Preprocessor {
    pub fn BuildRefNamespaces(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchema,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BuildRefNamespaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildRefNamespaces", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildSchemaList(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchema,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BuildSchemaList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildSchemaList", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRefinedAttributeGroup(
        &mut self,
        attributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckRefinedAttributeGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckRefinedAttributeGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (attributeGroup))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRefinedComplexType(
        &mut self,
        ctype: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaComplexType,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckRefinedComplexType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckRefinedComplexType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (ctype))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRefinedGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaGroup,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckRefinedGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckRefinedGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (group))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRefinedSimpleType(
        &mut self,
        stype: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaSimpleType,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckRefinedSimpleType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckRefinedSimpleType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (stype))? };
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchema,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Cleanup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn CleanupRedefine(
        &mut self,
        include: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaExternal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaExternal,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CleanupRedefine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CleanupRedefine", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (include))? };
        Ok(__cordl_ret.into())
    }
    pub fn CopyIncludedComponents(
        &mut self,
        includedSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "CopyIncludedComponents"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CopyIncludedComponents",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (includedSchema, schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn CountGroupSelfReference(
        &mut self,
        items: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObjectCollection>,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
        redefined: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaObjectCollection,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
                    ), i32, 3usize>("CountGroupSelfReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CountGroupSelfReference",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (items, name, redefined))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadExternals: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                    ), bool, 3usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (schema, targetNamespace, loadExternals))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBuildInSchema() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        0usize,
                    >("GetBuildInSchema")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBuildInSchema", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetChameleonSchema(
        &mut self,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>, 2usize>(
                        "GetChameleonSchema",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetChameleonSchema",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema> =
            unsafe { cordl_method_info.invoke_unchecked(self, (targetNamespace, schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIncludedSet(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        includesList: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
                    ), quest_hook::libil2cpp::Void, 2usize>("GetIncludedSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetIncludedSet",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schema, includesList))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetParentSchema(
        currentSchemaObject: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaObject,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        1usize,
                    >("GetParentSchema")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetParentSchema", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema> =
            unsafe { cordl_method_info.invoke_unchecked((), (currentSchemaObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSchemaEntity(
        &mut self,
        ruri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetSchemaEntity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetSchemaEntity", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, (ruri))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadExternals(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchema,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("LoadExternals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadExternals", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventHandler: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationEventHandler>,
        compilationSettings: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (nameTable, schemaNames, eventHandler, compilationSettings),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseUri(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        code: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sourceSchemaObject: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>("ParseUri")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ParseUri",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (uri, code, sourceSchemaObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn Preprocess(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        targetNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        imports: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
                    ), quest_hook::libil2cpp::Void, 3usize>("Preprocess")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Preprocess",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (schema, targetNamespace, imports))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessAnnotation_XmlSchemaAnnotation1(
        &mut self,
        annotation: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnnotation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAnnotation,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessAnnotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessAnnotation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (annotation))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessAnnotation_XmlSchemaObject0(
        &mut self,
        schemaObject: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaObject,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessAnnotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessAnnotation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schemaObject))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessAttribute(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAttribute,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (attribute))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessAttributeContent(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAttribute,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessAttributeContent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessAttributeContent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (attribute))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessAttributeGroup(
        &mut self,
        attributeGroup: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAttributeGroup,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessAttributeGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessAttributeGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (attributeGroup))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessAttributes(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaObjectCollection,
        >,
        anyAttribute: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAnyAttribute>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaObjectCollection,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAnyAttribute,
                        >,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "PreprocessAttributes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PreprocessAttributes",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (attributes, anyAttribute, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessComplexType(
        &mut self,
        complexType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
        local: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaComplexType>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "PreprocessComplexType"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PreprocessComplexType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (complexType, local))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessElement(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessElement", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (element))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessElementContent(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessElementContent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessElementContent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (element))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessGroup(
        &mut self,
        group: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaGroup>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaGroup,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessGroup", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (group))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessIdentityConstraint(
        &mut self,
        constraint: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Xml::Schema::XmlSchemaIdentityConstraint,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "PreprocessIdentityConstraint"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PreprocessIdentityConstraint",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (constraint))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessLocalAttribute(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaAttribute>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaAttribute,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessLocalAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessLocalAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (attribute))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessLocalElement(
        &mut self,
        element: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaElement,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessLocalElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessLocalElement", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (element))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessNotation(
        &mut self,
        notation: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaNotation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaNotation,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessNotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessNotation", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (notation))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessParticle(
        &mut self,
        particle: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaParticle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaParticle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessParticle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessParticle", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (particle))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessRedefine(
        &mut self,
        redefineEntry: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::RedefineEntry>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::RedefineEntry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PreprocessRedefine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PreprocessRedefine", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (redefineEntry))? };
        Ok(__cordl_ret.into())
    }
    pub fn PreprocessSimpleType(
        &mut self,
        simpleType: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
        local: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaSimpleType>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "PreprocessSimpleType"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PreprocessSimpleType",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (simpleType, local))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResolveSchemaLocationUri(
        &mut self,
        enclosingSchema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
        location: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Gc<crate::System::Uri>, 2usize>(
                        "ResolveSchemaLocationUri",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResolveSchemaLocationUri",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> =
            unsafe { cordl_method_info.invoke_unchecked(self, (enclosingSchema, location))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetParent(
        &mut self,
        child: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetParent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetParent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (child, parent))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetSchemaDefaults(
        &mut self,
        schema: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchema,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetSchemaDefaults")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetSchemaDefaults", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (schema))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateIdAttribute(
        &mut self,
        xso: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaObject,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateIdAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateIdAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xso))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateNameAttribute(
        &mut self,
        xso: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaObject,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ValidateNameAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateNameAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xso))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateQNameAttribute(
        &mut self,
        xso: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
        attributeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchemaObject>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlQualifiedName>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ValidateQNameAttribute"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateQNameAttribute",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (xso, attributeName, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nameTable: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
        schemaNames: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
        eventHandler: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ValidationEventHandler>,
        compilationSettings: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNameTable>,
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::SchemaNames>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::ValidationEventHandler,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Xml::Schema::XmlSchemaCompilationSettings,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (nameTable, schemaNames, eventHandler, compilationSettings),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RootSchema(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema>,
                        0usize,
                    >("get_RootSchema")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_RootSchema", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::XmlSchema> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ChameleonSchemas(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Hashtable,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ChameleonSchemas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ChameleonSchemas", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ReaderSettings(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReaderSettings>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Xml::XmlReaderSettings,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ReaderSettings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ReaderSettings", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_SchemaLocations(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Hashtable,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_SchemaLocations")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_SchemaLocations", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_XmlResolver(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Xml::XmlResolver>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_XmlResolver")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_XmlResolver", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Xml+Schema+Preprocessor")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::Preprocessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
