#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldMultipleFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct FieldMultipleFilter {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JsonPath::PathFilter,
    pub Names: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldMultipleFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::JsonPath::FieldMultipleFilter =>
    "Newtonsoft.Json.Linq.JsonPath"."FieldMultipleFilter"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldMultipleFilter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JsonPath::FieldMultipleFilter {
    type Target = crate::Newtonsoft::Json::Linq::JsonPath::PathFilter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldMultipleFilter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Linq::JsonPath::FieldMultipleFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldMultipleFilter")]
impl crate::Newtonsoft::Json::Linq::JsonPath::FieldMultipleFilter {
    pub fn ExecuteFilter(
        &mut self,
        root: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
        current: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        >,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::JsonSelectSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        > = __cordl_object.invoke("ExecuteFilter", (root, current, settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        names: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (names))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        names: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (names))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+FieldMultipleFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JsonPath::FieldMultipleFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
