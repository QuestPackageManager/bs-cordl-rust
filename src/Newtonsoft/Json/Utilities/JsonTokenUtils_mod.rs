#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonTokenUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::JsonTokenUtils =>
    "Newtonsoft.Json.Utilities"."JsonTokenUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    pub fn IsEndToken(
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEndToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitiveToken(
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrimitiveToken", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsStartToken(
        token: crate::Newtonsoft::Json::JsonToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsStartToken", (token))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+JsonTokenUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::JsonTokenUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
