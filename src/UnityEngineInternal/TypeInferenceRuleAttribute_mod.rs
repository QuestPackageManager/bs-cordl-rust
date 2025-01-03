#[cfg(feature = "UnityEngineInternal+TypeInferenceRuleAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeInferenceRuleAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _rule: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "UnityEngineInternal+TypeInferenceRuleAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::TypeInferenceRuleAttribute
    => "UnityEngineInternal"."TypeInferenceRuleAttribute"
);
#[cfg(feature = "UnityEngineInternal+TypeInferenceRuleAttribute")]
impl std::ops::Deref for crate::UnityEngineInternal::TypeInferenceRuleAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+TypeInferenceRuleAttribute")]
impl std::ops::DerefMut for crate::UnityEngineInternal::TypeInferenceRuleAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+TypeInferenceRuleAttribute")]
impl crate::UnityEngineInternal::TypeInferenceRuleAttribute {
    pub fn New_Il2CppString1(
        rule: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rule))?;
        Ok(__cordl_object.into())
    }
    pub fn New_TypeInferenceRules0(
        rule: crate::UnityEngineInternal::TypeInferenceRules,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rule))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        rule: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rule))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TypeInferenceRules0(
        &mut self,
        rule: crate::UnityEngineInternal::TypeInferenceRules,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rule))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngineInternal+TypeInferenceRuleAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngineInternal::TypeInferenceRuleAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
