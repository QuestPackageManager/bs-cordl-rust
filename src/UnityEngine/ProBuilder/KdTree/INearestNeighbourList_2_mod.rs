#[cfg(feature = "UnityEngine+ProBuilder+KdTree+INearestNeighbourList_2")]
#[repr(C)]
#[derive(Debug)]
pub struct INearestNeighbourList_2<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TDistance: std::marker::PhantomData<TDistance>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+INearestNeighbourList_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2 < TItem, TDistance > =>
    "UnityEngine.ProBuilder.KdTree"."INearestNeighbourList`2" < TItem, TDistance >
);
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+INearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<TItem, TDistance> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+INearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<TItem, TDistance> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+INearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<TItem, TDistance> {
    pub fn Add(
        &mut self,
        item: TItem,
        distance: TDistance,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Add", (item, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFurtherest(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TItem = __cordl_object.invoke("GetFurtherest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFurtherest(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TItem = __cordl_object.invoke("RemoveFurtherest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDistance: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxCapacity", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+INearestNeighbourList_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TDistance: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::KdTree::INearestNeighbourList_2<TItem, TDistance> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
