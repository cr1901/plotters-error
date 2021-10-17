use plotters::prelude::*;

// Checkpoint: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9a1400f1796cbaeace2d3a6f26dd3360
// AsRangedCoord requirements not satisfied message still present

//
// On the other hand, for the below function, the diagnostic doesn't show anything about
// AsRangedCoord requirements not being satisfied, although it correctly shows _other_ types
// affected because Ranged isn't satisfied.
//
// Because AsRangedCoord is the first bounds you see when opening the crate docs, it seems
// reasonable that "the inputs don't implement AsRangedCoord" should be shown as part of the
// diagnostic, and why. The Ranged trait doesn't even directly appear in build_cartesian_2d's
// signature.
//
// Cartesian2d:
// Nothing wrong here, but I would expect the inputs X and Y to also have a similar
// "the trait bound `(): Ranged` is not satisfied" message.
//
// CoordTranslate:
//6  | |         .build_cartesian_2d((), 0u32..500u32)?;
//   | |______________________________________________^ the trait `CoordTranslate` is not implemented for `Cartesian2d<(), RangedCoordu32>`
//   |
// = help: the following implementations were found:
// <Cartesian2d<X, Y> as CoordTranslate>
// The help feels like it contradicts the error, and it was one reason I was looking for hints
// with AsRangedCoord rather than the reported errors.
// Should the message be "<Cartesian2d<X: Ranged, Y: Ranged> as CoordTranslate>", or should it say
// "Cartesian2d's bounds are not satisfied, therefore transitively neither are CoordTranslate's".
//
// Perhaps caret over the ")" and caret over "?" can be more easily distinguished with a message
// if `rustc` insists on showing the same errors on both the result type and unwrapped variant?
//
// e.g.           .build_cartesian_2d((), 0u32..500u32)?;
//   | |______________________________________________^ the trait `CoordTranslate` is not implemented for `Cartesian2d<(), RangedCoordu32>`
//
// vs
//               .build_cartesian_2d((), 0u32..500u32)?;
//   | |______________________________________________^ the trait `CoordTranslate` is not implemented for `Cartesian2d<(), RangedCoordu32>` in
//                                                      *the unwrapped type either* (emph mine).

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("", (1024, 768)).into_drawing_area();
    let mut chart = ChartBuilder::on(&root).build_cartesian_2d((), 0u32..500u32)?;

    // This returns the desired "the trait bound `(): Ranged` is not satisfied" for AsRangedCoord message.
    // consume_ranged_coord(());

    Ok(())
}

pub fn consume_ranged_coord<X: plotters::coord::ranged1d::AsRangedCoord>(_x_spec: X) {
    unimplemented!()
}
