#[cfg(feature = "Newtonsoft+Json+Linq+JRaw")]
#[repr(C)]
#[derive(Debug)]
pub struct JRaw {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JValue,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JRaw")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JRaw =>
    "Newtonsoft.Json.Linq"."JRaw"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JRaw")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JRaw {
    type Target = crate::Newtonsoft::Json::Linq::JValue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JRaw")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JRaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JRaw")]
impl crate::Newtonsoft::Json::Linq::JRaw {
    #[cfg(feature = "Newtonsoft+Json+Linq+JRaw+_CreateAsync_d__0")]
    pub type _CreateAsync_d__0 = crate::Newtonsoft::Json::Linq::JRaw__CreateAsync_d__0;
    pub fn CloneToken(
        &mut self,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonCloneSettings,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("CloneToken", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppObject2(
        rawJson: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rawJson))?;
        Ok(__cordl_object)
    }
    pub fn New_JRaw0(
        other: *mut crate::Newtonsoft::Json::Linq::JRaw,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object)
    }
    pub fn New_JRaw_JsonCloneSettings1(
        other: *mut crate::Newtonsoft::Json::Linq::JRaw,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonCloneSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other, settings))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppObject2(
        &mut self,
        rawJson: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rawJson))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JRaw0(
        &mut self,
        other: *mut crate::Newtonsoft::Json::Linq::JRaw,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JRaw_JsonCloneSettings1(
        &mut self,
        other: *mut crate::Newtonsoft::Json::Linq::JRaw,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonCloneSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other, settings))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JRaw")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Linq::JRaw {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
