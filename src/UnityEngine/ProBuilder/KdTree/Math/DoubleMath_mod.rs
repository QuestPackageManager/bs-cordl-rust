#[cfg(feature = "UnityEngine+ProBuilder+KdTree+Math+DoubleMath")]
#[repr(C)]
#[derive(Debug)]
pub struct DoubleMath {
    __cordl_parent: crate::UnityEngine::ProBuilder::KdTree::Math::TypeMath_1<f64>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+Math+DoubleMath")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::KdTree::Math::DoubleMath =>
    "UnityEngine.ProBuilder.KdTree.Math"."DoubleMath"
);
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+Math+DoubleMath")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::KdTree::Math::DoubleMath {
    type Target = crate::UnityEngine::ProBuilder::KdTree::Math::TypeMath_1<f64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+Math+DoubleMath")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::KdTree::Math::DoubleMath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+Math+DoubleMath")]
impl crate::UnityEngine::ProBuilder::KdTree::Math::DoubleMath {
    pub fn get_NegativeInfinity(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_NegativeInfinity", ())?;
        Ok(__cordl_ret)
    }
    pub fn AreEqual(&mut self, a: f64, b: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AreEqual", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn get_Zero(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_Zero", ())?;
        Ok(__cordl_ret)
    }
    pub fn DistanceSquaredBetweenPoints(
        &mut self,
        a: *mut quest_hook::libil2cpp::Il2CppArray<f64>,
        b: *mut quest_hook::libil2cpp::Il2CppArray<f64>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("DistanceSquaredBetweenPoints", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn Add(&mut self, a: f64, b: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("Add", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn get_MinValue(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_MinValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxValue(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_MaxValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn Subtract(&mut self, a: f64, b: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("Subtract", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn Multiply(&mut self, a: f64, b: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("Multiply", (a, b))?;
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
    pub fn get_PositiveInfinity(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_PositiveInfinity", ())?;
        Ok(__cordl_ret)
    }
    pub fn Compare(&mut self, a: f64, b: f64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (a, b))?;
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
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+Math+DoubleMath")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::KdTree::Math::DoubleMath {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
