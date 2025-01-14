#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ImmutableCollectionsUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "ImmutableCollectionsUtils";
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
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    pub const ImmutableArrayGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableArray`1";
    pub const ImmutableArrayTypeName: &'static str = "System.Collections.Immutable.ImmutableArray";
    pub const ImmutableDictionaryGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableDictionary`2";
    pub const ImmutableDictionaryGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableDictionary`2";
    pub const ImmutableDictionaryTypeName: &'static str = "System.Collections.Immutable.ImmutableDictionary";
    pub const ImmutableHashSetGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableHashSet`1";
    pub const ImmutableHashSetTypeName: &'static str = "System.Collections.Immutable.ImmutableHashSet";
    pub const ImmutableListGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableList`1";
    pub const ImmutableListGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableList`1";
    pub const ImmutableListTypeName: &'static str = "System.Collections.Immutable.ImmutableList";
    pub const ImmutableQueueGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableQueue`1";
    pub const ImmutableQueueGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableQueue`1";
    pub const ImmutableQueueTypeName: &'static str = "System.Collections.Immutable.ImmutableQueue";
    pub const ImmutableSetGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableSet`1";
    pub const ImmutableSortedDictionaryGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedDictionary`2";
    pub const ImmutableSortedDictionaryTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedDictionary";
    pub const ImmutableSortedSetGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedSet`1";
    pub const ImmutableSortedSetTypeName: &'static str = "System.Collections.Immutable.ImmutableSortedSet";
    pub const ImmutableStackGenericInterfaceTypeName: &'static str = "System.Collections.Immutable.IImmutableStack`1";
    pub const ImmutableStackGenericTypeName: &'static str = "System.Collections.Immutable.ImmutableStack`1";
    pub const ImmutableStackTypeName: &'static str = "System.Collections.Immutable.ImmutableStack";
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
    )]
    pub type ImmutableCollectionTypeInfo = crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo;
    pub fn TryBuildImmutableForArrayContract(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        collectionItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        createdType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
        parameterizedCreator: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                    >,
                ),
                bool,
                4usize,
            >("TryBuildImmutableForArrayContract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryBuildImmutableForArrayContract", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        underlyingType,
                        collectionItemType,
                        createdType,
                        parameterizedCreator,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryBuildImmutableForDictionaryContract(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        keyItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        valueItemType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        createdType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
        parameterizedCreator: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                    >,
                ),
                bool,
                5usize,
            >("TryBuildImmutableForDictionaryContract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryBuildImmutableForDictionaryContract", 5usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        underlyingType,
                        keyItemType,
                        valueItemType,
                        createdType,
                        parameterizedCreator,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ContractTypeName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _CreatedTypeName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _BuilderTypeName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "ImmutableCollectionsUtils/ImmutableCollectionTypeInfo";
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
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl std::ops::Deref
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    pub fn New(
        contractTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        createdTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        builderTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (contractTypeName, createdTypeName, builderTypeName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        contractTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        createdTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        builderTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (contractTypeName, createdTypeName, builderTypeName),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BuilderTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_BuilderTypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_BuilderTypeName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_ContractTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_ContractTypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ContractTypeName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_CreatedTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_CreatedTypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_CreatedTypeName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_BuilderTypeName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_BuilderTypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_BuilderTypeName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ContractTypeName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ContractTypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_ContractTypeName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_CreatedTypeName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_CreatedTypeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_CreatedTypeName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Utilities+ImmutableCollectionsUtils+ImmutableCollectionTypeInfo"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ImmutableCollectionsUtils_ImmutableCollectionTypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
