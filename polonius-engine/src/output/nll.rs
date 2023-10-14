// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use datafrog::{Iteration, Relation, RelationLeaper};
use std::time::Instant;

use crate::facts::FactTypes;
use crate::output::{Context, Output};

pub(super) fn compute<T: FactTypes>(
    ctx: &Context<'_, T>,
    _result: &mut Output<T>,
) -> (
    Relation<(T::Loan, T::Point)>,
    Relation<(T::Origin, T::Origin)>,
) {
    let timer = Instant::now();

    let (potential_errors, potential_subset_errors) = {
        // Static inputs
        let loan_issued_at = Relation::from_iter(
            ctx.loan_issued_at
                .iter()
                .map(|&(origin, loan, _point)| (origin, loan)),
        );

        let loan_invalidated_at = Relation::from_iter(
            ctx.loan_invalidated_at
                .iter()
                .map(|&(loan, point)| (point, loan)),
        );

        let cfg_edge_rev = Relation::from_iter(ctx.cfg_edge.into_iter().map(|&(x, y)| (y, x)));

        // Create a new iteration context, ...
        let mut iteration = Iteration::new();

        // (from, point1), to
        let outlive = iteration.variable::<((T::Origin, T::Point), T::Origin)>("outlive");

        // need a lot of copies
        let may_contain = iteration.variable::<(T::Origin, T::Point)>("may_contain");
        may_contain.extend(
            ctx.origin_live_on_entry
                .iter()
                .map(|&(origin, point)| (origin, point)),
        );
        let may_contain_rev = iteration.variable::<(T::Point, T::Origin)>("may_contain_reverse");

        let potential_errors = iteration.variable::<(T::Loan, T::Point)>("potential_errors");
        let potential_subset_errors =
            iteration.variable::<(T::Origin, T::Origin)>("potential_subset_errors");

        // (from, point1), point2
        let cm = iteration.variable::<((T::Origin, T::Point), T::Point)>("cm");

        // (from, point1), to
        let so = iteration.variable::<((T::Origin, T::Point), T::Origin)>("so");
        so.extend(
            ctx.subset_base
                .into_iter()
                .map(|&(to, from, point)| ((from, point), to)),
        );

        // .. and then start iterating rules!
        while iteration.changed() {
            may_contain.from_map(&outlive, |&((_from, point1), to)| (to, point1));
            may_contain_rev.from_map(&may_contain, |&(x, y)| (y, x));

            cm.from_join(
                &may_contain_rev,
                &cfg_edge_rev,
                |&point2, &from, &point1| ((from, point1), point2),
            );

            so.from_map(&outlive, |&x| x);

            outlive.from_join(&cm, &so, |&(from, _point1), &point2, &to| {
                ((from, point2), to)
            });

            potential_errors.from_leapjoin(
                &may_contain,
                (
                    loan_issued_at.extend_with(|&(origin, _point)| origin),
                    loan_invalidated_at.extend_with(|&(_origin, point)| point),
                ),
                |&(_origin, point), &loan| (loan, point),
            );
        }

        (
            potential_errors.complete(),
            potential_subset_errors.complete(),
        )
    };

    info!(
        "analysis done: {} `potential_errors` tuples, {} `potential_subset_errors` tuples, {:?}",
        potential_errors.len(),
        potential_subset_errors.len(),
        timer.elapsed()
    );

    (potential_errors, potential_subset_errors)
}
