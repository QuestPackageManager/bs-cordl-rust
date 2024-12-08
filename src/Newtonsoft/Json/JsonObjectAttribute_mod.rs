#[cfg(feature = "Newtonsoft+Json+JsonObjectAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonObjectAttribute {
    __cordl_parent: crate::Newtonsoft::Json::JsonContainerAttribute,
    pub _memberSerialization: crate::Newtonsoft::Json::MemberSerialization,
    pub _missingMemberHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::MissingMemberHandling,
    >,
    pub _itemRequired: crate::System::Nullable_1<crate::Newtonsoft::Json::Required>,
    pub _itemNullValueHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::NullValueHandling,
    >,
}
#[cfg(feature = "Newtonsoft+Json+JsonObjectAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonObjectAttribute =>
    "Newtonsoft.Json"."JsonObjectAttribute"
);
#[cfg(feature = "Newtonsoft+Json+JsonObjectAttribute")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonObjectAttribute {
    type Target = crate::Newtonsoft::Json::JsonContainerAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonObjectAttribute")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonObjectAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonObjectAttribute")]
impl crate::Newtonsoft::Json::JsonObjectAttribute {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_MemberSerialization1(
        memberSerialization: crate::Newtonsoft::Json::MemberSerialization,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (memberSerialization))?;
        Ok(__cordl_object)
    }
    pub fn New_String2(
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id))?;
        Ok(__cordl_object)
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
    pub fn _ctor_MemberSerialization1(
        &mut self,
        memberSerialization: crate::Newtonsoft::Json::MemberSerialization,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (memberSerialization))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String2(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id))?;
        Ok(__cordl_ret)
    }
    pub fn get_ItemNullValueHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::NullValueHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::NullValueHandling = __cordl_object
            .invoke("get_ItemNullValueHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ItemRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Required> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Required = __cordl_object
            .invoke("get_ItemRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberSerialization(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::MemberSerialization> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::MemberSerialization = __cordl_object
            .invoke("get_MemberSerialization", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MissingMemberHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::MissingMemberHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::MissingMemberHandling = __cordl_object
            .invoke("get_MissingMemberHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ItemNullValueHandling(
        &mut self,
        value: crate::Newtonsoft::Json::NullValueHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemNullValueHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ItemRequired(
        &mut self,
        value: crate::Newtonsoft::Json::Required,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ItemRequired", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MemberSerialization(
        &mut self,
        value: crate::Newtonsoft::Json::MemberSerialization,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MemberSerialization", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MissingMemberHandling(
        &mut self,
        value: crate::Newtonsoft::Json::MissingMemberHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MissingMemberHandling", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonObjectAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::JsonObjectAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
