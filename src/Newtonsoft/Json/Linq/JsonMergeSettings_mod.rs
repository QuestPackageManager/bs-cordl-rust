#[cfg(feature = "Newtonsoft+Json+Linq+JsonMergeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonMergeSettings {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _mergeArrayHandling: crate::Newtonsoft::Json::Linq::MergeArrayHandling,
    pub _mergeNullValueHandling: crate::Newtonsoft::Json::Linq::MergeNullValueHandling,
    pub _propertyNameComparison: crate::System::StringComparison,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonMergeSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JsonMergeSettings =>
    "Newtonsoft.Json.Linq"."JsonMergeSettings"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonMergeSettings")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonMergeSettings {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonMergeSettings")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonMergeSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonMergeSettings")]
impl crate::Newtonsoft::Json::Linq::JsonMergeSettings {
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
    pub fn get_MergeArrayHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Linq::MergeArrayHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::MergeArrayHandling = __cordl_object
            .invoke("get_MergeArrayHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MergeNullValueHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Linq::MergeNullValueHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::MergeNullValueHandling = __cordl_object
            .invoke("get_MergeNullValueHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PropertyNameComparison(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::StringComparison> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::StringComparison = __cordl_object
            .invoke("get_PropertyNameComparison", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MergeArrayHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::MergeArrayHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MergeArrayHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MergeNullValueHandling(
        &mut self,
        value: crate::Newtonsoft::Json::Linq::MergeNullValueHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MergeNullValueHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PropertyNameComparison(
        &mut self,
        value: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PropertyNameComparison", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonMergeSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonMergeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
