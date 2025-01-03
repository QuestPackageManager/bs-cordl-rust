#[cfg(feature = "Zenject+BindingUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct BindingUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+BindingUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindingUtil => "Zenject"."BindingUtil"
);
#[cfg(feature = "Zenject+BindingUtil")]
impl std::ops::Deref for crate::Zenject::BindingUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindingUtil")]
impl std::ops::DerefMut for crate::Zenject::BindingUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindingUtil")]
impl crate::Zenject::BindingUtil {
    pub fn AssertConcreteTypeListIsNotEmpty(
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertConcreteTypeListIsNotEmpty", (concreteTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertDerivesFromUnityObject_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertDerivesFromUnityObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertDerivesFromUnityObject_IEnumerable_1_0(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertDerivesFromUnityObject", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertDerivesFromUnityObject_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertDerivesFromUnityObject", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertInstanceDerivesFromOrEqual_IEnumerable_1_0(
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parentTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertInstanceDerivesFromOrEqual", (instance, parentTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertInstanceDerivesFromOrEqual_Type1(
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertInstanceDerivesFromOrEqual", (instance, baseType))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsComponent_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsComponent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsComponent_IEnumerable_1_0(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsComponent", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsComponent_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsComponent", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsDerivedFromType(
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsDerivedFromType", (concreteType, parentType))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsDerivedFromTypes_IEnumerable_1_1(
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
        parentTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsDerivedFromTypes", (concreteTypes, parentTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsDerivedFromTypes_IEnumerable_1_InvalidBindResponses0(
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
        parentTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
        invalidBindResponse: crate::Zenject::InvalidBindResponses,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertIsDerivedFromTypes",
                (concreteTypes, parentTypes, invalidBindResponse),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsDerivedFromTypes_Type2(
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        parentTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsDerivedFromTypes", (concreteType, parentTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsInterfaceOrComponent_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsInterfaceOrComponent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsInterfaceOrComponent_IEnumerable_1_0(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsInterfaceOrComponent", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsInterfaceOrComponent_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsInterfaceOrComponent", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsInterfaceOrScriptableObject_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsInterfaceOrScriptableObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsInterfaceOrScriptableObject_IEnumerable_1_0(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsInterfaceOrScriptableObject", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsInterfaceOrScriptableObject_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsInterfaceOrScriptableObject", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsNotAbstract_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsNotAbstract", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsNotAbstract_IEnumerable_1_0(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsNotAbstract", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsNotAbstract_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsNotAbstract", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsNotComponent_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsNotComponent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsNotComponent_IEnumerable_1_0(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsNotComponent", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsNotComponent_Type2(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsNotComponent", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsValidGameObject(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsValidGameObject", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsValidPrefab(
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsValidPrefab", (prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertIsValidResourcePath(
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertIsValidResourcePath", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTypesAreNotAbstract(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertTypesAreNotAbstract", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTypesAreNotComponents(
        types: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AssertTypesAreNotComponents", (types))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCachedProvider(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCachedProvider", (creator))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+BindingUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::BindingUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
