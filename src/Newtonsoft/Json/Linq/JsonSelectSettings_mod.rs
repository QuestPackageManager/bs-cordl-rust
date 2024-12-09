#[cfg(feature = "Newtonsoft+Json+Linq+JsonSelectSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSelectSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _RegexMatchTimeout_k__BackingField: crate::System::Nullable_1<
        crate::System::TimeSpan,
    >,
    pub _ErrorWhenNoMatch_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonSelectSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JsonSelectSettings =>
    "Newtonsoft.Json.Linq"."JsonSelectSettings"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonSelectSettings")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonSelectSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonSelectSettings")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonSelectSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonSelectSettings")]
impl crate::Newtonsoft::Json::Linq::JsonSelectSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ErrorWhenNoMatch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ErrorWhenNoMatch", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RegexMatchTimeout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::TimeSpan>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::TimeSpan> = __cordl_object
            .invoke("get_RegexMatchTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ErrorWhenNoMatch(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ErrorWhenNoMatch", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_RegexMatchTimeout(
        &mut self,
        value: crate::System::Nullable_1<crate::System::TimeSpan>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RegexMatchTimeout", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonSelectSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonSelectSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
