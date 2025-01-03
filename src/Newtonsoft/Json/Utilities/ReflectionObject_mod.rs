#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionObject {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Creator_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub _Members_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IDictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::Newtonsoft::Json::Utilities::ReflectionMember,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ReflectionObject =>
    "Newtonsoft.Json.Utilities"."ReflectionObject"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionObject")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionObject {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn Create_Il2CppArray0(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::ReflectionObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::ReflectionObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (t, memberNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_MethodBase_Il2CppArray1(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        creator: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
        memberNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::ReflectionObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::ReflectionObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (t, creator, memberNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetType(
        &mut self,
        member: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetType", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (target, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        creator: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (creator))?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        member: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (target, member, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        creator: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (creator))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Creator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_Creator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Members(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::Newtonsoft::Json::Utilities::ReflectionMember,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IDictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::Newtonsoft::Json::Utilities::ReflectionMember,
            >,
        > = __cordl_object.invoke("get_Members", ())?;
        Ok(__cordl_ret.into())
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
