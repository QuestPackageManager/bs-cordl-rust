#[cfg(feature = "Unity+Properties+PropertyBag_1")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyBag_1<TContainer: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _InstantiationKind_k__BackingField: crate::Unity::Properties::InstantiationKind,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::PropertyBag_1 < TContainer >
    => "Unity.Properties"."PropertyBag`1" < TContainer >
);
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::PropertyBag_1<TContainer> {
    pub fn Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<crate::Unity::Properties::ITypeVisitor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Accept", (visitor))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate(&mut self) -> quest_hook::libil2cpp::Result<TContainer>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContainer = __cordl_object.invoke("Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unity_Properties_IConstructor_TContainer__Instantiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TContainer>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TContainer = __cordl_object
            .invoke("Unity.Properties.IConstructor<TContainer>.Instantiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IConstructor_get_InstantiationKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::InstantiationKind>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Properties::InstantiationKind = __cordl_object
            .invoke("Unity.Properties.IConstructor.get_InstantiationKind", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_Internal_IPropertyBagRegister_Register(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unity.Properties.Internal.IPropertyBagRegister.Register", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstantiationKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::InstantiationKind>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Properties::InstantiationKind = __cordl_object
            .invoke("get_InstantiationKind", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IConstructor>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IConstructor>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IConstructor_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IConstructor_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IConstructor_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IConstructor_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag_1<TContainer>>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TContainer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::Internal::IPropertyBagRegister>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_ref(&self) -> &crate::Unity::Properties::Internal::IPropertyBagRegister {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+PropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::Internal::IPropertyBagRegister>
for crate::Unity::Properties::PropertyBag_1<TContainer> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::Internal::IPropertyBagRegister {
        unsafe { std::mem::transmute(self) }
    }
}
