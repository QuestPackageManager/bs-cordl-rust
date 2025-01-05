#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonLoadSettings {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _commentHandling: crate::Newtonsoft::Json::Linq::CommentHandling,
    pub _lineInfoHandling: crate::Newtonsoft::Json::Linq::LineInfoHandling,
    pub _duplicatePropertyNameHandling: crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JsonLoadSettings =>
    "Newtonsoft.Json.Linq"."JsonLoadSettings"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_CommentHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Linq::CommentHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::CommentHandling = __cordl_object
            .invoke("get_CommentHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DuplicatePropertyNameHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling = __cordl_object
            .invoke("get_DuplicatePropertyNameHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineInfoHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Linq::LineInfoHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::LineInfoHandling = __cordl_object
            .invoke("get_LineInfoHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CommentHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::CommentHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CommentHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DuplicatePropertyNameHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::DuplicatePropertyNameHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DuplicatePropertyNameHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_LineInfoHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::LineInfoHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LineInfoHandling", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonLoadSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonLoadSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
