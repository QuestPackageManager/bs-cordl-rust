#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionMember")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionMember {
    __cordl_parent: crate::System::Object,
    pub _MemberType_k__BackingField: *mut crate::System::Type,
    pub _Getter_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _Setter_k__BackingField: *mut crate::System::Action_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionMember")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ReflectionMember =>
    "Newtonsoft.Json.Utilities"."ReflectionMember"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionMember")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionMember {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionMember")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ReflectionMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionMember")]
impl crate::Newtonsoft::Json::Utilities::ReflectionMember {
    pub fn get_Setter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_Setter", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Setter(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Setter", (value))?;
        Ok(__cordl_ret)
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
    pub fn set_Getter(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Getter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_MemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_MemberType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MemberType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Getter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_Getter", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionMember")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ReflectionMember {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
