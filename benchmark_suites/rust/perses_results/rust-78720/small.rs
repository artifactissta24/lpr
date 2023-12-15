fn server ( ) -> impl FilterBase2 {
segment2 (           { }   ) . map2 ( || "" )
}
trait FilterBase2 {
fn map2 < F > ( self ,       F ) -> Map2 < F >                           { }
}
struct Map2 < segment2 > { _func : F }
impl < F > FilterBase2 for        F   { }
fn segment2 < F > ( _fn : F )
     { }
