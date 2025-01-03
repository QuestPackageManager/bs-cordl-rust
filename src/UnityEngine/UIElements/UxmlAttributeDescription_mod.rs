#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlAttributeDescription {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_ObsoleteNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _type_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _typeNamespace_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _use_k__BackingField: crate::UnityEngine::UIElements::UxmlAttributeDescription_Use,
    pub _restriction_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlTypeRestriction,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UxmlAttributeDescription => "UnityEngine.UIElements"
    ."UxmlAttributeDescription"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlAttributeDescription {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlAttributeDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription")]
impl crate::UnityEngine::UIElements::UxmlAttributeDescription {
    #[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription+Use")]
    pub type Use = crate::UnityEngine::UIElements::UxmlAttributeDescription_Use;
    pub fn GetValueFromBag<T>(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
        converterFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<*mut quest_hook::libil2cpp::Il2CppString, T, T>,
        >,
        defaultValue: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("GetValueFromBag", (bag, cc, converterFunc, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetValueFromBag<T>(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
        converterFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<*mut quest_hook::libil2cpp::Il2CppString, T, T>,
        >,
        defaultValue: T,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "TryGetValueFromBag",
                (bag, cc, converterFunc, defaultValue, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValueFromBagAsString(
        &mut self,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
        value: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValueFromBagAsString", (bag, cc, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_obsoleteNames(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obsoleteNames", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_restriction(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UxmlTypeRestriction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_restriction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_type(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_type", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_typeNamespace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_typeNamespace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_use(
        &mut self,
        value: crate::UnityEngine::UIElements::UxmlAttributeDescription_Use,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_use", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UxmlAttributeDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription+Use")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UxmlAttributeDescription_Use {
    None = 0i32,
    Optional = 1i32,
    Prohibited = 2i32,
    Required = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAttributeDescription+Use")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UxmlAttributeDescription_Use => "UnityEngine.UIElements"
    ."UxmlAttributeDescription/Use"
);
