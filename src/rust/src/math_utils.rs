pub fn norm_inf<Dim1: nalgebra::Dim, Dim2: nalgebra::Dim>(
    x: nalgebra::OMatrix<f64, Dim1, Dim2>,
) -> f64
where
    nalgebra::DefaultAllocator: nalgebra::allocator::Allocator<f64, Dim1, Dim2>,
{
    x.abs().max()
}
