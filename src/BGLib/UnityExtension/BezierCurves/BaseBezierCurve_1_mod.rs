#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseBezierCurve_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve,
    pub _bezierCurveData: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1 < T > =>
    "BGLib.UnityExtension.BezierCurves"."BaseBezierCurve`1" < T >
);
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<T> {
    type Target = crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<T> {
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
#[cfg(feature = "BGLib+UnityExtension+BezierCurves+BaseBezierCurve_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::BezierCurves::BaseBezierCurve_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
