#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowSetBinderMember")]
#[repr(C)]
#[derive(Debug)]
pub struct NoThrowSetBinderMember {
    __cordl_parent: crate::System::Dynamic::SetMemberBinder,
    pub _innerBinder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetMemberBinder>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowSetBinderMember")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::NoThrowSetBinderMember => "Newtonsoft.Json.Utilities"
    ."NoThrowSetBinderMember"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowSetBinderMember")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::NoThrowSetBinderMember {
    type Target = crate::System::Dynamic::SetMemberBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowSetBinderMember")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::NoThrowSetBinderMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowSetBinderMember")]
impl crate::Newtonsoft::Json::Utilities::NoThrowSetBinderMember {
    pub fn FallbackSetMember(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        value: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
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
        > = __cordl_object
            .invoke("FallbackSetMember", (target, value, errorSuggestion))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        innerBinder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (innerBinder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        innerBinder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (innerBinder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+NoThrowSetBinderMember")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::NoThrowSetBinderMember {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
