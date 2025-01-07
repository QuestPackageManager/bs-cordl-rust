#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowGetBinderMember")]
#[repr(C)]
#[derive(Debug)]
pub struct NoThrowGetBinderMember {
    __cordl_parent: crate::System::Dynamic::GetMemberBinder,
    pub _innerBinder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetMemberBinder>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowGetBinderMember")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::NoThrowGetBinderMember {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "NoThrowGetBinderMember";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowGetBinderMember")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::NoThrowGetBinderMember {
    type Target = crate::System::Dynamic::GetMemberBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowGetBinderMember")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::NoThrowGetBinderMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowGetBinderMember")]
impl crate::Newtonsoft::Json::Utilities::NoThrowGetBinderMember {
    pub fn FallbackGetMember(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        errorSuggestion: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("FallbackGetMember", (target, errorSuggestion))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        innerBinder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerBinder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        innerBinder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerBinder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowGetBinderMember")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::NoThrowGetBinderMember {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
