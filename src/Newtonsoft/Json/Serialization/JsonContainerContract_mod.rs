#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonContainerContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContract,
    pub _itemContract: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonContract,
    >,
    pub _finalItemContract: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonContract,
    >,
    pub _ItemConverter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::JsonConverter,
    >,
    pub _ItemIsReference_k__BackingField: crate::System::Nullable_1<bool>,
    pub _ItemReferenceLoopHandling_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::ReferenceLoopHandling,
    >,
    pub _ItemTypeNameHandling_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::TypeNameHandling,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonContainerContract =>
    "Newtonsoft.Json.Serialization"."JsonContainerContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    pub fn New(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (underlyingType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FinalItemContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = __cordl_object.invoke("get_FinalItemContract", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = __cordl_object.invoke("get_ItemContract", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = __cordl_object.invoke("get_ItemConverter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemIsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke("get_ItemIsReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemReferenceLoopHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::ReferenceLoopHandling>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::ReferenceLoopHandling,
        > = __cordl_object.invoke("get_ItemReferenceLoopHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemTypeNameHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::TypeNameHandling>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::TypeNameHandling,
        > = __cordl_object.invoke("get_ItemTypeNameHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemContract(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemContract", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemConverter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemConverter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemIsReference(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemIsReference", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemReferenceLoopHandling(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::ReferenceLoopHandling>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemReferenceLoopHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemTypeNameHandling(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::TypeNameHandling>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemTypeNameHandling", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
