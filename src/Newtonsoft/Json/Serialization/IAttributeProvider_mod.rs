#[cfg(feature = "Newtonsoft+Json+Serialization+IAttributeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IAttributeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+IAttributeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::IAttributeProvider =>
    "Newtonsoft.Json.Serialization"."IAttributeProvider"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+IAttributeProvider")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::IAttributeProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+IAttributeProvider")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::IAttributeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+IAttributeProvider")]
impl crate::Newtonsoft::Json::Serialization::IAttributeProvider {
    pub fn GetAttributes_Type__cordl_bool1(
        &mut self,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        > = __cordl_object.invoke("GetAttributes", (attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes__cordl_bool0(
        &mut self,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::System::Attribute>,
            >,
        > = __cordl_object.invoke("GetAttributes", (inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+IAttributeProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::IAttributeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
