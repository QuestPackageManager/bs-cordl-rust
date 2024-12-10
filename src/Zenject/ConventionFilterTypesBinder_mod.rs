#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConventionFilterTypesBinder {
    __cordl_parent: crate::Zenject::ConventionAssemblySelectionBinder,
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConventionFilterTypesBinder =>
    "Zenject"."ConventionFilterTypesBinder"
);
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl std::ops::Deref for crate::Zenject::ConventionFilterTypesBinder {
    type Target = crate::Zenject::ConventionAssemblySelectionBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl std::ops::DerefMut for crate::Zenject::ConventionFilterTypesBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl crate::Zenject::ConventionFilterTypesBinder {
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass13_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass13_1")]
    pub type __c__DisplayClass13_1 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass13_1;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass14_0")]
    pub type __c__DisplayClass14_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass14_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass15_0")]
    pub type __c__DisplayClass15_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass15_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass18_0")]
    pub type __c__DisplayClass18_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass18_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass2_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass4_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass6_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass8_0;
    #[cfg(feature = "Zenject+ConventionFilterTypesBinder+__c__DisplayClass9_0_1")]
    pub type __c__DisplayClass9_0_1<T: quest_hook::libil2cpp::Type> = crate::Zenject::ConventionFilterTypesBinder___c__DisplayClass9_0_1<
        T,
    >;
    pub fn DerivingFromOrEqual_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("DerivingFromOrEqual", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivingFromOrEqual_Type1(
        &mut self,
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("DerivingFromOrEqual", (parentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivingFrom_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("DerivingFrom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivingFrom_Type1(
        &mut self,
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("DerivingFrom", (parentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InNamespace(
        &mut self,
        ns: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("InNamespace", (ns))?;
        Ok(__cordl_ret.into())
    }
    pub fn InNamespaces_IEnumerable_1_1(
        &mut self,
        namespaces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("InNamespaces", (namespaces))?;
        Ok(__cordl_ret.into())
    }
    pub fn InNamespaces_Il2CppArray0(
        &mut self,
        namespaces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("InNamespaces", (namespaces))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchingRegex_Il2CppString0(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("MatchingRegex", (pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchingRegex_Il2CppString_RegexOptions1(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("MatchingRegex", (pattern, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchingRegex_Regex2(
        &mut self,
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("MatchingRegex", (regex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::ConventionBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn Where(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut crate::System::Type, bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("Where", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithAttributeWhere<T>(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<crate::System::Func_2<T, bool>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithAttributeWhere", (predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithAttribute_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithAttribute_Type1(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithAttribute", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithPrefix(
        &mut self,
        prefix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithPrefix", (prefix))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithSuffix(
        &mut self,
        suffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithSuffix", (suffix))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithoutAttribute_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithoutAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WithoutAttribute_Type1(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConventionFilterTypesBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConventionFilterTypesBinder,
        > = __cordl_object.invoke("WithoutAttribute", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::ConventionBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ConventionFilterTypesBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConventionFilterTypesBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
