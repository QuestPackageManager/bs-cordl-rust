#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonContainerContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContract,
    pub _itemContract: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonContract,
    >,
    pub _finalItemContract: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::JsonContract,
    >,
    pub _ItemConverter_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::JsonConverter,
    >,
    pub _ItemIsReference_k__BackingField: crate::System::Nullable_1<bool>,
    pub _ItemReferenceLoopHandling_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::ReferenceLoopHandling,
    >,
    pub _ItemTypeNameHandling_k__BackingField: crate::System::Nullable_1<
        crate::Newtonsoft::Json::TypeNameHandling,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "JsonContainerContract";
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    pub fn New(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (underlyingType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_FinalItemContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::JsonContract,
                >,
                0usize,
            >("get_FinalItemContract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "get_FinalItemContract",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemContract(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::JsonContract,
                >,
                0usize,
            >("get_ItemContract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "get_ItemContract", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
                0usize,
            >("get_ItemConverter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "get_ItemConverter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemIsReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Nullable_1<bool>,
                0usize,
            >("get_ItemIsReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "get_ItemIsReference",
                    0usize
                )
            });
        let __cordl_ret: crate::System::Nullable_1<bool> = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemReferenceLoopHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::ReferenceLoopHandling>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Nullable_1<
                    crate::Newtonsoft::Json::ReferenceLoopHandling,
                >,
                0usize,
            >("get_ItemReferenceLoopHandling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(),
                    "get_ItemReferenceLoopHandling", 0usize
                )
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::ReferenceLoopHandling,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ItemTypeNameHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::Newtonsoft::Json::TypeNameHandling>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::System::Nullable_1<crate::Newtonsoft::Json::TypeNameHandling>,
                0usize,
            >("get_ItemTypeNameHandling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "get_ItemTypeNameHandling",
                    0usize
                )
            });
        let __cordl_ret: crate::System::Nullable_1<
            crate::Newtonsoft::Json::TypeNameHandling,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemContract(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::JsonContract,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ItemContract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "set_ItemContract", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemConverter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ItemConverter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "set_ItemConverter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemIsReference(
        &mut self,
        value: crate::System::Nullable_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Nullable_1<bool>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ItemIsReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "set_ItemIsReference",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemReferenceLoopHandling(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::ReferenceLoopHandling>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Nullable_1<
                    crate::Newtonsoft::Json::ReferenceLoopHandling,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ItemReferenceLoopHandling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(),
                    "set_ItemReferenceLoopHandling", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ItemTypeNameHandling(
        &mut self,
        value: crate::System::Nullable_1<crate::Newtonsoft::Json::TypeNameHandling>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonContainerContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Nullable_1<crate::Newtonsoft::Json::TypeNameHandling>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ItemTypeNameHandling")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonContainerContract as
                    quest_hook::libil2cpp::Type > ::class(), "set_ItemTypeNameHandling",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonContainerContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonContainerContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
