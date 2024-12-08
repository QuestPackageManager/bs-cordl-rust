#[cfg(feature = "RemoteProcedureCall_2")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteProcedureCall_2<
    T0: quest_hook::libil2cpp::Type,
    T1: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
    pub _value0: *mut crate::GlobalNamespace::RemoteProcedureCall_TypeWrapper_1<T0>,
    pub _value1: *mut crate::GlobalNamespace::RemoteProcedureCall_TypeWrapper_1<T1>,
    __cordl_phantom_T0: std::marker::PhantomData<T0>,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "RemoteProcedureCall_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RemoteProcedureCall_2 < T0, T1
    > => ""."RemoteProcedureCall`2" < T0, T1 >
);
#[cfg(feature = "RemoteProcedureCall_2")]
impl<T0: quest_hook::libil2cpp::Type, T1: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::RemoteProcedureCall_2<T0, T1> {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RemoteProcedureCall_2")]
impl<T0: quest_hook::libil2cpp::Type, T1: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::RemoteProcedureCall_2<T0, T1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RemoteProcedureCall_2")]
impl<
    T0: quest_hook::libil2cpp::Type,
    T1: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::RemoteProcedureCall_2<T0, T1> {
    pub fn DeserializeData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
        protocolVersion: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeData", (reader, protocolVersion))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        syncTime: i64,
        value0: T0,
        value1: T1,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IRemoteProcedureCall>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IRemoteProcedureCall = __cordl_object
            .invoke("Init", (syncTime, value0, value1))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn SerializeData(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
        protocolVersion: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeData", (writer, protocolVersion))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_value0(&mut self) -> quest_hook::libil2cpp::Result<T0>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T0 = __cordl_object.invoke("get_value0", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_value1(&mut self) -> quest_hook::libil2cpp::Result<T1>
    where
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T1 = __cordl_object.invoke("get_value1", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RemoteProcedureCall_2")]
impl<
    T0: quest_hook::libil2cpp::Type,
    T1: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RemoteProcedureCall_2<T0, T1> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
