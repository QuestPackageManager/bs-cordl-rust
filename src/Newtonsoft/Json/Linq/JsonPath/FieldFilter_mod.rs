#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct FieldFilter {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    pub fn ExecuteFilter(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        current: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        > = __cordl_object.invoke("ExecuteFilter", (root, current, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret.into())
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
