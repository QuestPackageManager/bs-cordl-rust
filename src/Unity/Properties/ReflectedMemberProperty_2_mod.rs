#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectedMemberProperty_2_GetClassValueAction<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction < TContainer,
    TValue > => "Unity.Properties"."ReflectedMemberProperty`2/GetClassValueAction" <
    TContainer, TValue >
);
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction<
    TContainer,
    TValue,
> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction<
    TContainer,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction<
    TContainer,
    TValue,
> {
    pub fn Invoke(
        &mut self,
        container: TContainer,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Invoke", (container))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction<
    TContainer,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectedMemberProperty_2_GetStructValueAction<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction < TContainer,
    TValue > => "Unity.Properties"."ReflectedMemberProperty`2/GetStructValueAction" <
    TContainer, TValue >
);
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction<
    TContainer,
    TValue,
> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction<
    TContainer,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction<
    TContainer,
    TValue,
> {
    pub fn Invoke(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("Invoke", (container))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction<
    TContainer,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectedMemberProperty_2<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::Property_2<TContainer, TValue>,
    pub m_Info: *mut crate::Unity::Properties::IMemberInfo,
    pub m_IsStructContainerType: bool,
    pub m_GetStructValueAction: *mut crate::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction<
        TContainer,
        TValue,
    >,
    pub m_SetStructValueAction: *mut crate::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction<
        TContainer,
        TValue,
    >,
    pub m_GetClassValueAction: *mut crate::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction<
        TContainer,
        TValue,
    >,
    pub m_SetClassValueAction: *mut crate::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction<
        TContainer,
        TValue,
    >,
    pub _Name_k__BackingField: *mut crate::System::String,
    pub _IsReadOnly_k__BackingField: bool,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::ReflectedMemberProperty_2 <
    TContainer, TValue > => "Unity.Properties"."ReflectedMemberProperty`2" < TContainer,
    TValue >
);
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::ReflectedMemberProperty_2<TContainer, TValue> {
    type Target = crate::Unity::Properties::Property_2<TContainer, TValue>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::ReflectedMemberProperty_2<TContainer, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::ReflectedMemberProperty_2<TContainer, TValue> {
    #[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetClassValueAction")]
    pub type GetClassValueAction = crate::Unity::Properties::ReflectedMemberProperty_2_GetClassValueAction<
        TContainer,
        TValue,
    >;
    #[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+GetStructValueAction")]
    pub type GetStructValueAction = crate::Unity::Properties::ReflectedMemberProperty_2_GetStructValueAction<
        TContainer,
        TValue,
    >;
    #[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
    pub type SetClassValueAction = crate::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction<
        TContainer,
        TValue,
    >;
    #[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
    pub type SetStructValueAction = crate::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction<
        TContainer,
        TValue,
    >;
    pub fn New(
        info: *mut crate::Unity::Properties::IMemberInfo,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        info: *mut crate::Unity::Properties::IMemberInfo,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ReflectedMemberProperty_2<TContainer, TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectedMemberProperty_2_SetClassValueAction<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction < TContainer,
    TValue > => "Unity.Properties"."ReflectedMemberProperty`2/SetClassValueAction" <
    TContainer, TValue >
);
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction<
    TContainer,
    TValue,
> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction<
    TContainer,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction<
    TContainer,
    TValue,
> {
    pub fn Invoke(
        &mut self,
        container: TContainer,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (container, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetClassValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ReflectedMemberProperty_2_SetClassValueAction<
    TContainer,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectedMemberProperty_2_SetStructValueAction<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction < TContainer,
    TValue > => "Unity.Properties"."ReflectedMemberProperty`2/SetStructValueAction" <
    TContainer, TValue >
);
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction<
    TContainer,
    TValue,
> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction<
    TContainer,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction<
    TContainer,
    TValue,
> {
    pub fn Invoke(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (container, value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Properties+ReflectedMemberProperty_2+SetStructValueAction")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::ReflectedMemberProperty_2_SetStructValueAction<
    TContainer,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
