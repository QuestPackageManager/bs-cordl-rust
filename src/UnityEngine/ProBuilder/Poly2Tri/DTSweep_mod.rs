#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
#[repr(C)]
#[derive(Debug)]
pub struct DTSweep {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.Poly2Tri";
    const CLASS_NAME: &'static str = "DTSweep";
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
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    pub const PI_3div4: f64 = 2.356194490192345f64;
    pub const PI_div2: f64 = 1.5707963267948966f64;
    pub fn BasinAngle(
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                >),
                f64,
                1usize,
            >("BasinAngle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BasinAngle", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn EdgeEvent_DTSweepConstraint_AdvancingFrontNode0(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("EdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EdgeEvent_TriangulationPoint_TriangulationPoint_DelaunayTriangle_TriangulationPoint1(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        ep: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        eq: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        triangle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        point: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("EdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EdgeEvent", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, ep, eq, triangle, point))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Fill(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Fill")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Fill", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillAdvancingFront(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        n: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("FillAdvancingFront")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillAdvancingFront", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, n))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillBasin(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("FillBasin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillBasin", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillBasinReq(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("FillBasinReq")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillBasinReq", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLeftAboveEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillLeftAboveEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillLeftAboveEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLeftBelowEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillLeftBelowEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillLeftBelowEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLeftConcaveEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillLeftConcaveEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillLeftConcaveEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillLeftConvexEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillLeftConvexEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillLeftConvexEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillRightAboveEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillRightAboveEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillRightAboveEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillRightBelowEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillRightBelowEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillRightBelowEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillRightConcaveEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillRightConcaveEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillRightConcaveEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillRightConvexEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        edge: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepConstraint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("FillRightConvexEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FillRightConvexEdgeEvent", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, edge, node))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FinalizationConvexHull(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FinalizationConvexHull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FinalizationConvexHull", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FinalizationPolygon(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FinalizationPolygon")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FinalizationPolygon", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlipEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        ep: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        eq: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("FlipEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FlipEdgeEvent", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, ep, eq, t, p))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FlipScanEdgeEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        ep: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        eq: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        flipTriangle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("FlipScanEdgeEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FlipScanEdgeEvent", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, ep, eq, flipTriangle, t, p))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HoleAngle(
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                >),
                f64,
                1usize,
            >("HoleAngle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HoleAngle", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsEdgeSideOfTriangle(
        triangle: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        ep: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        eq: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                bool,
                3usize,
            >("IsEdgeSideOfTriangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsEdgeSideOfTriangle", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (triangle, ep, eq))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsShallow(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                bool,
                2usize,
            >("IsShallow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsShallow", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (tcx, node)) };
        Ok(__cordl_ret.into())
    }
    pub fn Legalize(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                ),
                bool,
                2usize,
            >("Legalize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Legalize", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (tcx, t)) };
        Ok(__cordl_ret.into())
    }
    pub fn NewFrontTriangle(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        point: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                >,
                3usize,
            >("NewFrontTriangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NewFrontTriangle", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        > = unsafe { method.invoke_unchecked((), (tcx, point, node)) };
        Ok(__cordl_ret.into())
    }
    pub fn NextFlipPoint(
        ep: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        eq: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        ot: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        op: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                >,
                4usize,
            >("NextFlipPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NextFlipPoint", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        > = unsafe { method.invoke_unchecked((), (ep, eq, ot, op)) };
        Ok(__cordl_ret.into())
    }
    pub fn NextFlipTriangle(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        o: crate::UnityEngine::ProBuilder::Poly2Tri::Orientation,
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        ot: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        op: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    crate::UnityEngine::ProBuilder::Poly2Tri::Orientation,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                >,
                6usize,
            >("NextFlipTriangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NextFlipTriangle", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        > = unsafe { method.invoke_unchecked((), (tcx, o, t, ot, p, op)) };
        Ok(__cordl_ret.into())
    }
    pub fn PointEvent(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        point: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                >,
                2usize,
            >("PointEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointEvent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        > = unsafe { method.invoke_unchecked((), (tcx, point)) };
        Ok(__cordl_ret.into())
    }
    pub fn RotateTrianglePair(
        t: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        ot: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
        >,
        op: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DelaunayTriangle,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("RotateTrianglePair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RotateTrianglePair", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (t, p, ot, op))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SplitEdge(
        ep: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        eq: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
        p: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationPoint,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SplitEdge")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SplitEdge", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ep, eq, p))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sweep(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Sweep")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Sweep", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Triangulate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Triangulate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TurnAdvancingFrontConvex(
        tcx: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
        >,
        b: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
        c: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::DTSweepContext,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::Poly2Tri::AdvancingFrontNode,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("TurnAdvancingFrontConvex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TurnAdvancingFrontConvex", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (tcx, b, c))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Poly2Tri+DTSweep")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::Poly2Tri::DTSweep {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
