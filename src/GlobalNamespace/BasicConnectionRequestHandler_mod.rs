#[cfg(feature = "BasicConnectionRequestHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicConnectionRequestHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _secret_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BasicConnectionRequestHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BasicConnectionRequestHandler
    => ""."BasicConnectionRequestHandler"
);
#[cfg(feature = "BasicConnectionRequestHandler")]
impl std::ops::Deref for crate::GlobalNamespace::BasicConnectionRequestHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicConnectionRequestHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::BasicConnectionRequestHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicConnectionRequestHandler")]
impl crate::GlobalNamespace::BasicConnectionRequestHandler {
    pub fn GetConnectionMessage(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetConnectionMessage",
                (writer, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateConnectionMessage(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        userId: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        userName: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        isConnectionOwner: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ValidateConnectionMessage",
                (reader, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_secret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_secret", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_secret(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_secret", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BasicConnectionRequestHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BasicConnectionRequestHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BasicConnectionRequestHandler")]
impl AsRef<crate::GlobalNamespace::IConnectionRequestHandler>
for crate::GlobalNamespace::BasicConnectionRequestHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnectionRequestHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BasicConnectionRequestHandler")]
impl AsMut<crate::GlobalNamespace::IConnectionRequestHandler>
for crate::GlobalNamespace::BasicConnectionRequestHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnectionRequestHandler {
        unsafe { std::mem::transmute(self) }
    }
}
