#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectTypeInfo_InjectConstructorInfo {
    __cordl_parent: crate::System::Object,
    pub Factory: *mut crate::Zenject::ZenFactoryMethod,
    pub Parameters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Zenject::InjectableInfo,
    >,
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectTypeInfo_InjectConstructorInfo =>
    "Zenject"."InjectTypeInfo/InjectConstructorInfo"
);
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo_InjectConstructorInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
impl std::ops::DerefMut for crate::Zenject::InjectTypeInfo_InjectConstructorInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
impl crate::Zenject::InjectTypeInfo_InjectConstructorInfo {
    pub fn New(
        factory: *mut crate::Zenject::ZenFactoryMethod,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectableInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (factory, parameters))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        factory: *mut crate::Zenject::ZenFactoryMethod,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectableInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (factory, parameters))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::InjectTypeInfo_InjectConstructorInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectTypeInfo_InjectMemberInfo {
    __cordl_parent: crate::System::Object,
    pub Setter: *mut crate::Zenject::ZenMemberSetterMethod,
    pub Info: *mut crate::Zenject::InjectableInfo,
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectTypeInfo_InjectMemberInfo =>
    "Zenject"."InjectTypeInfo/InjectMemberInfo"
);
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo_InjectMemberInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
impl std::ops::DerefMut for crate::Zenject::InjectTypeInfo_InjectMemberInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
impl crate::Zenject::InjectTypeInfo_InjectMemberInfo {
    pub fn New(
        setter: *mut crate::Zenject::ZenMemberSetterMethod,
        info: *mut crate::Zenject::InjectableInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (setter, info))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        setter: *mut crate::Zenject::ZenMemberSetterMethod,
        info: *mut crate::Zenject::InjectableInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (setter, info))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::InjectTypeInfo_InjectMemberInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectTypeInfo_InjectMethodInfo {
    __cordl_parent: crate::System::Object,
    pub Name: *mut crate::System::String,
    pub Action: *mut crate::Zenject::ZenInjectMethod,
    pub Parameters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Zenject::InjectableInfo,
    >,
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectTypeInfo_InjectMethodInfo =>
    "Zenject"."InjectTypeInfo/InjectMethodInfo"
);
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo_InjectMethodInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
impl std::ops::DerefMut for crate::Zenject::InjectTypeInfo_InjectMethodInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
impl crate::Zenject::InjectTypeInfo_InjectMethodInfo {
    pub fn New(
        action: *mut crate::Zenject::ZenInjectMethod,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectableInfo,
        >,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, parameters, name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        action: *mut crate::Zenject::ZenInjectMethod,
        parameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectableInfo,
        >,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (action, parameters, name))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::InjectTypeInfo_InjectMethodInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectTypeInfo {
    __cordl_parent: crate::System::Object,
    pub Type: *mut crate::System::Type,
    pub InjectMethods: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Zenject::InjectTypeInfo_InjectMethodInfo,
    >,
    pub InjectMembers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Zenject::InjectTypeInfo_InjectMemberInfo,
    >,
    pub InjectConstructor: *mut crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
    pub _BaseTypeInfo_k__BackingField: *mut crate::Zenject::InjectTypeInfo,
}
#[cfg(feature = "Zenject+InjectTypeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectTypeInfo => "Zenject"
    ."InjectTypeInfo"
);
#[cfg(feature = "Zenject+InjectTypeInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo")]
impl std::ops::DerefMut for crate::Zenject::InjectTypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo")]
impl crate::Zenject::InjectTypeInfo {
    #[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
    pub type InjectConstructorInfo = crate::Zenject::InjectTypeInfo_InjectConstructorInfo;
    #[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
    pub type InjectMemberInfo = crate::Zenject::InjectTypeInfo_InjectMemberInfo;
    #[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
    pub type InjectMethodInfo = crate::Zenject::InjectTypeInfo_InjectMethodInfo;
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        injectConstructor: *mut crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
        injectMethods: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectTypeInfo_InjectMethodInfo,
        >,
        injectMembers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectTypeInfo_InjectMemberInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, injectConstructor, injectMethods, injectMembers),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        injectConstructor: *mut crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
        injectMethods: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectTypeInfo_InjectMethodInfo,
        >,
        injectMembers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Zenject::InjectTypeInfo_InjectMemberInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_type, injectConstructor, injectMethods, injectMembers),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_AllInjectables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::InjectableInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Zenject::InjectableInfo,
        > = __cordl_object.invoke("get_AllInjectables", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BaseTypeInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::InjectTypeInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::InjectTypeInfo = __cordl_object
            .invoke("get_BaseTypeInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_BaseTypeInfo(
        &mut self,
        value: *mut crate::Zenject::InjectTypeInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BaseTypeInfo", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+InjectTypeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::InjectTypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
