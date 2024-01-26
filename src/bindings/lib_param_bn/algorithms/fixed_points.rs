use crate::bindings::lib_param_bn::symbolic::asynchronous_graph::AsynchronousGraph;
use crate::bindings::lib_param_bn::symbolic::set_color::ColorSet;
use crate::bindings::lib_param_bn::symbolic::set_colored_vertex::ColoredVertexSet;
use crate::bindings::lib_param_bn::symbolic::set_vertex::VertexSet;
use crate::{global_log_level, AsNative};
use pyo3::prelude::*;

#[pyclass(module = "biodivine_aeon", frozen)]
pub struct FixedPoints {
    _dummy: (),
}

#[pymethods]
impl FixedPoints {
    /// Iteratively compute the colored set of fixed-points in an `AsynchronousGraph` that are the
    /// subset of the `restriction` set.
    #[staticmethod]
    pub fn symbolic(
        py: Python,
        stg: &AsynchronousGraph,
        restriction: &ColoredVertexSet,
    ) -> PyResult<ColoredVertexSet> {
        let result = biodivine_lib_param_bn::fixed_points::FixedPoints::_symbolic(
            stg.as_native(),
            restriction.as_native(),
            global_log_level(py)?,
            &|| py.check_signals(),
        )?;
        Ok(ColoredVertexSet::mk_native(stg.symbolic_context(), result))
    }

    /// Iteratively compute the set of fixed-point vertices in an `AsynchronousGraph`.
    ///
    /// This is equivalent to `FixedPoints.symbolic(graph, set).vertices()`, but can be
    /// significantly faster because the projection is applied on-demand within the algorithm.
    #[staticmethod]
    pub fn symbolic_vertices(
        py: Python,
        stg: &AsynchronousGraph,
        restriction: &ColoredVertexSet,
    ) -> PyResult<VertexSet> {
        let result = biodivine_lib_param_bn::fixed_points::FixedPoints::_symbolic_vertices(
            stg.as_native(),
            restriction.as_native(),
            global_log_level(py)?,
            &|| py.check_signals(),
        )?;
        Ok(VertexSet::mk_native(stg.symbolic_context(), result))
    }

    /// Iteratively compute the set of fixed-point vertices in an `AsynchronousGraph`.
    ///
    /// This is equivalent to `FixedPoints.symbolic(graph, set).colors()`, but can be
    /// significantly faster because the projection is applied on-demand within the algorithm.
    #[staticmethod]
    pub fn symbolic_colors(
        py: Python,
        stg: &AsynchronousGraph,
        restriction: &ColoredVertexSet,
    ) -> PyResult<ColorSet> {
        let result = biodivine_lib_param_bn::fixed_points::FixedPoints::_symbolic_colors(
            stg.as_native(),
            restriction.as_native(),
            global_log_level(py)?,
            &|| py.check_signals(),
        )?;
        Ok(ColorSet::mk_native(stg.symbolic_context(), result))
    }
}
