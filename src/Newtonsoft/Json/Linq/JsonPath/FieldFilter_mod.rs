#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct FieldFilter {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    pub Name: *mut crate::System::String,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JsonPath::FieldFilter =>
    "Newtonsoft.Json.Linq.JsonPath"."FieldFilter"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonPath::FieldFilter {
    type Target = crate::Newtonsoft::Json::Linq::JsonPath::PathFilter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JsonPath::FieldFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
impl crate::Newtonsoft::Json::Linq::JsonPath::FieldFilter {
    #[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter+_ExecuteFilter_d__2")]
    pub type _ExecuteFilter_d__2 = crate::Newtonsoft::Json::Linq::JsonPath::FieldFilter__ExecuteFilter_d__2;
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteFilter(
        &mut self,
        root: *mut crate::Newtonsoft::Json::Linq::JToken,
        current: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonSelectSettings,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object.invoke("ExecuteFilter", (root, current, settings))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::FieldFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
