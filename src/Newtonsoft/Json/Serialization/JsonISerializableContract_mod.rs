#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonISerializableContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContainerContract,
    pub _ISerializableCreator_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonISerializableContract =>
    "Newtonsoft.Json.Serialization"."JsonISerializableContract"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContainerContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    pub fn _ctor(
        &mut self,
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (underlyingType))?;
        Ok(__cordl_ret)
    }
    pub fn get_ISerializableCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_ISerializableCreator", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ISerializableCreator(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ISerializableCreator", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        underlyingType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
