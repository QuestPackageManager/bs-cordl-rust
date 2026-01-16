#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterInterpreter")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlFormatWriterInterpreter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub classContract: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ClassDataContract,
    >,
    pub collectionContract: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::CollectionDataContract,
    >,
    pub writer: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlWriterDelegator,
    >,
    pub obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub ctx: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
    >,
    pub dataContract: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::DataContract,
    >,
    pub objLocal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub contractNamespaces: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        >,
    >,
    pub memberNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        >,
    >,
    pub childElementNamespaces: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        >,
    >,
    pub typeIndex: i32,
    pub childElementIndex: i32,
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterInterpreter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::XmlFormatWriterInterpreter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization";
    const CLASS_NAME: &'static str = "XmlFormatWriterInterpreter";
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
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterInterpreter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::XmlFormatWriterInterpreter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterInterpreter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::XmlFormatWriterInterpreter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+XmlFormatWriterInterpreter")]
impl crate::System::Runtime::Serialization::XmlFormatWriterInterpreter {
    pub fn CheckIfConflictingMembersHaveDifferentTypes(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataMember,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataMember,
                        >),
                        bool,
                        1usize,
                    >("CheckIfConflictingMembersHaveDifferentTypes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckIfConflictingMembersHaveDifferentTypes", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (member))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfMemberHasConflict(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataMember,
        >,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
        derivedMostClassContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::DataMember,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                        ),
                        bool,
                        3usize,
                    >("CheckIfMemberHasConflict")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckIfMemberHasConflict", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (member, classContract, derivedMostClassContract),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultValue(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("GetDefaultValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDefaultValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitArgs(
        &mut self,
        objType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InitArgs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitArgs", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (objType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnSerialized(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
        objSerialized: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InvokeOnSerialized")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeOnSerialized", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (classContract, objSerialized, ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeOnSerializing(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
        objSerialized: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ctx: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InvokeOnSerializing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeOnSerializing", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (classContract, objSerialized, ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsDefaultValue(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        bool,
                        2usize,
                    >("IsDefaultValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsDefaultValue", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadMemberValue(
        &mut self,
        member: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataMember,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::DataMember,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("LoadMemberValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadMemberValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (member))? };
        Ok(__cordl_ret.into())
    }
    pub fn NeedsPrefix(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        bool,
                        2usize,
                    >("NeedsPrefix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NeedsPrefix", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type, ns))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_ClassDataContract0(
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (classContract))?;
        Ok(__cordl_object.into())
    }
    pub fn New_CollectionDataContract1(
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (collectionContract))?;
        Ok(__cordl_object.into())
    }
    pub fn TryWritePrimitive(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        arrayItemIndex: crate::System::Nullable_1<i32>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        name: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        nameIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Reflection::MemberInfo,
                            >,
                            crate::System::Nullable_1<i32>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            i32,
                        ),
                        bool,
                        7usize,
                    >("TryWritePrimitive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryWritePrimitive", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_type, value, memberInfo, arrayItemIndex, ns, name, nameIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryWritePrimitiveArray(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        itemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        itemName: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        itemNamespace: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("TryWritePrimitiveArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryWritePrimitiveArray", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_type, itemType, value, itemName, itemNamespace),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnwrapNullableObject(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        memberType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
        isNull: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        3usize,
                    >("UnwrapNullableObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnwrapNullableObject", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (memberValue, memberType, isNull))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteClass(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ClassDataContract,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteClass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteClass", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (classContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteCollection(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::CollectionDataContract,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteCollection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteCollection", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (collectionContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteCollectionToXml(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::CollectionDataContract,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteCollectionToXml")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteCollectionToXml", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlWriter, obj, context, collectionContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEndElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("WriteEndElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEndElement", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteMembers(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
        extensionData: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ExtensionDataObject,
        >,
        derivedMostClassContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ExtensionDataObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                        ),
                        i32,
                        3usize,
                    >("WriteMembers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteMembers", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (classContract, extensionData, derivedMostClassContract),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartElement(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        ns: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        namespaceLocal: quest_hook::libil2cpp::Gc<
            crate::System::Xml::XmlDictionaryString,
        >,
        nameLocal: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlDictionaryString>,
        nameIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Xml::XmlDictionaryString,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("WriteStartElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteStartElement", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (_cordl_type, ns, namespaceLocal, nameLocal, nameIndex),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteToXml(
        &mut self,
        xmlWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlWriterDelegator,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
        >,
        dataContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlWriterDelegator,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::XmlObjectSerializerWriteContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::ClassDataContract,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteToXml")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteToXml", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (xmlWriter, obj, context, dataContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteValue(
        &mut self,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        writeXsiType: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("WriteValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteValue", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (memberType, memberValue, writeXsiType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _WriteCollection_b__24_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("<WriteCollection>b__24_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<WriteCollection>b__24_0", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ClassDataContract0(
        &mut self,
        classContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ClassDataContract,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (classContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CollectionDataContract1(
        &mut self,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::CollectionDataContract,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (collectionContract))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_classDataContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::ClassDataContract,
                        >,
                        0usize,
                    >("get_classDataContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_classDataContract", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ClassDataContract,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_collectionDataContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Serialization::CollectionDataContract,
                        >,
                        0usize,
                    >("get_collectionDataContract")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_collectionDataContract", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::CollectionDataContract,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+Serialization+XmlFormatWriterInterpreter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::XmlFormatWriterInterpreter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
