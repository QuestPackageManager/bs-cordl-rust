#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_BoolSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, bool>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_BoolSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/BoolSerializer`1" < T >
);
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
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_ByteSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/ByteSerializer`1" < T >
);
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
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_CharSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/CharSerializer`1" < T >
);
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
    pub fn ElementWrite(
        &mut self,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
        prop: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementWrite", (w, prop))?;
        Ok(__cordl_ret)
    }
    pub fn ElementRead(
        &mut self,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
        prop: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementRead", (r, prop))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
    pub _serializers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
    >,
    pub _membersCount: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetSerializer_ClassInfo_1 < T
    > => "LiteNetLib.Utils"."NetSerializer/ClassInfo`1" < T >
);
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> {
    type Target = crate::System::Object;
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
    pub fn Write(
        &mut self,
        obj: T,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (obj, writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        serializers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializers))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        obj: T,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (obj, reader))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        serializers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializers))?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetSerializer_CustomType =>
    "LiteNetLib.Utils"."NetSerializer/CustomType"
);
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NetSerializer_CustomType {
    type Target = crate::System::Object;
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
        *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub _constructor: *mut crate::System::Func_1<TProperty>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1 < TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/CustomTypeClass`1" < TProperty >
);
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
        *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
    >
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        constructor: *mut crate::System::Func_1<TProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (constructor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        constructor: *mut crate::System::Func_1<TProperty>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructor))?;
        Ok(__cordl_object)
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
    pub _writer: *mut crate::System::Action_2<
        *mut crate::LiteNetLib::Utils::NetDataWriter,
        TProperty,
    >,
    pub _reader: *mut crate::System::Func_2<
        *mut crate::LiteNetLib::Utils::NetDataReader,
        TProperty,
    >,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1 < TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/CustomTypeStatic`1" < TProperty >
);
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
    pub fn _ctor(
        &mut self,
        writer: *mut crate::System::Action_2<
            *mut crate::LiteNetLib::Utils::NetDataWriter,
            TProperty,
        >,
        reader: *mut crate::System::Func_2<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            TProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (writer, reader))?;
        Ok(__cordl_ret)
    }
    pub fn Get<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
    >
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        writer: *mut crate::System::Action_2<
            *mut crate::LiteNetLib::Utils::NetDataWriter,
            TProperty,
        >,
        reader: *mut crate::System::Func_2<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            TProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (writer, reader))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1 < TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/CustomTypeStruct`1" < TProperty >
);
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
        *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T>,
    >
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/DoubleSerializer`1" < T >
);
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
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub Property: *mut crate::System::Reflection::PropertyInfo,
    pub PropertyType: *mut crate::System::Type,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/EnumByteSerializer`1" < T >
);
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
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        property: *mut crate::System::Reflection::PropertyInfo,
        propertyType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (property, propertyType))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        property: *mut crate::System::Reflection::PropertyInfo,
        propertyType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (property, propertyType))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/EnumIntSerializer`1" < T >
);
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
    pub fn _ctor(
        &mut self,
        property: *mut crate::System::Reflection::PropertyInfo,
        propertyType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (property, propertyType))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        property: *mut crate::System::Reflection::PropertyInfo,
        propertyType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (property, propertyType))?;
        Ok(__cordl_object)
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
    pub _constructor: *mut crate::System::Func_1<TProperty>,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetSerializer_FastCallClass_2
    < TClass, TProperty > => "LiteNetLib.Utils"."NetSerializer/FastCallClass`2" < TClass,
    TProperty >
);
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
    pub fn Write(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        constructor: *mut crate::System::Func_1<TProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (constructor))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        constructor: *mut crate::System::Func_1<TProperty>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructor))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2 < TClass, TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/FastCallSpecificAuto`2" < TClass, TProperty >
);
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
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn ElementRead(
        &mut self,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
        prop: quest_hook::libil2cpp::ByRefMut<TProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementRead", (r, prop))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn ElementWrite(
        &mut self,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
        prop: quest_hook::libil2cpp::ByRefMut<TProperty>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementWrite", (w, prop))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub Getter: *mut crate::System::Func_2<TClass, TProperty>,
    pub Setter: *mut crate::System::Action_2<TClass, TProperty>,
    pub GetterArr: *mut crate::System::Func_2<
        TClass,
        *mut quest_hook::libil2cpp::Il2CppArray<TProperty>,
    >,
    pub SetterArr: *mut crate::System::Action_2<
        TClass,
        *mut quest_hook::libil2cpp::Il2CppArray<TProperty>,
    >,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2 < TClass, TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/FastCallSpecific`2" < TClass, TProperty >
);
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
        getMethod: *mut crate::System::Reflection::MethodInfo,
        setMethod: *mut crate::System::Reflection::MethodInfo,
        isArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (getMethod, setMethod, isArray))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArrayHelper(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<TProperty>,
    >
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<TProperty> = __cordl_object
            .invoke("WriteArrayHelper", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArrayHelper(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<TProperty>,
    >
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<TProperty> = __cordl_object
            .invoke("ReadArrayHelper", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub _writer: *mut crate::System::Action_2<
        *mut crate::LiteNetLib::Utils::NetDataWriter,
        TProperty,
    >,
    pub _reader: *mut crate::System::Func_2<
        *mut crate::LiteNetLib::Utils::NetDataReader,
        TProperty,
    >,
    __cordl_phantom_TClass: std::marker::PhantomData<TClass>,
    __cordl_phantom_TProperty: std::marker::PhantomData<TProperty>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallStatic_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_FastCallStatic_2 < TClass, TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/FastCallStatic`2" < TClass, TProperty >
);
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
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        write: *mut crate::System::Action_2<
            *mut crate::LiteNetLib::Utils::NetDataWriter,
            TProperty,
        >,
        read: *mut crate::System::Func_2<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            TProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (write, read))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        write: *mut crate::System::Action_2<
            *mut crate::LiteNetLib::Utils::NetDataWriter,
            TProperty,
        >,
        read: *mut crate::System::Func_2<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            TProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (write, read))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_FastCallStruct_2 < TClass, TProperty > =>
    "LiteNetLib.Utils"."NetSerializer/FastCallStruct`2" < TClass, TProperty >
);
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
    pub fn Read(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: TClass,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: TClass,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TClass: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TProperty: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    __cordl_parent: crate::System::Object,
    pub IsArray: bool,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetSerializer_FastCall_1 < T
    > => "LiteNetLib.Utils"."NetSerializer/FastCall`1" < T >
);
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_FastCall_1<T> {
    type Target = crate::System::Object;
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
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        getMethod: *mut crate::System::Reflection::MethodInfo,
        setMethod: *mut crate::System::Reflection::MethodInfo,
        isArray: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (getMethod, setMethod, isArray))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_FloatSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/FloatSerializer`1" < T >
);
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
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
        *mut crate::System::Net::IPEndPoint,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/IPEndPointSerializer`1" < T >
);
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        T,
        *mut crate::System::Net::IPEndPoint,
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
    pub fn ElementWrite(
        &mut self,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
        prop: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementWrite", (w, prop))?;
        Ok(__cordl_ret)
    }
    pub fn ElementRead(
        &mut self,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
        prop: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Net::IPEndPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ElementRead", (r, prop))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetSerializer_IntSerializer_1
    < T > => "LiteNetLib.Utils"."NetSerializer/IntSerializer`1" < T >
);
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
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_LongSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/LongSerializer`1" < T >
);
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
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer {
    __cordl_parent: crate::System::Object,
    pub _writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    pub _maxStringLength: i32,
    pub _registeredTypes: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Type,
        *mut crate::LiteNetLib::Utils::NetSerializer_CustomType,
    >,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NetSerializer =>
    "LiteNetLib.Utils"."NetSerializer"
);
#[cfg(feature = "LiteNetLib+Utils+NetSerializer")]
impl std::ops::Deref for crate::LiteNetLib::Utils::NetSerializer {
    type Target = crate::System::Object;
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
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomType")]
    pub type CustomType = crate::LiteNetLib::Utils::NetSerializer_CustomType;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStatic_1")]
    pub type CustomTypeStatic_1<TProperty: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CustomTypeStatic_1<
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCall_1")]
    pub type FastCall_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_FastCall_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecific_2")]
    pub type FastCallSpecific_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<TClass, TProperty>;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+UIntSerializer_1")]
    pub type UIntSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_UIntSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+LongSerializer_1")]
    pub type LongSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_LongSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FloatSerializer_1")]
    pub type FloatSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_FloatSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+UShortSerializer_1")]
    pub type UShortSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_UShortSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CharSerializer_1")]
    pub type CharSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CharSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+DoubleSerializer_1")]
    pub type DoubleSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_DoubleSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+IntSerializer_1")]
    pub type IntSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_IntSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+BoolSerializer_1")]
    pub type BoolSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_BoolSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumByteSerializer_1")]
    pub type EnumByteSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_EnumByteSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallClass_2")]
    pub type FastCallClass_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallClass_2<TClass, TProperty>;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ClassInfo_1")]
    pub type ClassInfo_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeStruct_1")]
    pub type CustomTypeStruct_1<TProperty: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CustomTypeStruct_1<
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+CustomTypeClass_1")]
    pub type CustomTypeClass_1<TProperty: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_CustomTypeClass_1<
        TProperty,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+EnumIntSerializer_1")]
    pub type EnumIntSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_EnumIntSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
    pub type SByteSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_SByteSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+FastCallSpecificAuto_2")]
    pub type FastCallSpecificAuto_2<
        TClass: quest_hook::libil2cpp::Type,
        TProperty: quest_hook::libil2cpp::Type,
    > = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecificAuto_2<
        TClass,
        TProperty,
    >;
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
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ShortSerializer_1")]
    pub type ShortSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ShortSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ULongSerializer_1")]
    pub type ULongSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ULongSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+ByteSerializer_1")]
    pub type ByteSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_ByteSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+IPEndPointSerializer_1")]
    pub type IPEndPointSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_IPEndPointSerializer_1<
        T,
    >;
    #[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
    pub type StringSerializer_1<T: quest_hook::libil2cpp::Type> = crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<
        T,
    >;
    pub fn RegisterNestedType_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterNestedType", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterNestedType_Func_1_1<T>(
        &mut self,
        constructor: *mut crate::System::Func_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterNestedType", (constructor))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterNestedType_Action_2_Func_2_2<T>(
        &mut self,
        writer: *mut crate::System::Action_2<
            *mut crate::LiteNetLib::Utils::NetDataWriter,
            T,
        >,
        reader: *mut crate::System::Func_2<
            *mut crate::LiteNetLib::Utils::NetDataReader,
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterNestedType", (writer, reader))?;
        Ok(__cordl_ret)
    }
    pub fn Register<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", ())?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_NetDataWriter_T0<T>(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        obj: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer, obj))?;
        Ok(__cordl_ret)
    }
    pub fn Serialize_T1<T>(
        &mut self,
        obj: T,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Serialize", (obj))?;
        Ok(__cordl_ret)
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
    pub fn _ctor_i32_1(
        &mut self,
        maxStringLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxStringLength))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterInternal<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::LiteNetLib::Utils::NetSerializer_ClassInfo_1<T> = __cordl_object
            .invoke("RegisterInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_NetDataReader0<T>(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize_T1<T>(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        target: T,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Deserialize", (reader, target))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(
        maxStringLength: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxStringLength))?;
        Ok(__cordl_object)
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
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSerializer_SByteSerializer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<T, i8>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+SByteSerializer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_SByteSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/SByteSerializer`1" < T >
);
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
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_ShortSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/ShortSerializer`1" < T >
);
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
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
        *mut crate::System::String,
    >,
    pub _maxLength: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_StringSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/StringSerializer`1" < T >
);
#[cfg(feature = "LiteNetLib+Utils+NetSerializer+StringSerializer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::LiteNetLib::Utils::NetSerializer_StringSerializer_1<T> {
    type Target = crate::LiteNetLib::Utils::NetSerializer_FastCallSpecific_2<
        T,
        *mut crate::System::String,
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
    pub fn _ctor(
        &mut self,
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxLength))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn New(maxLength: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxLength))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_UIntSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/UIntSerializer`1" < T >
);
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
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_ULongSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/ULongSerializer`1" < T >
);
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
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::NetSerializer_UShortSerializer_1 < T > => "LiteNetLib.Utils"
    ."NetSerializer/UShortSerializer`1" < T >
);
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
    pub fn Read(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Read", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn ReadArray(
        &mut self,
        inf: T,
        r: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadArray", (inf, r))?;
        Ok(__cordl_ret)
    }
    pub fn WriteArray(
        &mut self,
        inf: T,
        w: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteArray", (inf, w))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
