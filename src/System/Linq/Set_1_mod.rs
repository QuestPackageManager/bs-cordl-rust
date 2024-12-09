#[cfg(feature = "System+Linq+Set_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Set_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub buckets: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub slots: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Linq::Set_1_Slot<TElement>,
    >,
    pub count: i32,
    pub freeList: i32,
    pub comparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
        TElement,
    >,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+Set_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Set_1 < TElement > =>
    "System.Linq"."Set`1" < TElement >
);
#[cfg(feature = "System+Linq+Set_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::Set_1<TElement> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Set_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::Set_1<TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Set_1")]
impl<TElement: quest_hook::libil2cpp::Type> crate::System::Linq::Set_1<TElement> {
    #[cfg(feature = "System+Linq+Set_1+Slot")]
    pub type Slot = crate::System::Linq::Set_1_Slot<TElement>;
    pub fn Add(&mut self, value: TElement) -> quest_hook::libil2cpp::Result<bool>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Add", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Find(
        &mut self,
        value: TElement,
        add: bool,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Find", (value, add))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetHashCode(
        &mut self,
        value: TElement,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("InternalGetHashCode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        comparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<TElement>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (comparer))?;
        Ok(__cordl_object)
    }
    pub fn Remove(&mut self, value: TElement) -> quest_hook::libil2cpp::Result<bool>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Resize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        comparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<TElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (comparer))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Linq+Set_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Set_1<TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+Set_1+Slot")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Set_1_Slot<TElement: quest_hook::libil2cpp::Type> {
    pub hashCode: i32,
    pub value: TElement,
    pub next: i32,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+Set_1+Slot")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Set_1_Slot < TElement > =>
    "System.Linq"."Set`1/Slot<TElement>" < TElement >
);
#[cfg(feature = "System+Linq+Set_1+Slot")]
unsafe impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Set_1_Slot<TElement> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Linq+Set_1+Slot")]
impl<TElement: quest_hook::libil2cpp::Type> crate::System::Linq::Set_1_Slot<TElement> {}
