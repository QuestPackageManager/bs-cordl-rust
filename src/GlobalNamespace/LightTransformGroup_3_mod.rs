#[cfg(feature = "LightTransformGroup_3")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTransformGroup_3<
    TX: quest_hook::libil2cpp::Type,
    TY: quest_hook::libil2cpp::Type,
    TZ: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::LightGroupSubsystem,
    pub _mirrorX: bool,
    pub _mirrorY: bool,
    pub _mirrorZ: bool,
    pub _xTransforms: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Transform,
    >,
    pub _yTransforms: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Transform,
    >,
    pub _zTransforms: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Transform,
    >,
    __cordl_phantom_TX: std::marker::PhantomData<TX>,
    __cordl_phantom_TY: std::marker::PhantomData<TY>,
    __cordl_phantom_TZ: std::marker::PhantomData<TZ>,
}
#[cfg(feature = "LightTransformGroup_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightTransformGroup_3 < TX, TY,
    TZ > => ""."LightTransformGroup`3" < TX, TY, TZ >
);
#[cfg(feature = "LightTransformGroup_3")]
impl<
    TX: quest_hook::libil2cpp::Type,
    TY: quest_hook::libil2cpp::Type,
    TZ: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::GlobalNamespace::LightTransformGroup_3<TX, TY, TZ> {
    type Target = crate::GlobalNamespace::LightGroupSubsystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTransformGroup_3")]
impl<
    TX: quest_hook::libil2cpp::Type,
    TY: quest_hook::libil2cpp::Type,
    TZ: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::GlobalNamespace::LightTransformGroup_3<TX, TY, TZ> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTransformGroup_3")]
impl<
    TX: quest_hook::libil2cpp::Type,
    TY: quest_hook::libil2cpp::Type,
    TZ: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::LightTransformGroup_3<TX, TY, TZ> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mirrorX(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mirrorX", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mirrorY(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mirrorY", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mirrorZ(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mirrorZ", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_xTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Transform,
        >,
    >
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Transform,
        > = __cordl_object.invoke("get_xTransforms", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_yTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Transform,
        >,
    >
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Transform,
        > = __cordl_object.invoke("get_yTransforms", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_zTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Transform,
        >,
    >
    where
        TX: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TY: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TZ: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Transform,
        > = __cordl_object.invoke("get_zTransforms", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightTransformGroup_3")]
impl<
    TX: quest_hook::libil2cpp::Type,
    TY: quest_hook::libil2cpp::Type,
    TZ: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightTransformGroup_3<TX, TY, TZ> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
