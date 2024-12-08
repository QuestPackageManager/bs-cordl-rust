#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionObject {
    __cordl_parent: crate::System::Object,
    pub _Creator_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
        *mut crate::System::Object,
    >,
    pub _Members_k__BackingField: *mut crate::System::Collections::Generic::IDictionary_2<
        *mut crate::System::String,
        *mut crate::Newtonsoft::Json::Utilities::ReflectionMember,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ReflectionObject =>
    "Newtonsoft.Json.Utilities"."ReflectionObject"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ReflectionObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
impl crate::Newtonsoft::Json::Utilities::ReflectionObject {
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::Newtonsoft::Json::Utilities::ReflectionObject___c__DisplayClass11_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject+__c__DisplayClass11_1")]
    pub type __c__DisplayClass11_1 = crate::Newtonsoft::Json::Utilities::ReflectionObject___c__DisplayClass11_1;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject+__c__DisplayClass11_2")]
    pub type __c__DisplayClass11_2 = crate::Newtonsoft::Json::Utilities::ReflectionObject___c__DisplayClass11_2;
    pub fn SetValue(
        &mut self,
        target: *mut crate::System::Object,
        member: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (target, member, value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Creator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_Creator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetType(
        &mut self,
        member: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetType", (member))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue(
        &mut self,
        target: *mut crate::System::Object,
        member: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetValue", (target, member))?;
        Ok(__cordl_ret)
    }
    pub fn get_Members(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Utilities::ReflectionMember,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IDictionary_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Utilities::ReflectionMember,
        > = __cordl_object.invoke("get_Members", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        creator: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (creator))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        creator: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (creator))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ReflectionObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
