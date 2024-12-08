#[cfg(feature = "System+Net+Http+Headers+RangeConditionHeaderValue")]
#[repr(C)]
#[derive(Debug)]
pub struct RangeConditionHeaderValue {
    __cordl_parent: crate::System::Object,
    pub _Date_k__BackingField: crate::System::Nullable_1<crate::System::DateTimeOffset>,
    pub _EntityTag_k__BackingField: *mut crate::System::Net::Http::Headers::EntityTagHeaderValue,
}
#[cfg(feature = "System+Net+Http+Headers+RangeConditionHeaderValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Http::Headers::RangeConditionHeaderValue => "System.Net.Http.Headers"
    ."RangeConditionHeaderValue"
);
#[cfg(feature = "System+Net+Http+Headers+RangeConditionHeaderValue")]
impl std::ops::Deref for crate::System::Net::Http::Headers::RangeConditionHeaderValue {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+RangeConditionHeaderValue")]
impl std::ops::DerefMut
for crate::System::Net::Http::Headers::RangeConditionHeaderValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+RangeConditionHeaderValue")]
impl crate::System::Net::Http::Headers::RangeConditionHeaderValue {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_DateTimeOffset0(
        date: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (date))?;
        Ok(__cordl_object)
    }
    pub fn New_EntityTagHeaderValue1(
        entityTag: *mut crate::System::Net::Http::Headers::EntityTagHeaderValue,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entityTag))?;
        Ok(__cordl_object)
    }
    pub fn System_ICloneable_Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.ICloneable.Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DateTimeOffset0(
        &mut self,
        date: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (date))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_EntityTagHeaderValue1(
        &mut self,
        entityTag: *mut crate::System::Net::Http::Headers::EntityTagHeaderValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entityTag))?;
        Ok(__cordl_ret)
    }
    pub fn get_Date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::System::DateTimeOffset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<crate::System::DateTimeOffset> = __cordl_object
            .invoke("get_Date", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EntityTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Http::Headers::EntityTagHeaderValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Http::Headers::EntityTagHeaderValue = __cordl_object
            .invoke("get_EntityTag", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Date(
        &mut self,
        value: crate::System::Nullable_1<crate::System::DateTimeOffset>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Date", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EntityTag(
        &mut self,
        value: *mut crate::System::Net::Http::Headers::EntityTagHeaderValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EntityTag", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Http+Headers+RangeConditionHeaderValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::RangeConditionHeaderValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
