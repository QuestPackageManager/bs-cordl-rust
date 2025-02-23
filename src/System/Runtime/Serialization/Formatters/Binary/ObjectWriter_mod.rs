#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectWriter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_objectQueue: quest_hook::libil2cpp::Gc<crate::System::Collections::Queue>,
    pub m_idGenerator: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ObjectIDGenerator,
    >,
    pub m_currentId: i32,
    pub m_surrogates: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::ISurrogateSelector,
    >,
    pub m_context: crate::System::Runtime::Serialization::StreamingContext,
    pub serWriter: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
    >,
    pub m_objectManager: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationObjectManager,
    >,
    pub topId: i64,
    pub topName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub headers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Runtime::Remoting::Messaging::Header,
            >,
        >,
    >,
    pub formatterEnums: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
    >,
    pub m_binder: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationBinder,
    >,
    pub serObjectInfoInit: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::SerObjectInfoInit,
    >,
    pub m_formatterConverter: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::IFormatterConverter,
    >,
    pub crossAppDomainArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub previousObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub previousId: i64,
    pub previousType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub previousCode: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    pub assemblyToIdTable: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Hashtable,
    >,
    pub niPool: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::Formatters::Binary::SerStack,
    >,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Serialization.Formatters.Binary";
    const CLASS_NAME: &'static str = "ObjectWriter";
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
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    pub fn CheckForNull(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                4usize,
            >("CheckForNull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckForNull", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(self, (objectInfo, memberNameInfo, typeNameInfo, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckTypeFormat(
        &mut self,
        test: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
        want: crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
                    crate::System::Runtime::Serialization::Formatters::FormatterTypeStyle,
                ),
                bool,
                2usize,
            >("CheckTypeFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CheckTypeFormat", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (test, want)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblyId(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                >),
                i64,
                1usize,
            >("GetAssemblyId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAssemblyId", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, (objectInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNameInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >,
                0usize,
            >("GetNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNameInfo", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNext(
        &mut self,
        objID: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<i64>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                1usize,
            >("GetNext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNext", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (objID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetType(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                1usize,
            >("GetType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            method.invoke_unchecked(self, (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetId(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        assignUniqueIdToValueType: bool,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        isNew: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                i64,
                4usize,
            >("InternalGetId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalGetId", 4usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (obj, assignUniqueIdToValueType, _cordl_type, isNew),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn MemberToNameInfo(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >,
                1usize,
            >("MemberToNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MemberToNameInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        > = unsafe { method.invoke_unchecked(self, (name)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        formatterEnums: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
        >,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (selector, context, formatterEnums, binder))?;
        Ok(__cordl_object.into())
    }
    pub fn PutNameInfo(
        &mut self,
        nameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PutNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PutNameInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Schedule_Il2CppObject__cordl_bool_Type0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        assignUniqueIdToValueType: bool,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                i64,
                3usize,
            >("Schedule")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Schedule", 3usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method.invoke_unchecked(self, (obj, assignUniqueIdToValueType, _cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Schedule_WriteObjectInfo1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        assignUniqueIdToValueType: bool,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                ),
                i64,
                4usize,
            >("Schedule")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Schedule", 4usize
                )
            });
        let __cordl_ret: i64 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (obj, assignUniqueIdToValueType, _cordl_type, objectInfo),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        inHeaders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Remoting::Messaging::Header,
                >,
            >,
        >,
        serWriter: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
        >,
        fCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Remoting::Messaging::Header,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::__BinaryWriter,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Serialize", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (graph, inHeaders, serWriter, fCheck))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToCode(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
                1usize,
            >("ToCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToCode", 1usize
                )
            });
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE = unsafe {
            method.invoke_unchecked(self, (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TypeToNameInfo_Type1(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >,
                1usize,
            >("TypeToNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TypeToNameInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
    pub fn TypeToNameInfo_Type_NameInfo4(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("TypeToNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TypeToNameInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (_cordl_type, nameInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TypeToNameInfo_Type_WriteObjectInfo_InternalPrimitiveTypeE_NameInfo0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        code: crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
        nameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    crate::System::Runtime::Serialization::Formatters::Binary::InternalPrimitiveTypeE,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >,
                4usize,
            >("TypeToNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TypeToNameInfo", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        > = unsafe {
            method.invoke_unchecked(self, (_cordl_type, objectInfo, code, nameInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TypeToNameInfo_WriteObjectInfo2(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >,
                1usize,
            >("TypeToNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TypeToNameInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        > = unsafe { method.invoke_unchecked(self, (objectInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn TypeToNameInfo_WriteObjectInfo_NameInfo3(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        nameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                >,
                2usize,
            >("TypeToNameInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TypeToNameInfo", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        > = unsafe { method.invoke_unchecked(self, (objectInfo, nameInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        memberObjectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteArray", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (objectInfo, memberNameInfo, memberObjectInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArrayMember(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        arrayElemTypeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteArrayMember")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteArrayMember", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (objectInfo, arrayElemTypeNameInfo, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteKnownValueClass(
        &mut self,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                bool,
                3usize,
            >("WriteKnownValueClass")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteKnownValueClass", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (memberNameInfo, typeNameInfo, data))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteMemberSetup(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        memberName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        memberType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        memberObjectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("WriteMemberSetup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteMemberSetup", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        objectInfo,
                        memberNameInfo,
                        typeNameInfo,
                        memberName,
                        memberType,
                        memberData,
                        memberObjectInfo,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteMembers(
        &mut self,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        memberTypeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        memberData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        memberObjectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("WriteMembers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteMembers", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        memberNameInfo,
                        memberTypeNameInfo,
                        memberData,
                        objectInfo,
                        typeNameInfo,
                        memberObjectInfo,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectRef(
        &mut self,
        nameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        objectId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    i64,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteObjectRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteObjectRef", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (nameInfo, objectId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteRectangle(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        rank: i32,
        maxA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        arrayElemNameTypeInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        lowerBoundA: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    quest_hook::libil2cpp::Gc<crate::System::Array>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("WriteRectangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteRectangle", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (objectInfo, rank, maxA, array, arrayElemNameTypeInfo, lowerBoundA),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteSerializedStreamHeader(
        &mut self,
        topId: i64,
        headerId: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64, i64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteSerializedStreamHeader")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteSerializedStreamHeader", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topId, headerId))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteString(
        &mut self,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        stringObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteString", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (memberNameInfo, typeNameInfo, stringObject))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_Il2CppArray_Il2CppArray_Il2CppArray_Il2CppArray1(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        memberTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        memberData: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        memberObjectInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Write", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        objectInfo,
                        memberNameInfo,
                        typeNameInfo,
                        memberNames,
                        memberTypes,
                        memberData,
                        memberObjectInfos,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write_WriteObjectInfo_NameInfo_NameInfo0(
        &mut self,
        objectInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
        >,
        memberNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
        typeNameInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::WriteObjectInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::NameInfo,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Write", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (objectInfo, memberNameInfo, typeNameInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        selector: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISurrogateSelector,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
        formatterEnums: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
        >,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::ISurrogateSelector,
                    >,
                    crate::System::Runtime::Serialization::StreamingContext,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::Formatters::Binary::InternalFE,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Runtime::Serialization::SerializationBinder,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (selector, context, formatterEnums, binder))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationObjectManager,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::SerializationObjectManager,
                >,
                0usize,
            >("get_ObjectManager")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ObjectManager", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationObjectManager,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+ObjectWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::ObjectWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
