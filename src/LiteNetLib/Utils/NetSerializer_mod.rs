#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    pub _maxStringLength: i32,
    pub _registeredTypes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetSerializer_CustomType>,
        >,
    >,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
unsafe impl quest_hook::libil2cpp::Type for crate::LiteNetLib::Utils::NetSerializer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer";
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NetSerializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::NetSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
impl crate::LiteNetLib::Utils::NetSerializer {
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
    pub type BoolSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
    pub type ByteSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
    pub type CharSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
    pub type ClassInfo_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
    pub type CustomType = crate::LiteNetLib::Utils::NetSerializer_CustomType;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
    pub type CustomTypeClass_1<TProperty: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
    pub type CustomTypeStatic_1<TProperty: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
    pub type CustomTypeStruct_1<TProperty: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
    pub type DoubleSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
    pub type EnumByteSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
    pub type EnumIntSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
    pub type FastCallClass_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty>;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
    pub type FastCallSpecificAuto_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        TClass,
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
    pub type FastCallSpecific_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty>;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
    pub type FastCallStatic_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<TClass, TProperty>;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
    pub type FastCallStruct_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<TClass, TProperty>;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
    pub type FastCall_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
    pub type FloatSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
    pub type IPEndPointSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
    pub type IntSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
    pub type LongSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
    pub type SByteSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
    pub type ShortSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
    pub type StringSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
    pub type UIntSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
    pub type ULongSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
    pub type UShortSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
        T,
    >;
    pub fn Deserialize_NetDataReader0<T>(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                T,
                1usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "Deserialize", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, (reader))? };
        Ok(__cordl_ret.into())
    }
    pub fn Deserialize_T1<T>(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        target: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>, T),
                bool,
                2usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "Deserialize", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (reader, target))?
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
    pub fn New_i32_1(
        maxStringLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxStringLength))?;
        Ok(__cordl_object.into())
    }
    pub fn Register<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Register")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "Register", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterInternal<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T>,
                >,
                0usize,
            >("RegisterInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterInternal", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNestedType_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RegisterNestedType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterNestedType", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNestedType_Action_2_Func_2_2<T>(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                T,
            >,
        >,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                T,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataWriter,
                            >,
                            T,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataReader,
                            >,
                            T,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterNestedType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterNestedType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNestedType_Func_1_1<T>(
        &mut self,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterNestedType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "RegisterNestedType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (constructor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_NetDataWriter_T0<T>(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        obj: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>, T),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "Serialize", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize_T1<T>(
        &mut self,
        obj: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                1usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), "Serialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (obj))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        maxStringLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (maxStringLength))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::NetSerializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_BoolSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, bool>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/BoolSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/BoolSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_BoolSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_BoolSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_BoolSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_BoolSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_BoolSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_ByteSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u8>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/ByteSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/ByteSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u8>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_CharSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        T,
        char,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/CharSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/CharSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        T,
        char,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<T> {
    pub fn ElementRead(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        prop: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                    quest_hook::libil2cpp::ByRefMut<char>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ElementRead")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CharSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ElementRead", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (r, prop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ElementWrite(
        &mut self,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        prop: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    quest_hook::libil2cpp::ByRefMut<char>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ElementWrite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CharSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ElementWrite", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (w, prop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CharSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_ClassInfo_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _serializers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
            >,
        >,
    >,
    pub _membersCount: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/ClassInfo`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/ClassInfo`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> {
    pub fn New(
        serializers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializers))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        obj: T,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ClassInfo_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obj, reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        obj: T,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ClassInfo_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obj, writer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        serializers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ClassInfo_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (serializers))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_CustomType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_CustomType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/CustomType";
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NetSerializer_CustomType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::NetSerializer_CustomType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
impl crate::LiteNetLib::Utils::NetSerializer_CustomType {
    pub fn Get<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomType as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                >,
                0usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomType as
                    quest_hook::libil2cpp::Type > ::class(), "Get", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomType as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomType as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
impl quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_CustomType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_CustomTypeClass_1<TProperty: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_CustomType,
    pub _constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
unsafe impl<TProperty: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/CustomTypeClass`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/CustomTypeClass`1",
                    )
                    .unwrap()
                    .make_generic::<(TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
impl<TProperty: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_CustomType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
impl<TProperty: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
impl<
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<TProperty> {
    pub fn Get<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>>,
    >
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                >,
                0usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1 <
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Get", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1 <
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (constructor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
impl<TProperty: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_CustomTypeStatic_1<TProperty: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_CustomType,
    pub _writer: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
            TProperty,
        >,
    >,
    pub _reader: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
            TProperty,
        >,
    >,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
unsafe impl<TProperty: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/CustomTypeStatic`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/CustomTypeStatic`1",
                    )
                    .unwrap()
                    .make_generic::<(TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
impl<TProperty: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_CustomType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
impl<TProperty: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
impl<
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<TProperty> {
    pub fn Get<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>>,
    >
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                >,
                0usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1 <
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Get", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                TProperty,
            >,
        >,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                TProperty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer, reader))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                TProperty,
            >,
        >,
        reader: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                TProperty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataWriter,
                            >,
                            TProperty,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataReader,
                            >,
                            TProperty,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1 <
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, reader))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
impl<TProperty: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_CustomTypeStruct_1<TProperty: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_CustomType,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
unsafe impl<TProperty: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/CustomTypeStruct`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/CustomTypeStruct`1",
                    )
                    .unwrap()
                    .make_generic::<(TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
impl<TProperty: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_CustomType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
impl<TProperty: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
impl<
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<TProperty> {
    pub fn Get<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>>,
    >
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
                >,
                0usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1 <
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Get", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1 <
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
impl<TProperty: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_DoubleSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, f64>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/DoubleSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/DoubleSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, f64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_EnumByteSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
    pub Property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    pub PropertyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/EnumByteSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/EnumByteSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T> {
    pub fn New(
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        propertyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (property, propertyType))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1 < T >
                    as quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1 < T >
                    as quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        propertyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1 < T >
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (property, propertyType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_EnumIntSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/EnumIntSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/EnumIntSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<T> {
    pub fn New(
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        propertyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (property, propertyType))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        propertyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (property, propertyType))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FastCallClass_2<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >,
    pub _constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
unsafe impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FastCallClass`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FastCallClass`2",
                    )
                    .unwrap()
                    .make_generic::<(TClass, TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty> {
    pub fn New(
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructor))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallClass_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Read",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallClass_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "ReadArray",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallClass_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Write",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallClass_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "WriteArray",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Func_1<TProperty>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallClass_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (constructor))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FastCallSpecificAuto_2<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
unsafe impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<TClass, TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FastCallSpecificAuto`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FastCallSpecificAuto`2",
                    )
                    .unwrap()
                    .make_generic::<(TClass, TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<TClass, TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<TClass, TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<TClass, TProperty> {
    pub fn ElementRead(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        prop: quest_hook::libil2cpp::ByRefMut<TProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                    quest_hook::libil2cpp::ByRefMut<TProperty>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ElementRead")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "ElementRead", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (r, prop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ElementWrite(
        &mut self,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        prop: quest_hook::libil2cpp::ByRefMut<TProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    quest_hook::libil2cpp::ByRefMut<TProperty>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ElementWrite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "ElementWrite", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (w, prop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 <
                    TClass, TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<TClass, TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FastCallSpecific_2<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCall_1<TClass>,
    pub Getter: quest_hook::libil2cpp::Gc<crate::System::Func_2<TClass, TProperty>>,
    pub Setter: quest_hook::libil2cpp::Gc<crate::System::Action_2<TClass, TProperty>>,
    pub GetterArr: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            TClass,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TProperty>>,
        >,
    >,
    pub SetterArr: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            TClass,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TProperty>>,
        >,
    >,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
unsafe impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FastCallSpecific`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FastCallSpecific`2",
                    )
                    .unwrap()
                    .make_generic::<(TClass, TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCall_1<TClass>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty> {
    pub fn Init(
        &mut self,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        isArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Init",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (getMethod, setMethod, isArray))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadArrayHelper(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TProperty>>,
    >
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TProperty>>,
                2usize,
            >("ReadArrayHelper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "ReadArrayHelper", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TProperty>,
        > = unsafe { method.invoke_unchecked(self, (inf, r))? };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArrayHelper(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TProperty>>,
    >
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TProperty>>,
                2usize,
            >("WriteArrayHelper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(),
                    "WriteArrayHelper", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TProperty>,
        > = unsafe { method.invoke_unchecked(self, (inf, w))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FastCallStatic_2<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >,
    pub _writer: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
            TProperty,
        >,
    >,
    pub _reader: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
            TProperty,
        >,
    >,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
unsafe impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<TClass, TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FastCallStatic`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FastCallStatic`2",
                    )
                    .unwrap()
                    .make_generic::<(TClass, TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<TClass, TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<TClass, TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<TClass, TProperty> {
    pub fn New(
        write: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                TProperty,
            >,
        >,
        read: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                TProperty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (write, read))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStatic_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Read",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStatic_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "ReadArray",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStatic_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Write",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStatic_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "WriteArray",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        write: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                TProperty,
            >,
        >,
        read: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                TProperty,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataWriter,
                            >,
                            TProperty,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<
                                crate::LiteNetLib::Utils::NetDataReader,
                            >,
                            TProperty,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStatic_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (write, read))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FastCallStatic_2<TClass, TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FastCallStruct_2<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >,
    pub _p: TProperty,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
unsafe impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<TClass, TProperty> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FastCallStruct`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FastCallStruct`2",
                    )
                    .unwrap()
                    .make_generic::<(TClass, TProperty)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<TClass, TProperty> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        TClass,
        TProperty,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<TClass, TProperty> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<TClass, TProperty> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStruct_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Read",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStruct_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "ReadArray",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStruct_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "Write",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    TClass,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStruct_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), "WriteArray",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<
            TClass,
            TProperty,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCallStruct_2 < TClass,
                    TProperty > as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStruct_2")]
impl<
    TClass: quest_hook::libil2cpp::Type,
    TProperty: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FastCallStruct_2<TClass, TProperty> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FastCall_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub IsArray: bool,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FastCall`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FastCall`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> {
    pub fn Init(
        &mut self,
        getMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        setMethod: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        isArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                    quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (getMethod, setMethod, isArray))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_FloatSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, f32>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/FloatSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/FloatSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FloatSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FloatSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FloatSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FloatSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_FloatSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_IPEndPointSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        T,
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/IPEndPointSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/IPEndPointSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        T,
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<T> {
    pub fn ElementRead(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        prop: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ElementRead")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1 < T >
                    as quest_hook::libil2cpp::Type > ::class(), "ElementRead", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (r, prop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ElementWrite(
        &mut self,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        prop: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ElementWrite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1 < T >
                    as quest_hook::libil2cpp::Type > ::class(), "ElementWrite", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (w, prop))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1 < T >
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_IntSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i32>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/IntSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/IntSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_IntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_LongSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i64>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/LongSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/LongSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_LongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_LongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_LongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_LongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_LongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_SByteSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i8>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/SByteSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/SByteSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i8>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_SByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_SByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_SByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_SByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_SByteSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_ShortSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i16>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/ShortSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/ShortSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i16>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_StringSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        T,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _maxLength: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/StringSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/StringSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        T,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<T> {
    pub fn New(
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxLength))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_StringSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_StringSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_StringSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_StringSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_StringSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (maxLength))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_UIntSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u32>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/UIntSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/UIntSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UIntSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_ULongSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u64>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/ULongSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/ULongSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ULongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ULongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ULongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ULongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_ULongSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_UShortSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u16>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LiteNetLib.Utils";
    const CLASS_NAME: &'static str = "NetSerializer/UShortSerializer`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "LiteNetLib.Utils",
                        "NetSerializer/UShortSerializer`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, u16>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Read")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Read", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ReadArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "ReadArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, r))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Write")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "Write", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (T, quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), "WriteArray", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inf, w))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
            T,
        > as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::LiteNetLib::Utils::NetSerializer_UShortSerializer_1 < T > as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
