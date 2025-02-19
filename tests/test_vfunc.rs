/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use anyhow::Result;
use dsi_progress_logger::*;
use epserde::prelude::*;
use sux::{bits::BitFieldVec, func::VFunc, prelude::VFuncBuilder, utils::FromIntoIterator};

#[test]
fn test_vfunc() -> Result<()> {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Info)
        .try_init();

    let mut pl = ProgressLogger::default();

    for offline in [false, true] {
        for n in [0, 10, 1000, 100_000, 3_000_000] {
            dbg!(offline, n);
            let func = VFuncBuilder::<_, _, BitFieldVec<_>, [u64; 2], true>::default()
                .log2_buckets(4)
                .offline(offline)
                .try_build(
                    FromIntoIterator::from(0..n),
                    FromIntoIterator::from(0_usize..),
                    &mut pl,
                )?;
            let mut cursor = <AlignedCursor<maligned::A16>>::new();
            func.serialize(&mut cursor).unwrap();
            cursor.set_position(0);
            let func =
                VFunc::<_, _, BitFieldVec<_>, [u64; 2], true>::deserialize_eps(cursor.as_bytes())
                    .unwrap();
            pl.start("Querying...");
            for i in 0..n {
                assert_eq!(i, func.get(&i));
            }
            pl.done_with_count(n);
        }
    }

    for offline in [false, true] {
        for n in [0, 10, 1000, 100_000, 3_000_000] {
            dbg!(offline, n);
            let func = VFuncBuilder::<_, _, Vec<_>, [u64; 2], true>::default()
                .log2_buckets(4)
                .offline(offline)
                .try_build(
                    FromIntoIterator::from(0..n),
                    FromIntoIterator::from(0_usize..),
                    &mut pl,
                )?;
            let mut cursor = <AlignedCursor<maligned::A16>>::new();
            func.serialize(&mut cursor).unwrap();
            cursor.set_position(0);
            let func =
                VFunc::<_, _, Vec<_>, [u64; 2], true>::deserialize_eps(cursor.as_bytes()).unwrap();
            pl.start("Querying...");
            for i in 0..n {
                assert_eq!(i, func.get(&i));
            }
            pl.done_with_count(n);
        }
    }

    Ok(())
}

#[test]
fn test_dup_key() -> Result<()> {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Info)
        .try_init();

    assert!(VFuncBuilder::<usize, usize>::default()
        .try_build(
            FromIntoIterator::from(std::iter::repeat(0).take(10)),
            FromIntoIterator::from(0..),
            &mut ProgressLogger::default(),
        )
        .is_err());

    Ok(())
}
