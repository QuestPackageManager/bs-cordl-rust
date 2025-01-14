#[cfg(feature = "Zenject+InjectTypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectTypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub InjectMethods: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMethodInfo>,
        >,
    >,
    pub InjectMembers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMemberInfo>,
        >,
    >,
    pub InjectConstructor: quest_hook::libil2cpp::Gc<
        crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
    >,
    pub _BaseTypeInfo_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Zenject::InjectTypeInfo,
    >,
}
#[cfg(feature = "Zenject+InjectTypeInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::InjectTypeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "InjectTypeInfo";
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
#[cfg(feature = "Zenject+InjectTypeInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
        >,
        injectMethods: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::InjectTypeInfo_InjectMethodInfo,
                >,
            >,
        >,
        injectMembers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::InjectTypeInfo_InjectMemberInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, injectConstructor, injectMethods, injectMembers),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
        >,
        injectMethods: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::InjectTypeInfo_InjectMethodInfo,
                >,
            >,
        >,
        injectMembers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::InjectTypeInfo_InjectMemberInfo,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<
                        crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::InjectTypeInfo_InjectMethodInfo,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::InjectTypeInfo_InjectMemberInfo,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (_cordl_type, injectConstructor, injectMethods, injectMembers),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _get_AllInjectables_b__0(
        x: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Zenject::InjectTypeInfo_InjectMemberInfo,
                >),
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                1usize,
            >("<get_AllInjectables>b__0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<get_AllInjectables>b__0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo> = unsafe {
            method.invoke_unchecked((), (x))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _get_AllInjectables_b__1(
        x: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Zenject::InjectTypeInfo_InjectMethodInfo,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                    >,
                >,
                1usize,
            >("<get_AllInjectables>b__1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<get_AllInjectables>b__1", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        > = unsafe { method.invoke_unchecked((), (x)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_AllInjectables(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                    >,
                >,
                0usize,
            >("get_AllInjectables")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_AllInjectables", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_BaseTypeInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
                0usize,
            >("get_BaseTypeInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_BaseTypeInfo", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_BaseTypeInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_BaseTypeInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_BaseTypeInfo", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectTypeInfo_InjectConstructorInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Factory: quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod>,
    pub Parameters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
        >,
    >,
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::InjectTypeInfo_InjectConstructorInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "InjectTypeInfo/InjectConstructorInfo";
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
#[cfg(feature = "Zenject+InjectTypeInfo+InjectConstructorInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo_InjectConstructorInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        factory: quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (factory, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        factory: quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (factory, parameters))
        };
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Setter: quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
    pub Info: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::InjectTypeInfo_InjectMemberInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "InjectTypeInfo/InjectMemberInfo";
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
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMemberInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo_InjectMemberInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        setter: quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
        info: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (setter, info))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        setter: quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
        info: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
                    quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (setter, info))
        };
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Action: quest_hook::libil2cpp::Gc<crate::Zenject::ZenInjectMethod>,
    pub Parameters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
        >,
    >,
}
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::InjectTypeInfo_InjectMethodInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "InjectTypeInfo/InjectMethodInfo";
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
#[cfg(feature = "Zenject+InjectTypeInfo+InjectMethodInfo")]
impl std::ops::Deref for crate::Zenject::InjectTypeInfo_InjectMethodInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        action: quest_hook::libil2cpp::Gc<crate::Zenject::ZenInjectMethod>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (action, parameters, name))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::Zenject::ZenInjectMethod>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
            >,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Zenject::ZenInjectMethod>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
                        >,
                    >,
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
            method.invoke_unchecked(self, (action, parameters, name))
        };
        Ok(__cordl_ret.into())
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
