#[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
#[repr(C)]
#[derive(Debug, Clone)]
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
    pub fn get_CanBeInstantiated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanBeInstantiated", ())?;
        Ok(__cordl_ret)
    }
    pub fn Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Instantiate", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
        Ok(__cordl_ret)
    }
    pub fn SetExplicitConstructor(
        &mut self,
        constructor: *mut crate::System::Func_1<T>,
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
        Ok(__cordl_ret)
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
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_NonConstructable {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility_NonConstructable
    => "Unity.Properties"."TypeUtility/NonConstructable"
);
#[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility_NonConstructable {
    type Target = crate::System::Object;
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
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Instantiate", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_TypeConstructorVisitor {
    __cordl_parent: crate::System::Object,
    pub TypeConstructor: *mut crate::Unity::Properties::TypeUtility_ITypeConstructor,
}
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::TypeUtility_TypeConstructorVisitor => "Unity.Properties"
    ."TypeUtility/TypeConstructorVisitor"
);
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility_TypeConstructorVisitor {
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility_TypeConstructor_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub m_ExplicitConstructor: *mut crate::System::Func_1<T>,
    pub m_ImplicitConstructor: *mut crate::System::Func_1<T>,
    pub m_OverrideConstructor: *mut crate::Unity::Properties::IConstructor_1<T>,
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
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetExplicitConstructor(
        &mut self,
        constructor: *mut crate::System::Func_1<T>,
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
        Ok(__cordl_ret)
    }
    pub fn Unity_Properties_TypeUtility_ITypeConstructor_Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Unity.Properties.TypeUtility.ITypeConstructor.Instantiate", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "Unity+Properties+TypeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Properties+TypeUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::TypeUtility =>
    "Unity.Properties"."TypeUtility"
);
#[cfg(feature = "Unity+Properties+TypeUtility")]
impl std::ops::Deref for crate::Unity::Properties::TypeUtility {
    type Target = crate::System::Object;
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
    #[cfg(feature = "Unity+Properties+TypeUtility+__c")]
    pub type __c = crate::Unity::Properties::TypeUtility___c;
    #[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructorVisitor")]
    pub type TypeConstructorVisitor = crate::Unity::Properties::TypeUtility_TypeConstructorVisitor;
    #[cfg(feature = "Unity+Properties+TypeUtility+TypeConstructor_1")]
    pub type TypeConstructor_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Properties::TypeUtility_TypeConstructor_1<
        T,
    >;
    #[cfg(feature = "Unity+Properties+TypeUtility+NonConstructable")]
    pub type NonConstructable = crate::Unity::Properties::TypeUtility_NonConstructable;
    #[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor_1")]
    type ITypeConstructor_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Properties::TypeUtility_ITypeConstructor_1<
        T,
    >;
    #[cfg(feature = "Unity+Properties+TypeUtility+Cache_1")]
    pub type Cache_1<T: quest_hook::libil2cpp::Type> = crate::Unity::Properties::TypeUtility_Cache_1<
        T,
    >;
    #[cfg(feature = "Unity+Properties+TypeUtility+ITypeConstructor")]
    type ITypeConstructor = crate::Unity::Properties::TypeUtility_ITypeConstructor;
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
