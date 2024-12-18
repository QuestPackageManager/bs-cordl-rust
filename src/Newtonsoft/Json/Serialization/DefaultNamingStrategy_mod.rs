#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultNamingStrategy")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultNamingStrategy {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::NamingStrategy,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultNamingStrategy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DefaultNamingStrategy =>
    "Newtonsoft.Json.Serialization"."DefaultNamingStrategy"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultNamingStrategy")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::DefaultNamingStrategy {
    type Target = crate::Newtonsoft::Json::Serialization::NamingStrategy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultNamingStrategy")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DefaultNamingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultNamingStrategy")]
impl crate::Newtonsoft::Json::Serialization::DefaultNamingStrategy {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolvePropertyName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolvePropertyName", (name))?;
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
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultNamingStrategy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DefaultNamingStrategy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
