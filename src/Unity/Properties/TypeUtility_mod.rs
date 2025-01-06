#[cfg(feature = "Unity+Properties+TypeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+TypeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility =>
    "Unity.Properties"."TypeUtility"
);
#[cfg(feature = "Unity+Properties+TypeUtility")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility")]
impl std::ops::DerefMut for crate::Unity::Properties::TypeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility")]
impl crate::Unity::Properties::TypeUtility {
    #[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
    pub type Cache_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Properties::TypeUtility_Cache_1<
        T,
    >;
    #[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
    type ITypeConstructor = crate::Unity::Properties::TypeUtility_ITypeConstructor;
    #[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
    type ITypeConstructor_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Properties::TypeUtility_ITypeConstructor_1<
        T,
    >;
    #[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
    pub type NonConstructable = crate::Unity::Properties::TypeUtility_NonConstructable;
    #[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
    pub type TypeConstructorVisitor = crate::Unity::Properties::TypeUtility_TypeConstructorVisitor;
    #[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
    pub type TypeConstructor_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Properties::TypeUtility_TypeConstructor_1<
        T,
    >;
    pub fn CanBeInstantiated_1<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanBeInstantiated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CanBeInstantiated_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanBeInstantiated", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCanBeInstantiated_TypeUtility_ITypeConstructor_1_0<T>(
        constructor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCanBeInstantiated", (constructor))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCanBeInstantiated_TypeUtility_ITypeConstructor_Type1(
        constructor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCanBeInstantiated", (constructor, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIsAssignableFrom(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        derivedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIsAssignableFrom", (_cordl_type, derivedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTypeConstructor_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTypeConstructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTypeConstructor_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::TypeUtility_ITypeConstructor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTypeConstructor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRootType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeConstructor_1<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeConstructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeConstructor_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::TypeUtility_ITypeConstructor>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::TypeUtility_ITypeConstructor,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeConstructor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeDisplayName_IReadOnlyList_1_ByRefMut1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
        argIndex: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeDisplayName", (_cordl_type, args, argIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeDisplayName_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeDisplayName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateArray_Type_i32_1<TArray>(
        derivedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<TArray>
    where
        TArray: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TArray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateArray", (derivedType, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateArray_i32_0<TArray>(
        count: i32,
    ) -> quest_hook::libil2cpp::Result<TArray>
    where
        TArray: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TArray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateArray", (count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_0<T>() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Type1<T>(
        derivedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (derivedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExplicitInstantiationMethod<T>(
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetExplicitInstantiationMethod", (constructor))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryInstantiateArray<TArray>(
        count: i32,
        instance: quest_hook::libil2cpp::ByRefMut<TArray>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TArray: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryInstantiateArray", (count, instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryInstantiate_ByRefMut0<T>(
        instance: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryInstantiate", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryInstantiate_Type_ByRefMut1<T>(
        derivedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryInstantiate", (derivedType, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Properties::TypeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeUtility_Cache_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility_Cache_1 < T > =>
    "Unity.Properties"."TypeUtility/Cache`1<T>" < T >
);
#[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::TypeUtility_Cache_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Unity::Properties::TypeUtility_Cache_1<T> {}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_ITypeConstructor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility_ITypeConstructor
    => "Unity.Properties"."TypeUtility/ITypeConstructor"
);
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility_ITypeConstructor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
impl std::ops::DerefMut for crate::Unity::Properties::TypeUtility_ITypeConstructor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
impl crate::Unity::Properties::TypeUtility_ITypeConstructor {
    pub fn Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_CanBeInstantiated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanBeInstantiated", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::TypeUtility_ITypeConstructor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_ITypeConstructor_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::TypeUtility_ITypeConstructor_1 < T > => "Unity.Properties"
    ."TypeUtility/ITypeConstructor`1" < T >
);
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
    pub fn Instantiate(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExplicitConstructor(
        &mut self,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExplicitConstructor", (constructor))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::TypeUtility_ITypeConstructor>
for crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
    fn as_ref(&self) -> &crate::Unity::Properties::TypeUtility_ITypeConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::TypeUtility_ITypeConstructor>
for crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::TypeUtility_ITypeConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_NonConstructable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility_NonConstructable
    => "Unity.Properties"."TypeUtility/NonConstructable"
);
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility_NonConstructable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl std::ops::DerefMut for crate::Unity::Properties::TypeUtility_NonConstructable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl crate::Unity::Properties::TypeUtility_NonConstructable {
    pub fn Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unity_Properties_TypeUtility_ITypeConstructor_get_CanBeInstantiated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Unity.Properties.TypeUtility.ITypeConstructor.get_CanBeInstantiated",
                (),
            )?;
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
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::TypeUtility_NonConstructable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl AsRef<crate::Unity::Properties::TypeUtility_ITypeConstructor>
for crate::Unity::Properties::TypeUtility_NonConstructable {
    fn as_ref(&self) -> &crate::Unity::Properties::TypeUtility_ITypeConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl AsMut<crate::Unity::Properties::TypeUtility_ITypeConstructor>
for crate::Unity::Properties::TypeUtility_NonConstructable {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::TypeUtility_ITypeConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_TypeConstructorVisitor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub TypeConstructor: quest_hook::libil2cpp::Gc<
        crate::Unity::Properties::TypeUtility_ITypeConstructor,
    >,
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::TypeUtility_TypeConstructorVisitor => "Unity.Properties"
    ."TypeUtility/TypeConstructorVisitor"
);
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl std::ops::DerefMut
for crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Visit<TContainer>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Visit", ())?;
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
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl AsRef<crate::Unity::Properties::ITypeVisitor>
for crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    fn as_ref(&self) -> &crate::Unity::Properties::ITypeVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl AsMut<crate::Unity::Properties::ITypeVisitor>
for crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::ITypeVisitor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_TypeConstructor_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ExplicitConstructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    pub m_ImplicitConstructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    pub m_OverrideConstructor: quest_hook::libil2cpp::Gc<
        crate::Unity::Properties::IConstructor_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility_TypeConstructor_1
    < T > => "Unity.Properties"."TypeUtility/TypeConstructor`1" < T >
);
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    pub fn CreateClassInstance() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateClassInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScriptableObjectInstance() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateScriptableObjectInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateValueTypeInstance() -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateValueTypeInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetExplicitConstructor(
        &mut self,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExplicitConstructor", (constructor))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetImplicitConstructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetImplicitConstructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_TypeUtility_ITypeConstructor_Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("Unity.Properties.TypeUtility.ITypeConstructor.Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_TypeUtility_ITypeConstructor_T__Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("Unity.Properties.TypeUtility.ITypeConstructor<T>.Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_TypeUtility_ITypeConstructor_get_CanBeInstantiated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Unity.Properties.TypeUtility.ITypeConstructor.get_CanBeInstantiated",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::TypeUtility_ITypeConstructor>
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    fn as_ref(&self) -> &crate::Unity::Properties::TypeUtility_ITypeConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::TypeUtility_ITypeConstructor>
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::TypeUtility_ITypeConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>>
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    fn as_ref(&self) -> &crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T>>
for crate::Unity::Properties::TypeUtility_TypeConstructor_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::TypeUtility_ITypeConstructor_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
