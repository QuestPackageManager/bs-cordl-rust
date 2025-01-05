#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultReferenceResolver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _referenceCount: i32,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DefaultReferenceResolver =>
    "Newtonsoft.Json.Serialization"."DefaultReferenceResolver"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    pub fn AddReference(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        reference: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReference", (context, reference, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMappings(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("GetMappings", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReference(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetReference", (context, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsReferenced(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsReferenced", (context, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveReference(
        &mut self,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        reference: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("ResolveReference", (context, reference))?;
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
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl AsRef<crate::Newtonsoft::Json::Serialization::IReferenceResolver>
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Serialization::IReferenceResolver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultReferenceResolver")]
impl AsMut<crate::Newtonsoft::Json::Serialization::IReferenceResolver>
for crate::Newtonsoft::Json::Serialization::DefaultReferenceResolver {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Newtonsoft::Json::Serialization::IReferenceResolver {
        unsafe { std::mem::transmute(self) }
    }
}
