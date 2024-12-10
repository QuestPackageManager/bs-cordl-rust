#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionValueProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _memberInfo: *mut crate::System::Reflection::MemberInfo,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::ReflectionValueProvider =>
    "Newtonsoft.Json.Serialization"."ReflectionValueProvider"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::ReflectionValueProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::ReflectionValueProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
impl crate::Newtonsoft::Json::Serialization::ReflectionValueProvider {
    pub fn GetValue(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (memberInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (target, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (memberInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::ReflectionValueProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
impl AsRef<crate::Newtonsoft::Json::Serialization::IValueProvider>
for crate::Newtonsoft::Json::Serialization::ReflectionValueProvider {
    fn as_ref(&self) -> &crate::Newtonsoft::Json::Serialization::IValueProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+ReflectionValueProvider")]
impl AsMut<crate::Newtonsoft::Json::Serialization::IValueProvider>
for crate::Newtonsoft::Json::Serialization::ReflectionValueProvider {
    fn as_mut(&mut self) -> &mut crate::Newtonsoft::Json::Serialization::IValueProvider {
        unsafe { std::mem::transmute(self) }
    }
}
